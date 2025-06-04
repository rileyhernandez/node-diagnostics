use std::array;
use std::time::{Duration, Instant};
use libra::scale::ConnectedScale;
use crate::filter::Filter;
use crate::data::Data;
use crate::error::Error;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum WeightTrialType {
    Raw,
    Median,
    Filtered(Filter)
}
impl WeightTrialType {
    pub fn collect_sample(&mut self, scale: &ConnectedScale) -> Result<f64, Error> {
        match self {
            WeightTrialType::Raw => {
                Ok(scale.get_weight().map_err(Error::Libra)?.get())
            }
            WeightTrialType::Filtered(filter) => {
                Ok(filter.apply(scale.get_weight().map_err(Error::Libra)?.get()))
            }
            _ => {
                Err(Error::NotImplemented)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeightTrial {
    trial_type: WeightTrialType,
    samples: usize,
    sample_period: Duration
}
impl WeightTrial {
    pub fn new(trial_type: WeightTrialType, samples: usize, sample_period: Duration) -> Self {
        Self { trial_type, samples, sample_period }
    }
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
impl Default for WeightTrial {
    fn default() -> Self { Self::new(WeightTrialType::Raw, 100, Duration::from_millis(80)) }
}

#[derive(Serialize, Deserialize)]
pub struct LoadCellTrial {
    samples: usize,
    sample_period: Duration
}
impl LoadCellTrial {
    pub fn new(samples: usize, sample_period: Duration) -> Self {
        Self { samples, sample_period }
    }
    pub fn conduct(self, scale: &ConnectedScale) -> Result<[Data; 4], Error> {
        let mut data_arr = array::from_fn(|_| Data::new(self.samples));
        let start = Instant::now();
        for _sample in 0..self.samples {
            let readings = scale.get_raw_readings().map_err(Error::Libra)?;
            let time = Instant::now() - start;
            data_arr.iter_mut().enumerate().for_each(|(lc, data)| { data.push(time, readings[lc]) });
            std::thread::sleep(self.sample_period)
        }
        Ok(data_arr)
    }
}