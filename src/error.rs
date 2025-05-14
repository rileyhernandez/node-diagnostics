use libra::scale::ScaleError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("This feature is not yet implemented!")]
    NotImplemented,
    #[error("Error from libra crate: {0}")]
    Libra(ScaleError)
}