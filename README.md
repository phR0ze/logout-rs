# logout-rs
Simple, elegant linux desktop logout utility

### Quick links
* [Usage](#usage)
  * [Configuration](#configuration)
* [Backlog](#backlog)
* [Changelog](#changelog)

---

# Usage

## Configuration
***logout-rs*** can be configured with the global config at `/etc/logout-rs.toml` or using a user
override config at `~/.config/logout-rs.toml`.

```toml
# General settings
[settings]
opacity = 80
icon_size = 150
font_size = 25
font_color = "#FFFFFF"
theme = "cyber"
# Action buttons to display on the logout screen in the order given
actions = ["lock", "logout", "reboot", "shutdown", "suspend", "hibernate"]

# Action definitions
# Key is a single case insesitive value to be pressed to trigger the action.
[actions]
lock = { key = "l", icon = "lock.svg", hover = "lock-hover.svg", cmd = "xflock4" }
logout = { key = "o", icon = "logout.svg", hover = "logout-hover.svg", cmd = "pkill -SIGTERM -f lxsession" }
reboot = { key = "r", icon = "reboot.svg", hover = "reboot-hover.svg", cmd = "systemctl reboot" }
shutdown = { key = "s", icon = "shutdown.svg", hover = "shutdown-hover.svg", cmd = "systemctl poweroff" }
suspend = { key = "u", icon = "suspend.svg", hover = "suspend-hover.svg", cmd = "systemctl suspend" }
hibernate = { key = "h", icon = "hibernate.svg", hover = "hibernate-hover.svg", cmd = "systemctl hibernate" }
```

---

# Backlog

# Changelog
