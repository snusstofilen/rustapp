use axum::{routing::{get, post}, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
    .route("/hello", get(|| async { "Hello, World!" }))
    .route("/post_example", post(post_example_handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn post_example_handler() -> &'static str {
    // Handle the POST request here
    "Received a POST request"
}