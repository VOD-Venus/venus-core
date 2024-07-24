use crate::config::error::ConfigError;

#[derive(thiserror::Error, Debug)]
pub enum VenusError {
    #[error("config error {0}")]
    Config(#[from] ConfigError),

    // from
    #[error("venus io error {0}")]
    IO(#[from] std::io::Error),
    #[error("venus error {0}")]
    Anyhow(#[from] anyhow::Error),
}

pub type VenusResult<T, E = VenusError> = Result<T, E>;
