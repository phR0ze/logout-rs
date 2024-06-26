use gtk::gdk;

// Enum for all actions
pub(crate) enum Action {
    Cancel,
    Lock,
    Logout,
    Restart,
    Shutdown,
    Suspend,
    None,
}

impl Action {
    pub(crate) fn from(key: gdk::Key) -> Action {
        match key {
            gdk::Key::Escape => Action::Cancel,
            gdk::Key::l => Action::Lock,
            gdk::Key::o => Action::Logout,
            gdk::Key::r => Action::Restart,
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
            Action::Lock => eprintln!("lock clicked!"),
            Action::Logout => eprintln!("logout clicked!"),
            Action::Restart => eprintln!("restart clicked!"),
            Action::Shutdown => eprintln!("shutdown clicked!"),
            Action::Suspend => eprintln!("suspend clicked!"),
            _ => (),
        }
    }
}
