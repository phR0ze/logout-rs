use serde::Deserialize;
use std::fmt;

#[derive(PartialEq, Deserialize, Debug)]
pub(super) struct Settings {
    #[serde(default = "default_opacity")]
    pub(super) opacity: f64,

    #[serde(default = "default_icon_size")]
    pub(super) icon_size: i32,

    #[serde(default = "default_font_size")]
    pub(super) font_size: u32,

    #[serde(default = "default_font_color")]
    pub(super) font_color: String,

    #[serde(default = "default_theme")]
    pub(super) theme: String,
}

fn default_opacity() -> f64 {
    crate::OPACITY
}
fn default_icon_size() -> i32 {
    crate::ICON_SIZE
}
fn default_font_color() -> String {
    crate::FONT_COLOR.into()
}
fn default_font_size() -> u32 {
    crate::FONT_SIZE
}

fn default_theme() -> String {
    crate::THEME.into()
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            opacity: crate::OPACITY,
            icon_size: crate::ICON_SIZE,
            font_color: crate::FONT_COLOR.into(),
            font_size: crate::FONT_SIZE,
            theme: crate::THEME.into(),
        }
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Settings:\n")?;
        f.write_fmt(format_args!("  Opacity: {}\n", self.opacity))?;
        f.write_fmt(format_args!("  Icon Size: {}\n", self.icon_size))?;
        f.write_fmt(format_args!("  Font Size: {}\n", self.font_size))?;
        f.write_fmt(format_args!("  Font Color: {}\n", self.font_color))?;
        f.write_fmt(format_args!("  Theme: {}\n", self.theme))
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize)]
    struct Config {
        settings: Settings,
    }

    #[test]
    fn test_defaults() {
        let settings = toml::from_str::<Config>("[settings]").unwrap().settings;
        assert_eq!(settings, Settings::default());
    }
}
