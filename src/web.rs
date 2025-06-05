use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use log::debug;
use serde::Deserialize;

use crate::{
    storage::Storage,
    types::{Event, EventTypes},
};

pub async fn write_event(
    State(mut state): State<Storage>,
    Json(payload): Json<Event>,
) -> impl IntoResponse {
    debug!("write_event");
    let event = Event {
        payload: payload.payload,
        timestamp: payload.timestamp,
        event_type: payload.event_type,
    };
    println!("{event:?}");

    state.write_log_to_storage(event);

    StatusCode::OK
}

#[derive(Debug, Deserialize)]
pub struct Params {
    start: Option<u64>,
    end: Option<u64>,
    event_type: Option<EventTypes>,
}

pub async fn read_event(
    State(state): State<Storage>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    println!("read_event");

    println!("start_time: {:?}", params.start);
    println!("end_time: {:?}", params.end);
    println!("event_type: {:?}", params.event_type);

    let res = state.get_logs_in_range(params.start, params.end, params.event_type);
    let t = Json(res);
    debug!("{t:?}");
    (StatusCode::OK, t)
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
