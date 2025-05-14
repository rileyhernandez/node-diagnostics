use std::time::Duration;

#[derive(Debug)]
pub struct Data {
    // TODO: for testing
    pub times: Vec<Duration>,
    readings: Vec<f64>
}
impl Data {
    pub fn new(samples: usize) -> Self {
        Self { times: Vec::with_capacity(samples), readings: Vec::with_capacity(samples) }
    }
    pub fn push(&mut self, time: Duration, reading: f64) {
        self.times.push(time); self.readings.push(reading)
    }
}