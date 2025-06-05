// use std::sync::Mutex;
// use std::sync::Arc;

use axum::{
    routing::{get, post},
    // http::StatusCode,
    // Json, 
    Router,
};
use event_tracker::{web::{read_event, write_event}, storage::Storage};


#[tokio::main]
async fn main() {
    // let state = Arc::new(Mutex::new(Storage::new()));
    let state = Storage::new();
    let app = Router::new()
        .route("/events", get(write_event))
        .route("/events", post(read_event))
        .with_state(state)
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
