use serde::Deserialize;
use std::fmt::{self, Write};

// Enum for all actions
#[derive(PartialEq, Deserialize, Clone, Debug)]
pub(crate) struct Action {
    name: String,
    key: String,
    icon: String,
    hover: String,
    cmd: String,
}

impl Action {
    pub(crate) fn new(name: &str, key: &str, icon: &str, hover: &str, cmd: &str) -> Self {
        Action {
            name: name.into(),
            key: key.into(),
            icon: icon.into(),
            hover: hover.into(),
            cmd: cmd.into(),
        }
    }

    /// Return the action name
    pub(crate) fn name(&self) -> String {
        self.name.clone()
    }

    /// Return the action hot key
    pub(crate) fn key(&self) -> String {
        self.key.clone()
    }

    pub(crate) fn run(&self) {
        eprintln!("running: {}", self.name);
    }
}

impl Default for Action {
    fn default() -> Self {
        Action {
            name: "".into(),
            key: "".into(),
            cmd: "".into(),
            icon: "".into(),
            hover: "".into(),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Action:\n")?;
        f.write_fmt(format_args!("  Name: {:?}\n", self.name))?;
        f.write_fmt(format_args!("  Key: {:?}\n", self.key))?;
        f.write_fmt(format_args!("  Cmd: {:?}\n", self.cmd))?;
        f.write_fmt(format_args!("  Icon: {:?}\n", self.icon))?;
        f.write_fmt(format_args!("  Hover: {:?}\n", self.hover))
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_toml() {
        #[derive(Deserialize, PartialEq)]
        struct Config {
            actions: Vec<Action>,
        }

        let config = toml::from_str::<Config>(
            r#"[[actions]]
            name = 'foo1'
            key = '1'
            icon = 'icon.svg'
            hover = 'hov.svg'
            cmd = 'foo1'

            [[actions]]
            name = 'foo2'
            key = '2'
            icon = 'icon.svg'
            hover = 'hov.svg'
            cmd = 'foo2'

        "#,
        )
        .unwrap();

        assert_eq!(
            config.actions,
            vec![
                Action::new("foo1", "1", "icon.svg", "hov.svg", "foo1"),
                Action::new("foo2", "2", "icon.svg", "hov.svg", "foo2")
            ]
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", Action::default()),
            r#"Action { name: "", key: "", icon: "", hover: "", cmd: "" }"#,
        );
    }

    #[test]
    fn test_debug() {
        assert_eq!(
            format!("{:?}", Action::default()),
            r#"Action { name: "", key: "", icon: "", hover: "", cmd: "" }"#
        );
    }

    #[test]
    fn test_default_action() {
        assert_eq!(Action::default().name, "");
    }
}
