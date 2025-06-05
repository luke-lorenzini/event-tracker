use std::{
    collections::BTreeMap,
    ops::Bound::Included,
    sync::Arc,
};

use log::debug;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::{types::{Event, EventTypes} ,get_current_time_in_ms};

type Log = (u64, (EventTypes, Value));

#[derive(Clone, Default)]
pub struct Storage {
    inner: Arc<Mutex<BTreeMap<u64, (EventTypes, Value)>>>,
}

impl Storage {
    #[must_use]
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

    #[tokio::test]
    async fn test_write_then_read_log() {
        let mut storage = Storage::new();
        let event_type = EventTypes::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms(),
            event_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await;

        let res = storage.get_logs_in_range(None, None, None).await;
        assert_eq!(1, res.len())
    }

    #[tokio::test]
    async fn test_write_then_read_specifc_event() {
        let mut storage = Storage::new();
        let event_type = EventTypes::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms(),
            event_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await;

        let res = storage.get_logs_in_range(None, None, Some(event_type)).await;
        assert_eq!(1, res.len())
    }

    #[tokio::test]
    #[should_panic]
    async fn test_write_historical_event() {
        let mut storage = Storage::new();
        let event_type = EventTypes::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: 0,
            event_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await;

        storage.get_logs_in_range(None, None, None).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_invalid_time_range() {
        let storage = Storage::new();
        storage.get_logs_in_range(Some(10), Some(0), None).await;
    }

    #[tokio::test]
    async fn test_valid_time_range() {
        let mut storage = Storage::new();
        let event_type = EventTypes::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms(),
            event_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await;
        storage.get_logs_in_range(Some(0), Some(10), None).await;
    }

    #[tokio::test]
    #[should_panic]
    async fn test_get_before_write() {
        let storage = Storage::new();
        storage.get_logs_in_range(None, None, None).await;
    }

    #[test]
    fn test_get_log_range() {}
}
