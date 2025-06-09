pub mod trial;
pub mod filter;
pub mod data;
pub mod error;
pub mod dispenser;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use libra::scale::ConnectedScale;
    use crate::error::Error;
    use crate::filter::Filter;
    use crate::trial::{LoadCellTrial, WeightTrial, WeightTrialType};
    use control_components::controllers::clear_core::Controller;

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
        let data = WeightTrial::default().conduct(&scale)?;
        let times = data.times;
        println!("{:?}", times);
        Ok(())
    }
    
    #[test]
    fn filter() -> Result<(), Error> {
        let scale = make_scale()?;
        let data = WeightTrial::new(
            WeightTrialType::Filtered(
                Filter::default()
            ),
            100,
            Duration::from_millis(20)
        ).conduct(&scale)?;
        println!("{:?}", data);
        Ok(())
    }
    #[test]
    fn load_cell_trial() -> Result<(), Error> {
        let scale = make_scale()?;
        let trial = LoadCellTrial::new(20, Duration::from_millis(80));
        let data = trial.conduct(&scale)?;
        println!("{:?}", data);
        Ok(())
    }
}
/*
self.controller.get_or_insert(ControllerHandle::new(
            "192.168.1.12:8888",
            array::from_fn(|_| MotorBuilder { id: 0, scale: 800 }),
        ))
        
pub fn get_motor(&mut self, id: usize) -> ClearCoreMotor {
        if self.clear_core.is_none() {
            let (controller, controller_client) = clear_core::Controller::with_client(
                "192.168.1.2:8888",
                &[
                    clear_core::MotorBuilder {
                        id: 0,
                        scale: 800,
                    },
                    clear_core::MotorBuilder {
                        id: 1,
                        scale: 800,
                    },
                ],
            );
            tauri::async_runtime::spawn(async move {
                if let Err(_) = controller_client.await {
                    println!("No motor/io controller connected...");
                }
            });
            thread::sleep(Duration::from_secs(5));
            controller.get_motor(id)
        } else {
            self.clear_core.clone().unwrap().get_motor(id)
        }
    }
 */