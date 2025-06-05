use crate::Event;
use crate::storage::Storage;
use axum::{
    // routing::{get, post},
    // http::StatusCode,
    Json, 
    // Router,
    extract::{State, 
        // Path
    },
};
// use serde::{Deserialize, Serialize};

pub async fn write_event(
    State(mut state): State<Storage>, 
    Json(payload): Json<Event>,
) {
    let event = Event {
        payload: payload.payload,
        timestamp: payload.timestamp,
        event_type: payload.event_type
    };
    println!("{event:?}");

    // let mut x = Storage::new();
    // let x = state.write_log(event);
    state.write_log(event);
    // x.write_log(event);
}

pub async fn read_event() {
    
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
