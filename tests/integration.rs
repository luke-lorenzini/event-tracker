use std::{thread, time::Duration};

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use event_tracker::{app, get_current_time_in_ms};
use http_body_util::BodyExt;
use serde_json::json;
use tower::ServiceExt;

// Summary of this test. Write three events to the log of 2 different types. Query the log to receive just one type back. Should see two records returned.
#[tokio::test]
async fn test_request_specific_events() {
    const EXPECTED_VECTOR_SIZE: usize = 2;
    let app = app();

    let event1 = json!({
        "payload": "event 1",
        "timestamp": get_current_time_in_ms().unwrap(),
        "log_type": "yyz"
    });

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/events")
                .header("Content-Type", "application/json")
                .body(Body::from(event1.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, res.status());
    thread::sleep(Duration::from_millis(1));

    let event2 = json!({
        "payload": "event 2",
        "timestamp": get_current_time_in_ms().unwrap(),
        "log_type": "yyz"
    });

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/events")
                .header("Content-Type", "application/json")
                .body(Body::from(event2.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, res.status());
    thread::sleep(Duration::from_millis(1));

    let event3 = json!({
        "payload": "event 3",
        "timestamp": get_current_time_in_ms().unwrap(),
        "log_type": "xxx"
    });

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/events?log_type=abs")
                .header("Content-Type", "application/json")
                .body(Body::from(event3.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, res.status());
    thread::sleep(Duration::from_millis(1));

    let req = Request::builder()
        .method("GET")
        .uri("/events?log_type=yyz")
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(StatusCode::OK, res.status());

    let collected = res.into_body().collect().await.unwrap();
    let parsed: Result<Vec<(u64, [String; EXPECTED_VECTOR_SIZE])>, _> =
        serde_json::from_slice(&collected.to_bytes());
    println!("{parsed:?}");
    assert!(parsed.is_ok());
}

// Log an event than request a log type that hasn't been logged. Expect a bad result.
#[tokio::test]
async fn test_request_events_that_dont_exist() {
    let app = app();

    let event1 = json!({
        "payload": "event 1",
        "timestamp": get_current_time_in_ms().unwrap(),
        "log_type": "yyz"
    });

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/events")
                .header("Content-Type", "application/json")
                .body(Body::from(event1.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::OK, res.status());
    thread::sleep(Duration::from_millis(1));

    let req = Request::builder()
        .method("GET")
        .uri("/events?log_type=zzz")
        .body(Body::empty())
        .unwrap();

    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(StatusCode::BAD_REQUEST, res.status());
}

// Attempt to write a log of a type which doesn't exist.
#[tokio::test]
async fn test_log_that_doesnt_exist() {
    let app = app();

    let event1 = json!({
        "payload": "event 1",
        "timestamp": get_current_time_in_ms().unwrap(),
        "log_type": "abc"
    });

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/events")
                .header("Content-Type", "application/json")
                .body(Body::from(event1.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, res.status());
}
