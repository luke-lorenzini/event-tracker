use std::{collections::HashMap, sync::{Arc, Mutex}};
use crate::Event;

#[derive(Clone, Default)]
pub struct Storage {
    inner: Arc<Mutex<HashMap<String, Event>>>,
}

impl Storage {
    pub fn new() -> Self {
        let inner = Arc::new(Mutex::new(HashMap::new()));

        Storage { inner }
    }

    fn _get_log_range(_start_range: u64, _end_range: u64, _event_type: String) {

    }

    pub fn write_log(&mut self, event: Event) {
        let _storage = self.inner.clone().lock().unwrap().insert("".into(), event).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let storage = Storage::new();
        assert!(storage.inner.clone().lock().unwrap().is_empty())
    }

    #[test]
    fn test_write_log() {
        let event = Event {
            payload: "".into(),
            timestamp: u64::default(),
            event_type: "".into()
        };
        let storage = Storage::new();
        let _inner = storage.inner.clone().lock().unwrap().insert("".into(), event);
    }

    #[test]
    fn test_get_log_range() {

    }
}
