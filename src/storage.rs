use std::{collections::BTreeMap, ops::Bound::Included, sync::Arc};

use log::debug;
use serde_json::Value;
use tokio::sync::Mutex;

use crate::{
    get_current_time_in_ms,
    types::{ErrorTypes, Event, LogType, TrackerResult},
};

type Log = (u64, (LogType, Value));

/// A wrapper around the container type to store the logs. This can (and is intended to be) swapped out in the future with a more robust type, like a database.
#[derive(Clone, Default)]
pub struct Storage {
    inner: Arc<Mutex<BTreeMap<u64, (LogType, Value)>>>,
}

impl Storage {
    /// Create a new instance to store a set of logs in.
    #[must_use]
    pub fn new() -> Self {
        let inner = Arc::new(Mutex::new(BTreeMap::new()));

        Storage { inner }
    }

    /// From a specified `start_time` and `end_time`, return a vector of the logs in the range. Optionally, can add an additional filter for specific events. `start_time` and `end_time` are optional as well, if omitted range will start from beginning and go until end.
    /// # Errors
    ///
    /// Will return `Err` if `start_time` is greater than `end_time`.
    #[allow(clippy::missing_panics_doc)]
    pub async fn get_logs_in_range(
        &self,
        start_time: Option<u64>,
        end_time: Option<u64>,
        event_type: Option<LogType>,
    ) -> TrackerResult<Vec<Log>> {
        debug!("start_range: {start_time:?} end_range: {end_time:?} event_type: {event_type:?}");
        if let (Some(start_time), Some(end_time)) = (start_time, end_time) {
            if start_time > end_time {
                return Err(ErrorTypes::InvalidRange(
                    "Start time must be earlier than end time".into(),
                ));
            }
        }

        let inner = self.inner.lock().await;

        if inner.is_empty() {
            return Err(ErrorTypes::EmptyLogFile);
        }

        let start_time = start_time.unwrap_or(0);
        let end_time = end_time.unwrap_or(
            *inner
                .last_key_value()
                .expect("Already ensured non-empty tree")
                .0,
        );

        Ok(inner
            .range((Included(start_time), Included(end_time)))
            .filter(|(_, (et, _))| event_type.as_ref().is_none_or(|e| e == et))
            .map(|(k, v)| (*k, v.clone()))
            .collect())
    }

    /// Write a new log event to the logs container. Cannot write future events.
    /// # Errors
    ///
    /// Will return `Err` if the timestamp time exceeds the current system time.
    pub async fn write_log_to_storage(&self, event: Event) -> TrackerResult<()> {
        debug!("event: {event:?}");
        if event.timestamp > get_current_time_in_ms()? {
            return Err(ErrorTypes::InvalidRange("Cannot log future events".into()));
        }

        let mut inner = self.inner.lock().await;
        let _res = inner.insert(event.timestamp, (event.log_type, event.payload));
        debug!("inner: {inner:?}");

        Ok(())
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
        let event_type = LogType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        let storage = Storage::new();
        let mut inner = storage.inner.lock().await;
        inner.insert(event.timestamp, (event.log_type, event.payload));
        let res = inner.get(&event.timestamp).unwrap().clone();
        let expected = event_type;
        assert_eq!(expected, res.0)
    }

    #[tokio::test]
    async fn test_write_multiple_logs() {
        let number_of_logs = 10;
        let storage = Storage::new();
        for _ in 0..number_of_logs {
            let event_type = LogType::Yyz;
            let event = Event {
                payload: "a payload".into(),
                timestamp: get_current_time_in_ms().unwrap(),
                log_type: event_type.clone(),
            };

            thread::sleep(Duration::from_millis(1));

            let mut inner = storage.inner.lock().await;
            inner.insert(event.timestamp, (event.log_type, event.payload));
        }
        let res = storage.inner.clone().lock().await.len();
        assert_eq!(number_of_logs, res)
    }

    #[tokio::test]
    async fn test_write_then_read_log() {
        let storage = Storage::new();
        let event_type = LogType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await.unwrap();

        let res = storage.get_logs_in_range(None, None, None).await.unwrap();
        assert_eq!(1, res.len())
    }

    #[tokio::test]
    async fn test_write_then_read_multiple_events_filtered() {
        let storage = Storage::new();
        let event_type = LogType::Yyz;
        let event = Event {
            payload: "1st payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await.unwrap();
        thread::sleep(Duration::from_millis(1));

        let event_type = LogType::Xxx;
        let event = Event {
            payload: "2nd payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await.unwrap();
        thread::sleep(Duration::from_millis(1));

        let event_type = LogType::Xxx;
        let event = Event {
            payload: "3rd payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await.unwrap();
        thread::sleep(Duration::from_millis(1));

        let res = storage
            .get_logs_in_range(None, None, Some(LogType::Xxx))
            .await
            .unwrap();
        assert_eq!(2, res.len())
    }

    #[tokio::test]
    async fn test_write_then_read_multiple_events() {
        let storage = Storage::new();
        let event_type = LogType::Yyz;
        let event = Event {
            payload: "1st payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await.unwrap();
        thread::sleep(Duration::from_millis(1));

        let event_type = LogType::Xxx;
        let event = Event {
            payload: "2nd payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await.unwrap();
        thread::sleep(Duration::from_millis(1));

        // let event_type = EventType::Xxx;
        let event = Event {
            payload: "3rd payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await.unwrap();
        thread::sleep(Duration::from_millis(1));

        let res = storage.get_logs_in_range(None, None, None).await.unwrap();
        assert_eq!(3, res.len())
    }

    #[tokio::test]
    async fn test_write_future_event() {
        let storage = Storage::new();
        let event_type = LogType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms().unwrap() * 2,
            log_type: event_type.clone(),
        };
        let res = storage.write_log_to_storage(event).await;
        assert!(res.is_err())
    }

    #[tokio::test]
    async fn test_invalid_time_range() {
        let storage = Storage::new();
        let res = storage.get_logs_in_range(Some(10), Some(0), None).await;
        assert!(res.is_err())
    }

    #[tokio::test]
    async fn test_valid_time_range() {
        let storage = Storage::new();
        let event_type = LogType::Yyz;
        let event = Event {
            payload: "a payload".into(),
            timestamp: get_current_time_in_ms().unwrap(),
            log_type: event_type.clone(),
        };
        storage.write_log_to_storage(event).await.unwrap();
        storage
            .get_logs_in_range(Some(0), Some(10), None)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_get_before_write() {
        let storage = Storage::new();
        let res = storage.get_logs_in_range(None, None, None).await;
        assert!(res.is_err())
    }

    #[test]
    fn test_get_log_range() {}
}
