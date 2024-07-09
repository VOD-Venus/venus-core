use crate::config::error::ConfigError;

#[derive(thiserror::Error, Debug)]
pub enum VenusError {
    #[error("config error {0}")]
    Config(#[from] ConfigError),
}

pub type VenusResult<T, E = VenusError> = Result<T, E>;
