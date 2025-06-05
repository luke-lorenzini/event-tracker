use std::time::SystemTimeError;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    Xyz,
    Xxx,
    Yyz,
    Zyx,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub timestamp: u64,
    pub payload: Value,
}

pub type TrackerResult<T> = Result<T, ErrorTypes>;

#[derive(Debug, Error)]
pub enum ErrorTypes {
    #[error("Invalid Range")]
    InvalidRange(String),
    #[error("Time Anomoly")]
    TimeAnomoly(SystemTimeError),
    #[error("Empty Log File")]
    EmptyLogFile,
}

impl From<SystemTimeError> for ErrorTypes {
    fn from(value: SystemTimeError) -> Self {
        Self::TimeAnomoly(value)
    }
}
