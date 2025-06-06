use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    Router,
    routing::{get, post},
};
use log::debug;

use crate::{
    storage::Storage,
    types::TrackerResult,
    web::{health_check, read_events, root, write_event},
};

/// An abstraction over a container to store the logs in.
pub mod storage;
/// General types shared throughout the project.
pub mod types;
/// The API endpoints.
pub mod web;

/// Return the current time in ms.
pub fn get_current_time_in_ms() -> TrackerResult<u64> {
    let since_the_epoch =
        u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()).unwrap();
    debug!("{since_the_epoch:?}");
    Ok(since_the_epoch)
}

/// Create the Axum router endpoints (with state).
pub fn app() -> Router {
    let state = Storage::new();
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/events", post(write_event))
        .route("/events", get(read_events))
        .with_state(state)
}
