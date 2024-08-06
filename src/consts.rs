use std::{borrow::Cow, env, sync::LazyLock};

use log::warn;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Default venus config location
pub const DEFAULT_VENUS_CONFIG_PATH: &str = "./config.toml";
/// Read venus config localtion from environment varable `VENUS_CONFIG_PATH`
pub static VENUS_CONFIG_PATH: LazyLock<Cow<'static, str>> = LazyLock::new(|| {
    env::var("VENUS_CONFIG")
            .map_err(|err| { warn!("VENUS_CONFIG env not specified: {err}. using default location {DEFAULT_VENUS_CONFIG_PATH}"); })
            .unwrap_or(DEFAULT_VENUS_CONFIG_PATH.into()).into()
});

/// Default v2ray config location
pub const DEFAULT_V2RAY_CONFIG_PATH: &str = "./config.json";
/// Read v2ray config localtion from environment varable `V2RAY_CONFIG_PATH`
pub static V2RAY_CONFIG_PATH: LazyLock<Cow<'static, str>> = LazyLock::new(|| {
    env::var("VENUS_V2RAY_CONFIG")
            .map_err(|err| { warn!("VENUS_V2RAY_CONFIG env not specified: {err}. using default localtion {DEFAULT_V2RAY_CONFIG_PATH}"); })
            .unwrap_or(DEFAULT_V2RAY_CONFIG_PATH.into()).into()
});

/// Default v2ray executable location
pub const DEFAULT_VENUS_V2RAY_PATH: &str = "./v2ray-core/v2ray";
/// v2ray core executable binary path
pub static VENUS_V2RAY_PATH: LazyLock<Cow<'static, str>> = LazyLock::new(|| {
    env::var("VENUS_V2RAY_PATH")
            .map_err(|err| { warn!("VENUS_V2RAY_PATH env not specified: {err}. using default localtion {DEFAULT_VENUS_V2RAY_PATH}"); })
            .unwrap_or(DEFAULT_VENUS_V2RAY_PATH.into()).into()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_venus_config_path() {
        env::set_var("VENUS_CONFIG", "./config/config.toml");
        let venus_path = &*VENUS_CONFIG_PATH;
        assert_eq!("./config/config.toml", venus_path);
    }

    #[test]
    fn test_get_v2ray_config_path() {
        env::set_var("VENUS_V2RAY_CONFIG", "./config/config.json");
        let venus_path = &*VENUS_CONFIG_PATH;
        assert_eq!("./config/config.json", venus_path);
    }
}
