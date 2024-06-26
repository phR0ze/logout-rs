use rivia::prelude::*;
use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;
use toml;

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
        Config {
            settings: Settings {
                opacity: crate::OPACITY,
                icon_size: crate::ICON_SIZE,
                font_size: crate::FONT_SIZE,
            },
        }
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

/// Return the configured font size
pub(crate) fn font_size() -> u32 {
    CONFIG.get().unwrap().settings.font_size
}

// Config structures and defaults
// -------------------------------------------------------------------------------------------------

/// Configuration structure
#[derive(Deserialize)]
struct Config {
    pub(crate) settings: Settings,
    //pub(super) logout: String,
}

/// Settings sub structure
#[derive(Deserialize)]
struct Settings {
    #[serde(default = "default_opacity")]
    pub(crate) opacity: f64,

    #[serde(default = "default_icon_size")]
    pub(crate) icon_size: u32,

    #[serde(default = "default_font_size")]
    pub(crate) font_size: u32,
    //pub(crate) theme: white
    //#pub(crate) buttons = ["lock", "logout", "restart", "shutdown", "suspend", "hibernate"]
}

fn default_opacity() -> f64 {
    crate::OPACITY
}
fn default_icon_size() -> u32 {
    crate::ICON_SIZE
}
fn default_font_size() -> u32 {
    crate::FONT_SIZE
}
