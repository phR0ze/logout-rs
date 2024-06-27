use serde::Deserialize;
use std::fmt;

#[derive(PartialEq, Deserialize)]
pub(super) struct Settings {
    #[serde(default = "default_opacity")]
    pub(crate) opacity: f64,

    #[serde(default = "default_icon_size")]
    pub(crate) icon_size: u32,

    #[serde(default = "default_font_size")]
    pub(crate) font_size: u32,

    #[serde(default = "default_font_color")]
    pub(crate) font_color: String,

    #[serde(default = "default_theme")]
    pub(crate) theme: String,

    #[serde(default = "default_actions")]
    pub(crate) actions: Vec<crate::Action>,
}

fn default_opacity() -> f64 {
    crate::OPACITY
}
fn default_icon_size() -> u32 {
    crate::ICON_SIZE
}
fn default_font_color() -> String {
    crate::FONT_COLOR.into()
}
fn default_font_size() -> u32 {
    crate::FONT_SIZE
}
fn default_actions() -> Vec<crate::Action> {
    vec![
        crate::Action::Logout,
        crate::Action::Reboot,
        crate::Action::Shutdown,
        crate::Action::Suspend,
        crate::Action::Hibernate,
    ]
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
            actions: default_actions(),
            theme: crate::THEME.into(),
        }
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Debug for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Settings").field("", &self.opacity).finish()
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
    fn test_actions_case_insensitive() {
        let data = r#"[settings]
            actions = ['lock', 'shutdown']
        "#;
        let settings = toml::from_str::<Config>(data).unwrap().settings;
        assert_ne!(settings, Settings::default());
        assert_eq!(
            settings.actions,
            vec![crate::Action::Lock, crate::Action::Shutdown]
        );
    }

    #[test]
    fn test_actions() {
        let data = r#"[settings]
            actions = ['Lock', 'Shutdown']
        "#;
        let settings = toml::from_str::<Config>(data).unwrap().settings;
        assert_ne!(settings, Settings::default());
        assert_eq!(
            settings.actions,
            vec![crate::Action::Lock, crate::Action::Shutdown]
        );
    }

    #[test]
    fn test_defaults() {
        let settings = toml::from_str::<Config>("[settings]").unwrap().settings;
        assert_eq!(settings, Settings::default());
        assert_eq!(settings.actions, default_actions());
    }
}
