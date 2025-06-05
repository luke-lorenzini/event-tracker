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
    types::{Event, EventType},
};

#[derive(Debug, Deserialize)]
pub struct Params {
    start: Option<u64>,
    end: Option<u64>,
    event_type: Option<EventType>,
}

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
    debug!("{event:?}");

    state.write_log_to_storage(event).await;

    StatusCode::OK.into_response()
}

pub async fn read_event(
    State(state): State<Storage>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    debug!("read_event");
    debug!("start_time: {:?}", params.start);
    debug!("end_time: {:?}", params.end);
    debug!("event_type: {:?}", params.event_type);

    let logs = state
        .get_logs_in_range(params.start, params.end, params.event_type)
        .await;
    let json_logs = Json(logs);
    debug!("{json_logs:?}");
    (StatusCode::OK, json_logs)
}

#[cfg(test)]
mod test {
    use super::*;

    use http::Uri;

    use crate::get_current_time_in_ms;

    #[tokio::test]
    async fn test_write_event() {
        let event_type = EventType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms(),
            event_type: event_type.clone(),
        };
        let json = Json(event);
        let storage = Storage::new();
        let state = State(storage);
        let _res = write_event(state, json).await;
        //    assert_eq!(200, res)
    }

    #[tokio::test]
    async fn test_read_event() {
        let event_type = EventType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms(),
            event_type: event_type.clone(),
        };
        let json = Json(event);
        let storage = Storage::new();
        let state = State(storage);
        let _res = write_event(state.clone(), json).await;

        let uri: Uri = "http://localhost:3000/events?".parse().unwrap();
        let query = Query::try_from_uri(&uri).unwrap();
        let _res = read_event(state, query).await;
    }
}
