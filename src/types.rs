use std::time::SystemTimeError;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

/// Allowable events for logging and reading.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LogType {
    Xyz,
    Xxx,
    Yyz,
    Zyx,
}

/// The struct for the event to be logged.
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    /// Specific log type. From enum.
    pub log_type: LogType,
    /// Unix timestamp of the log.
    pub timestamp: u64,
    /// Generic payload.
    pub payload: Value,
}

/// A custom return type for functions which are fallible.
pub type TrackerResult<T> = Result<T, ErrorTypes>;

/// A listing of custom error types.
#[derive(Debug, Error)]
pub enum ErrorTypes {
    #[error("Invalid Range")]
    InvalidRange(String),
    #[error("Time Anomaly")]
    TimeAnomaly(SystemTimeError),
    #[error("Empty Log File")]
    EmptyLogFile,
}

impl From<SystemTimeError> for ErrorTypes {
    fn from(value: SystemTimeError) -> Self {
        Self::TimeAnomaly(value)
    }
}
