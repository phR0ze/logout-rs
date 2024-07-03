/// Name of the application
/// ID must have at least two segments separated by a period
pub(crate) const APP_ID: &str = "cyberlinux.logout";
pub(crate) const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const APP_VER: &str = env!("CARGO_PKG_VERSION");

/// Configuration filename and path to use
pub(crate) const CONFIG_DIR: &str = "logout-rs";
pub(crate) const CONFIG_NAME: &str = "config.toml";

/// Spacing between widgets
pub(crate) const SPACING: i32 = 40;

/// Defaults for settigns
pub(crate) const OPACITY: f64 = 0.4;

pub(crate) const ICON_SIZE: i32 = 200;

pub(crate) const FONT_SIZE: u32 = 40;
pub(crate) const FONT_COLOR: &str = "#FFFFFF";

pub(crate) const THEME: &str = "cyber";

/// Default icons
pub(crate) static LOGOUT_SVG: &[u8] = include_bytes!("../assets/themes/default/logout.svg");
