use axum::{
    Router,
    routing::{get, post},
};
use event_tracker::{
    storage::Storage,
    web::{read_event, write_event},
};

#[tokio::main]
async fn main() {
    let state = Storage::new();
    let app = Router::new()
        .route("/events", post(write_event))
        .route("/events", get(read_event))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
