use config::Config;
use error::VenusResult;
use serde::{Deserialize, Serialize};

pub mod config;
pub mod consts;
pub mod error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venus {
    /// v2ray and venus's self config
    pub config: Config,
}

impl Venus {
    pub fn new() -> VenusResult<Self> {
        let config = Config::new()?;
        Ok(Self { config })
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
