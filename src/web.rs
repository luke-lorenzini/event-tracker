use axum::{
    Json,
    extract::{Query, State},
};
use log::debug;
use serde::Deserialize;

use crate::{Event, EventTypes, storage::Storage};

pub async fn write_event(State(mut state): State<Storage>, Json(payload): Json<Event>) {
    debug!("write_event");
    let event = Event {
        payload: payload.payload,
        timestamp: payload.timestamp,
        event_type: payload.event_type,
    };
    println!("{event:?}");

    state.write_log(event);
}

#[derive(Debug, Deserialize)]
pub struct Params {
    start: u64,
    end: u64,
    event_type: Option<EventTypes>,
}

pub async fn read_event(State(state): State<Storage>, Query(params): Query<Params>) {
    println!("read_event");

    println!("start_time: {:?}", params.start);
    println!("end_time: {:?}", params.end);
    println!("event_type: {:?}", params.event_type);

    state.get_log_range(params.start, params.end, params.event_type);
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[tokio::test]
//     async fn test_write_event() {
//         let payload = axum::Json::from("value");
//         // let res = write_event(payload).await;
//     }
// }
