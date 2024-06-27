use gtk::gdk;
use serde::Deserialize;
use std::fmt;

// Enum for all actions
#[derive(PartialEq, Deserialize, Clone, Copy)]
pub(crate) enum Action {
    Cancel,

    #[serde(alias = "hibernate")]
    Hibernate,

    #[serde(alias = "lock")]
    Lock,

    #[serde(alias = "logout")]
    Logout,

    #[serde(alias = "reboot")]
    Reboot,

    #[serde(alias = "shutdown")]
    Shutdown,

    #[serde(alias = "suspend")]
    Suspend,

    None,
}

impl Action {
    pub(crate) fn from(key: gdk::Key) -> Action {
        match key {
            gdk::Key::Escape => Action::Cancel,
            gdk::Key::h => Action::Hibernate,
            gdk::Key::l => Action::Lock,
            gdk::Key::o => Action::Logout,
            gdk::Key::r => Action::Reboot,
            gdk::Key::s => Action::Shutdown,
            gdk::Key::u => Action::Suspend,
            _ => Action::None,
        }
    }

    pub(crate) fn run(&self) {
        match self {
            Action::Cancel => {
                std::process::exit(0);
            }
            _ => eprintln!("{} clicked!", self),
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::None
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Cancel => write!(f, "Cancel"),
            Action::Hibernate => write!(f, "Hibernate"),
            Action::Lock => write!(f, "Lock"),
            Action::Logout => write!(f, "Logout"),
            Action::Reboot => write!(f, "Reboot"),
            Action::Shutdown => write!(f, "Shutdown"),
            Action::Suspend => write!(f, "Suspend"),
            Action::None => write!(f, "None"),
        }
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_toml_list() {
        #[derive(Deserialize)]
        struct ConfigList {
            actions: Vec<Action>,
        }

        assert_eq!(
            toml::from_str::<ConfigList>("actions = ['Lock', 'Shutdown']")
                .unwrap()
                .actions,
            vec![Action::Lock, Action::Shutdown]
        );
    }

    #[test]
    fn test_from_toml_single() {
        #[derive(Deserialize)]
        struct Config {
            action: Action,
        }

        // Lower case
        assert_eq!(
            toml::from_str::<Config>("action = 'lock'").unwrap().action,
            Action::Lock
        );

        // Pascal case
        assert_eq!(
            toml::from_str::<Config>("action = 'Cancel'")
                .unwrap()
                .action,
            Action::Cancel
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Action::default()), "None");
    }

    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", Action::default()), "None");
    }

    #[test]
    fn test_default_action() {
        assert_eq!(Action::default(), Action::None);
    }
}
