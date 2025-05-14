use std::time::{Duration, Instant};
use libra::scale::ConnectedScale;
use crate::filter::Filter;
use crate::data::Data;
use crate::error::Error;

pub enum TrialType {
    Raw,
    Median,
    Filtered(Filter)
}
impl TrialType {
    pub fn collect_sample(&mut self, scale: &ConnectedScale) -> Result<f64, Error> {
        match self {
            TrialType::Raw => {
                Ok(scale.get_weight().map_err(Error::Libra)?.get())
            }
            TrialType::Filtered(filter) => {
                Ok(filter.apply(scale.get_weight().map_err(Error::Libra)?.get()))
            }
            _ => {
                Err(Error::NotImplemented)
            }
        }
    }
}


pub struct Trial {
    trial_type: TrialType,
    samples: usize,
    sample_period: Duration
}
impl Trial {
    pub fn new(trial_type: TrialType, samples: usize, sample_period: Duration) -> Self {
        Self { trial_type, samples, sample_period }
    }
    pub fn default() -> Self { Self::new(TrialType::Raw, 100, Duration::from_millis(20))} // 50Hz
    pub fn conduct(mut self, scale: &ConnectedScale) -> Result<Data, Error> {
        let mut data = Data::new(self.samples);
        let start = Instant::now();
        for _sample in 0..self.samples {
            data.push(
              Instant::now() - start,
              self.trial_type.collect_sample(scale)?
            );
            
            std::thread::sleep(self.sample_period)
        }
        Ok(data)
    }
}
