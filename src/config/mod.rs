use rivia_vfs::prelude::*;
use serde::Deserialize;
use std::sync::OnceLock;
use std::{fmt, fs};
use toml;

mod settings;
use settings::*;

// Config singleton
// -------------------------------------------------------------------------------------------------
static CONFIG: OnceLock<Config> = OnceLock::new();

pub(crate) fn init() {
    // Determin path to read from
    let user_config = user::config_dir().unwrap().mash(crate::CONFIG_NAME);
    let sys_config = user::sys_config_dir().unwrap().mash(crate::CONFIG_NAME);

    // Use the local user's configuration
    let config = if user_config.exists() {
        let cfg = fs::read_to_string(user_config).unwrap();
        toml::from_str::<Config>(&cfg).unwrap()

    // Use the system-wide configuration
    } else if sys_config.exists() {
        let cfg = fs::read_to_string(sys_config).unwrap();
        toml::from_str::<Config>(&cfg).unwrap()

    // Use defaults as no config file was found
    } else {
        Config::default()
    };

    // Set the singleton's value
    CONFIG.get_or_init(|| config);
}

// Config public accessors
// -------------------------------------------------------------------------------------------------

/// Return the configured opacity
pub(crate) fn opacity() -> f64 {
    CONFIG.get().unwrap().settings.opacity
}

/// Return the configured icon size
pub(crate) fn icon_size() -> u32 {
    CONFIG.get().unwrap().settings.icon_size
}

/// Return the configured font color
pub(crate) fn font_color() -> String {
    CONFIG.get().unwrap().settings.font_color.clone()
}

/// Return the configured font size
pub(crate) fn font_size() -> u32 {
    CONFIG.get().unwrap().settings.font_size
}

// Config structures and defaults
// -------------------------------------------------------------------------------------------------

/// Configuration structure
#[derive(PartialEq, Deserialize)]
struct Config {
    settings: Settings,
    //pub(super) logout: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings::default(),
        }
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        init();
        // assert_eq!(Config::default(), *CONFIG.get().unwrap());
        //assert_eq!(super::font_color(), crate::FONT_COLOR);
    }
}
