use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EventTypes {
    Xyz,
    Xxx,
    Yyz,
    Zyx,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub event_type: EventTypes,
    pub timestamp: u64,
    pub payload: Value,
}

// pub enum ErrorTypes {
//     TimeMismatch,
// }
