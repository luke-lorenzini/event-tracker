use std::time::{SystemTime, UNIX_EPOCH};

use log::debug;

use crate::types::TrackerResult;

pub mod storage;
pub(crate) mod types;
pub mod web;

fn get_current_time_in_ms() -> TrackerResult<u64> {
    let since_the_epoch =
        u64::try_from(SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis()).unwrap();
    debug!("{since_the_epoch:?}");
    Ok(since_the_epoch)
}
