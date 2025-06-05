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

#[allow(clippy::unused_async)]
pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Welcome home")
}

#[allow(clippy::unused_async)]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Healthy")
}

pub async fn write_event(
    State(state): State<Storage>,
    Json(payload): Json<Event>,
) -> Result<impl IntoResponse, StatusCode> {
    debug!("write_event");
    let event = Event {
        payload: payload.payload,
        timestamp: payload.timestamp,
        event_type: payload.event_type,
    };
    debug!("{event:?}");

    match state.write_log_to_storage(event).await {
        Ok(()) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn read_event(
    State(state): State<Storage>,
    Query(params): Query<Params>,
) -> Result<impl IntoResponse, StatusCode> {
    debug!("read_event");
    debug!("start_time: {:?}", params.start);
    debug!("end_time: {:?}", params.end);
    debug!("event_type: {:?}", params.event_type);

    let logs = state
        .get_logs_in_range(params.start, params.end, params.event_type)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let json_logs = Json(logs);
    debug!("{json_logs:?}");
    Ok((StatusCode::OK, json_logs))
}

#[cfg(test)]
mod test {
    use super::*;

    use http::Uri;

    use crate::get_current_time_in_ms;

    #[tokio::test]
    async fn test_root() {
        let res = root().await.into_response();
        assert_eq!(StatusCode::OK, res.status())
    }

    #[tokio::test]
    async fn test_health_check() {
        let res = health_check().await.into_response();
        assert_eq!(StatusCode::OK, res.status())
    }

    #[tokio::test]
    async fn test_write_event() {
        let event_type = EventType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            event_type: event_type.clone(),
        };
        let json = Json(event);
        let storage = Storage::new();
        let state = State(storage);
        let res = write_event(state, json).await.into_response();
        assert_eq!(StatusCode::OK, res.status())
    }

    #[tokio::test]
    async fn test_write_event_bad_params() {
        let event_type = EventType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms().unwrap() * 2,
            event_type: event_type.clone(),
        };
        let json = Json(event);
        let storage = Storage::new();
        let state = State(storage);
        let res = write_event(state, json).await.into_response();
        assert_eq!(StatusCode::BAD_REQUEST, res.status())
    }

    #[tokio::test]
    async fn test_read_event() {
        let event_type = EventType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            event_type: event_type.clone(),
        };
        let json = Json(event);
        let storage = Storage::new();
        let state = State(storage);
        let _res = write_event(state.clone(), json).await;

        let uri: Uri = "http://localhost:3000/events?".parse().unwrap();
        let query = Query::try_from_uri(&uri).unwrap();
        let res = read_event(state, query).await.into_response();
        assert_eq!(StatusCode::OK, res.status())
    }
}
