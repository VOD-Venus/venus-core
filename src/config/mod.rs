use anyhow::{anyhow, Result};
use error::ConfigResult;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use types::{CoreConfig, VenusConfig};

use crate::consts::VERSION;

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

    /// Re-read config from file
    ///
    /// ## Arguments
    ///
    /// `resource_path`: the store path of `config.json` and `config.toml`
    pub fn init(&mut self, resource_path: &Path) -> ConfigResult<()> {
        let mut core_default = PathBuf::from(resource_path);
        core_default.push("config.json");

        /* detect_and_create(&core_path, core_default)?;
        if !rua_path.exists() {
            self.write_rua()?;
        } */

        self.reload()?;

        /* if self.rua.logging {
            LOGGING.store(true, Ordering::Relaxed);
        } */
        Ok(())
    }

    /// Reload core and rua config from file
    pub fn reload(&mut self) -> ConfigResult<()> {
        /* self.reload_core()?;
        self.reload_rua()?; */
        Ok(())
    }
    /*
    pub fn reload_rua(&mut self) -> ConfigResult<()> {
        let mut config_file = File::open(&self.rua_path)?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;
        let mut rua_config = toml::from_str::<VenusConfig>(&buffer)?;
        rua_config.version = VERSION.into();
        self.venus = rua_config;
        Ok(())
    }

    /// Reload core config file from VConfig
    pub fn reload_core(&mut self) -> ConfigResult<()> {
        let core_file = File::open(&self.core_path)?;
        let core_config: CoreConfig = serde_json::from_reader(core_file)?;
        self.core = Some(core_config);
        Ok(())
    }

    ///  Write core config to config file
    pub fn write_core(&mut self) -> Result<()> {
        let config = self.core.as_ref().ok_or(anyhow!("core config is empty"))?;
        let core_file = OpenOptions::new().write(true).open(&self.core_path)?;
        core_file.set_len(0)?;
        serde_json::to_writer_pretty(&core_file, &config)?;
        Ok(())
    }

    pub fn write_rua(&mut self) -> Result<()> {
        let mut rua_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.rua_path)?;
        let rua_string = toml::to_string(&self.venus)?;
        rua_file.set_len(0)?;
        rua_file.write_all(rua_string.as_bytes())?;
        Ok(())
    } */
}
