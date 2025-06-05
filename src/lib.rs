use serde::Deserialize;
use serde_json::Value;

pub mod storage;
pub mod web;

// pub type Result<T> = Result<T, ErrorTypes>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventTypes {
    Xyz,
    Xxx,
    Yyz,
    Zyx,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    event_type: EventTypes,
    timestamp: u64,
    payload: Value,
}

pub enum ErrorTypes {
    TimeMismatch,
}

#[cfg(test)]
mod test {}
