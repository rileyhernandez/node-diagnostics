mod trial;
mod filter;
mod data;
mod error;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use libra::scale::ConnectedScale;
    use crate::error::Error;
    use crate::filter::Filter;
    use crate::trial::{Trial, TrialType};

    fn make_scale() -> Result<ConnectedScale, Error> {
        ConnectedScale::without_id(Duration::from_secs(5)).map_err(Error::Libra)
    }

    #[test]
    fn try_scale() {
        assert!(make_scale().is_ok())
    }
    
    #[test]
    fn raw_weight_trial() -> Result<(), Error> {
        let scale = make_scale()?;
        let data = Trial::default().conduct(&scale)?;
        let times = data.times;
        println!("{:?}", times);
        Ok(())
    }
    
    #[test]
    fn filter() -> Result<(), Error> {
        let scale = make_scale()?;
        let data = Trial::new(
            TrialType::Filtered(
                Filter::default()
            ),
            100,
            Duration::from_millis(20)
        ).conduct(&scale)?;
        println!("{:?}", data);
        Ok(())
    }
}
