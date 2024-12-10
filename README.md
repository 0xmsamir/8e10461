## To run the project
```bash
RUST_LOG=debug cargo run
```

the server listens on 127.0.0.1:3000

## Endpoints
```
GET /movie/:id - retrieve a movie
POST /movie - create a movie
```

### Sample Requests
```bash
$ curl localhost:3000/movie/1
{"success":false,"error":"movie not found"}


$ curl -XPOST localhost:3000/movie -d '{"id": "1", "name": "test movie", "year": 2024, "was_good": true}' -H 'content-type: application/json'
{"success":true,"data":"movie created"}

$ curl localhost:3000/movie/1
{"success":true,"data":{"id":"1","name":"test movie","was_good":true,"year":2024}}
```

if run with `RUST_LOG=debug`, the server prints some debug info:
```
RUST_LOG=debug cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/interview`
[2024-12-10T19:01:14Z DEBUG interview] saving movie 1 to db
[2024-12-10T19:01:30Z DEBUG interview] fetched movie 1 from db
[2024-12-10T19:01:48Z DEBUG interview] fetched movie 1 from cache
[2024-12-10T19:01:52Z DEBUG interview] fetched movie 1 from cache
[2024-12-10T19:01:53Z DEBUG interview] fetched movie 1 from cache
```


