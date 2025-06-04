use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Filter {
    smoothing_factor: f64,
    last_reading: f64
}
impl Filter {
    pub fn new(sample_rate: f64, cutoff_frequency: f64) -> Self {
        let period = 1. / sample_rate;
        let rc = 1. / (cutoff_frequency * 2. * std::f64::consts::PI);
        Self {
            smoothing_factor: period / (rc + period),
            last_reading: 0.,
        }
    }
    // TODO: not sure if i need this?
    // fn update(self, incoming_reading: f64) -> Self {
    //     Self { smoothing_factor: self.smoothing_factor, last_reading: incoming_reading}
    // }
    pub fn apply(&mut self, incoming_reading: f64) -> f64 {
        let last_reading = self.last_reading;
        self.last_reading = incoming_reading;
        self.smoothing_factor * incoming_reading + (1. - self.smoothing_factor) * last_reading
    }
}
impl Default for Filter {
    fn default() -> Self { Self::new(50., 0.5) }
}