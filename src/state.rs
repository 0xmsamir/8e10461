use lru::LruCache;
use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

use crate::Movie;

// State to hold shared resources between requests.
// Implements Clone so that it can be shared between tokio tasks (and possibly threads);
#[derive(Clone)]
pub struct AppState {
    // mutexes are used to provide synchronized access across threads.
    pub db: Arc<Mutex<HashMap<String, Movie>>>,
    pub cache: Arc<Mutex<LruCache<String, Movie>>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            db: Arc::new(Mutex::new(HashMap::new())),
            // Lru cache is used to emulate a caching mechanism. Could also be a HashMap.
            cache: Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(2).unwrap()))),
        }
    }
}
