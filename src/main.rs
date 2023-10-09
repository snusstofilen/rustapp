use axum::{routing::{get, post}, Router, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostData {
    a: String,
    b: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    .route("/hello", get(|| async { "Hello, World!" }))
    .route("/post_example", post(post_example_handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn post_example_handler(
    // `Json` supports any type that implements `serde::Deserialize`
    Json(payload): Json<PostData>,
) -> String {
    // use server_config and payload to run the `auth` logic
    format!("{}, {}", payload.a, payload.b)
}