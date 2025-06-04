use std::time::Duration;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    // TODO: pub for testing
    pub times: Vec<Duration>,
    pub readings: Vec<f64>
}
impl Data {
    pub fn new(samples: usize) -> Self {
        Self { times: Vec::with_capacity(samples), readings: Vec::with_capacity(samples) }
    }
    pub fn push(&mut self, time: Duration, reading: f64) {
        self.times.push(time); self.readings.push(reading)
    }
}