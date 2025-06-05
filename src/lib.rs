use std::time::{SystemTime, UNIX_EPOCH};

pub mod storage;
pub(crate) mod types;
pub mod web;

// todo: fix me
fn get_current_time_in_ms() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1_000
        + u64::from(since_the_epoch.subsec_nanos()) / 1_000_000_000;
    println!("{since_the_epoch:?}");
    println!("{in_ms:?}");
    in_ms
}
