mod response;
mod state;

use axum::extract::{self, State};
use axum::routing::post;
use axum::Json;
use axum::{extract::Path, routing::get, Router};
use log::debug;
use response::Response;
use serde::{Deserialize, Serialize};
use serde_json::json;
use state::AppState;
use tokio::net::TcpListener;

#[derive(Clone, Serialize, Deserialize)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool,
}

async fn get_movie(Path(movie_id): Path<String>, State(state): State<AppState>) -> Json<Response> {
    // check if cached
    let mut cache = state.cache.lock().unwrap();
    if let Some(movie) = cache.get(&movie_id) {
        debug!("fetched movie {} from cache", movie.id);
        return Json(Response::Ok(json!(movie)));
    }

    // if not, try the db and cache the result if exists
    let db = state.db.lock().unwrap();
    match db.get(&movie_id) {
        Some(movie) => {
            debug!("fetched movie {} from db", movie.id);
            cache.put(movie_id, movie.clone());
            Json(Response::Ok(json!(movie)))
        }
        None => Json(Response::Error(json!("movie not found"))),
    }
}

async fn create_movie(
    State(state): State<AppState>,
    extract::Json(movie): extract::Json<Movie>,
) -> Json<Response> {
    // check if already exists
    let mut db = state.db.lock().unwrap();
    if db.get(&movie.id).is_some() {
        return Json(Response::Error(json!("movie already exists")));
    }

    // create movie
    debug!("saving movie {} to db", movie.id);
    db.insert(movie.id.clone(), movie);
    Json(Response::Ok(json!("movie created")))
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let state = AppState::new();
    let app = Router::new()
        .route("/movie/:movie_id", get(get_movie))
        .route("/movie", post(create_movie))
        // app state contains the db and the cache. It could also
        // contain connections or connection pools to external db/cache
        .with_state(state);

    // start a tokio tcp listener
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to address");

    // run the app
    axum::serve(listener, app).await.unwrap();
}
