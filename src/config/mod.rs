use error::{ConfigError, ConfigResult};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use types::{CoreConfig, VenusConfig};

use crate::consts::{get_v2ray_config_path, get_venus_config_path, VERSION};

pub mod error;
pub mod types;

/// All config field
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    /// Core config from `config.json`
    pub core: Option<CoreConfig>,
    pub venus: VenusConfig,
}

/// Core config and global states
impl Config {
    pub fn new() -> ConfigResult<Self> {
        let v_config = VenusConfig::default();

        let config = Self {
            core: None,
            venus: v_config,
        };

        Ok(config)
    }

    pub fn reload_rua(&mut self) -> ConfigResult<()> {
        let path = PathBuf::from(get_venus_config_path().as_ref());
        let mut config_file = File::open(path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;
        let mut rua_config = toml::from_str::<VenusConfig>(&buffer)?;
        rua_config.version = VERSION.into();
        self.venus = rua_config;
        Ok(())
    }

    /// Reload core config file from VConfig
    pub fn reload_core(&mut self) -> ConfigResult<()> {
        let path = PathBuf::from(get_v2ray_config_path().as_ref());
        let core_file = File::open(path)?;
        let core_config: CoreConfig = serde_json::from_reader(core_file)?;
        self.core = Some(core_config);
        Ok(())
    }

    ///  Write core config to config file
    pub fn write_core(&mut self) -> ConfigResult<()> {
        let path = PathBuf::from(get_v2ray_config_path().as_ref());
        let config = self
            .core
            .as_ref()
            .ok_or(ConfigError::Empty("v2ray core config is empty".into()))?;
        let core_file = OpenOptions::new().write(true).open(path)?;
        core_file.set_len(0)?;
        serde_json::to_writer_pretty(&core_file, &config)?;
        Ok(())
    }

    pub fn write_rua(&mut self) -> ConfigResult<()> {
        let path = PathBuf::from(get_venus_config_path().as_ref());
        let mut rua_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        let rua_string = toml::to_string(&self.venus)?;
        rua_file.set_len(0)?;
        rua_file.write_all(rua_string.as_bytes())?;
        Ok(())
    }
}
