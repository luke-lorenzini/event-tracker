use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    Router,
    routing::{get, post},
};
use log::debug;
use tokio::time::sleep;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

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
#[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
pub fn get_current_time_in_ms() -> TrackerResult<u64> {
    let since_the_epoch =
        u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()).unwrap();
    debug!("{since_the_epoch:?}");
    Ok(since_the_epoch)
}

/// Create the Axum router endpoints (with state).
#[allow(clippy::missing_panics_doc)]
pub fn app(rate_limiting: bool) -> Router {
    let state = Storage::new();

    let mut router = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .route("/events", post(write_event))
        .route("/events", get(read_events))
        .with_state(state);

    if rate_limiting {
        let governor_conf = Arc::new(
            GovernorConfigBuilder::default()
                .per_second(2)
                .burst_size(5)
                .finish()
                .expect("Ensured burst_size non-zero"),
        );

        let governor_limiter = governor_conf.limiter().clone();
        let interval = Duration::from_secs(60);
        tokio::spawn(async move {
            loop {
                sleep(interval).await;
                governor_limiter.retain_recent();
            }
        });

        router = router.layer(GovernorLayer {
            config: governor_conf,
        });
    }

    router
}
