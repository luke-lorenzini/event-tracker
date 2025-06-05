use std::{
    collections::BTreeMap,
    ops::Bound::Included,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use log::debug;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::types::{Event, EventTypes};

type Log = (u64, (EventTypes, Value));

#[derive(Clone, Default)]
pub struct Storage {
    inner: Arc<Mutex<BTreeMap<u64, (EventTypes, Value)>>>,
}

impl Storage {
    pub fn new() -> Self {
        let inner = Arc::new(Mutex::new(BTreeMap::new()));

        Storage { inner }
    }

    pub async fn get_logs_in_range(
        &self,
        start_time: Option<u64>,
        end_time: Option<u64>,
        event_type: Option<EventTypes>,
    ) -> Vec<Log> {
        debug!("start_range: {start_time:?} end_range: {end_time:?} event_type: {event_type:?}");
        if let (Some(start_time), Some(end_time)) = (start_time, end_time) {
            if start_time > end_time {
                todo!("Range invalid")
            }
        }

        let inner = self.inner.lock().await;

        if inner.is_empty() {
            todo!("Nothing to see here")
        }

        let start_time = start_time.unwrap_or(0);
        let end_time = end_time.unwrap_or(*inner.last_key_value().unwrap().0);

        match event_type {
            Some(event_type) => inner
                .range((Included(start_time), Included(end_time)))
                .filter(|(_, (et, _))| et == &event_type.clone())
                .map(|(k, v)| (*k, v.clone()))
                .collect(),
            None => inner
                .range((Included(start_time), Included(end_time)))
                .map(|(k, v)| (*k, v.clone()))
                .collect(),
        }
    }

    pub async fn write_log_to_storage(&mut self, event: Event) {
        debug!("event: {event:?}");
        if event.timestamp < get_current_time_in_ms() {
            todo!("Cannot log historical events")
        }

        let mut inner = self.inner.lock().await;
        let _res = inner.insert(event.timestamp, (event.event_type, event.payload));
        // println!("result of insert: {res:?}");
        println!("inner: {inner:?}");
    }
}

// todo: fix me
fn get_current_time_in_ms() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let in_ms =
        since_the_epoch.as_secs() * 1_000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000_000;
    println!("{since_the_epoch:?}");
    println!("{in_ms:?}");
    in_ms
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;

    #[tokio::test]
    async fn test_new() {
        let storage = Storage::new();
        let inner = storage.inner.lock().await;
        assert!(inner.is_empty())
    }

    #[tokio::test]
    async fn test_write_single_log() {
        let event_type = EventTypes::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms(),
            event_type: event_type.clone(),
        };
        let storage = Storage::new();
        let mut inner = storage.inner.lock().await;
        inner.insert(event.timestamp, (event.event_type, event.payload));
        let res = inner.get(&event.timestamp).unwrap().clone();
        let expected = event_type;
        assert_eq!(expected, res.0)
    }

    #[tokio::test]
    async fn test_write_multiple_logs() {
        let number_of_logs = 10;
        let storage = Storage::new();
        for _ in 0..number_of_logs {
            let event_type = EventTypes::Yyz;
            let event = Event {
                payload: "a payload".into(),
                timestamp: get_current_time_in_ms(),
                event_type: event_type.clone(),
            };

            thread::sleep(Duration::from_millis(1000));

            let mut inner = storage.inner.lock().await;
            inner.insert(event.timestamp, (event.event_type, event.payload));
        }
        let res = storage.inner.clone().lock().await.len();
        assert_eq!(number_of_logs, res)
    }

    #[test]
    fn test_get_log_range() {}
}
