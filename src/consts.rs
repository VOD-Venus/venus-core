use std::{borrow::Cow, env, sync::OnceLock};

use log::error;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Default venus config location
pub const DEFAULT_VENUS_CONFIG_PATH: &str = "./config.toml";
/// Read venus config localtion from environment varable `VENUS_CONFIG_PATH`
pub static VENUS_CONFIG_PATH: OnceLock<Cow<'static, str>> = OnceLock::new();
pub fn get_venus_config_path() -> &'static Cow<'static, str> {
    VENUS_CONFIG_PATH.get_or_init(|| match env::var("VENUS_CONFIG") {
        Ok(var) => var.into(),
        Err(err) => {
            error!("read VENUS_CONFIG_PATH failed {err}");
            DEFAULT_VENUS_CONFIG_PATH.into()
        }
    })
}

/// Default v2ray config location
pub const DEFAULT_V2RAY_CONFIG_PATH: &str = "./config.json";
/// Read v2ray config localtion from environment varable `V2RAY_CONFIG_PATH`
pub static V2RAY_CONFIG_PATH: OnceLock<Cow<'static, str>> = OnceLock::new();
pub fn get_v2ray_config_path() -> &'static Cow<'static, str> {
    V2RAY_CONFIG_PATH.get_or_init(|| match env::var("VENUS_V2RAY_CONFIG") {
        Ok(var) => var.into(),
        Err(err) => {
            error!("read VENUS_CONFIG_PATH failed {err}");
            DEFAULT_V2RAY_CONFIG_PATH.into()
        }
    })
}
