use rivia_vfs::prelude::*;
use serde::Deserialize;
use std::{env, sync::OnceLock};

mod action;
pub(crate) use action::*;
mod settings;
use settings::*;

// Config singleton
// -------------------------------------------------------------------------------------------------
static CONFIG: OnceLock<Config> = OnceLock::new();

/// Read the configuration file and set the singleton for further access
pub(crate) fn init() {
    let config_dir = if cfg!(debug_assertions) {
        // Debug config directory
        Some(
            env::current_exe()
                .expect("to get exe .../target/debug/logout-rs")
                .parent()
                .expect("to get parent path .../target/debug")
                .parent()
                .expect("to get parent path .../target")
                .parent()
                .expect("to get parent path .../")
                .mash("assets"),
        )
    } else {
        // Release config directory
        let filepath = PathBuf::from(crate::CONFIG_DIR)
            .join(crate::CONFIG_NAME)
            .to_string()
            .expect("config path to convert to a string");
        vfs::config_dir(&filepath)
    };

    // Read the config file or default it
    let config = if let Some(dir) = config_dir {
        let data =
            vfs::read_all(dir.mash(crate::CONFIG_NAME)).expect("config to exist and be readable");
        let mut config =
            toml::from_str::<Config>(&data).expect("config to be valid TOML config file");
        config.active_path = Some(dir);
        config
    } else {
        Config::default()
    };

    // Validate the new configuration
    validate(&config);

    // Set the singleton's value
    CONFIG.get_or_init(|| config);
}

// Validate the deserialized configuration is valid
fn validate(config: &Config) {
    if config.settings.opacity < 0.0 || config.settings.opacity > 1.0 {
        eprintln!("Invalid opacity value: {}", config.settings.opacity);
        std::process::exit(1);
    }
}

// Config public accessors
// -------------------------------------------------------------------------------------------------

/// Return the active config path
pub(crate) fn active_path() -> Option<PathBuf> {
    CONFIG.get().unwrap().active_path.clone()
}

/// Return the config system path
pub(crate) fn sys_path() -> PathBuf {
    CONFIG.get().unwrap().sys_path.clone()
}

/// Return the config user path
pub(crate) fn user_path() -> PathBuf {
    CONFIG.get().unwrap().user_path.clone()
}

/// Return the configured opacity
pub(crate) fn opacity() -> f64 {
    CONFIG.get().unwrap().settings.opacity
}

/// Return the configured icon size
pub(crate) fn icon_size() -> i32 {
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

/// Return the configured actions
pub(crate) fn actions() -> Vec<Action> {
    CONFIG.get().unwrap().actions.clone()
}

// Config structures and defaults
// -------------------------------------------------------------------------------------------------

/// Configuration structure
#[derive(PartialEq, Deserialize)]
struct Config {
    #[serde(skip)]
    active_path: Option<PathBuf>,

    #[serde(default = "default_system_path")]
    sys_path: PathBuf,

    #[serde(default = "default_user_path")]
    user_path: PathBuf,

    #[serde(default)]
    settings: Settings,

    #[serde(default = "default_actions")]
    actions: Vec<Action>,
}

// Default actions to use
fn default_actions() -> Vec<Action> {
    default_xfce_actions()
}

/// Default system path
fn default_system_path() -> PathBuf {
    PathBuf::from("/etc/xdg").mash(crate::CONFIG_DIR)
}

/// Default user path
fn default_user_path() -> PathBuf {
    user::config_dir().unwrap().mash(crate::CONFIG_DIR)
}

// Default actions for the XFCE desktop environment
fn default_xfce_actions() -> Vec<Action> {
    vec![
        Action::new(
            "Logout",
            "o",
            "logout.svg",
            "logout-hover.svg",
            "xfce4-session-logout --logout",
        ),
        Action::new(
            "Reboot",
            "r",
            "reboot.svg",
            "reboot-hover.svg",
            "xfce4-session-logout --reboot",
        ),
        Action::new(
            "Shutdown",
            "s",
            "shutdown.svg",
            "shutdown-hover.svg",
            "xfce4-session-logout --halt",
        ),
        Action::new(
            "Suspend",
            "u",
            "suspend.svg",
            "suspend-hover.svg",
            "xfce4-session-logout --suspend",
        ),
        Action::new(
            "Hibernate",
            "h",
            "hibernate.svg",
            "hibernate-hover.svg",
            "xfce4-session-logout --hibernate",
        ),
    ]
}
impl Default for Config {
    fn default() -> Self {
        Config {
            active_path: None,
            sys_path: default_system_path(),
            user_path: default_user_path(),
            settings: Settings::default(),
            actions: default_actions(),
        }
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_system_config() {
    //     // Write system config
    //     assert!(vfs::set_memfs().is_ok());
    //     let file = vfs::root().mash("etc").mash(crate::CONFIG_NAME);

    //     let data = r#"[settings]
    //         actions = ['Lock', 'Shutdown']
    //     "#;
    //     let settings = toml::from_str::<Config>(data).unwrap().settings;
    //     assert_ne!(settings, Settings::default());
    //     assert_eq!(settings.actions, vec![Action::Lock, Action::Shutdown]);
    // }

    #[test]
    fn test_default_config() {
        assert!(vfs::set_memfs().is_ok());
        init();
        assert_eq!(super::opacity(), crate::OPACITY);
    }
}
