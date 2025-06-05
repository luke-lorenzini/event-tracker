use serde::Deserialize;

pub mod storage;
pub mod web;

#[derive(Debug, Deserialize)]
pub struct Event {
    event_type: String,
    timestamp: u64,
    payload: String,
}

#[cfg(test)]
mod test {

}
