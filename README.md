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
# General settings for the logout-rs utility.
[settings]
opacity = 80
icon_size = 150
font_size = 25
font_color = "#FFFFFF"
# Action buttons to display on the logout screen in the order given
actions = ["lock", "logout", "reboot", "shutdown", "suspend", "hibernate"]

# Actions to execute when the buttons or keybindings are triggered.
[actions]
shutdown = "systemctl poweroff"
hibernate = "systemctl hibernate"
logout = "pkill -SIGTERM -f lxsession"
reboot = "systemctl reboot"
shutdown = "systemctl poweroff"
suspend = "systemctl suspend"

# Keybindings to trigger the actions. A single character is expected.
[keys]
hibernate = "h"
lock = "l"
logout = "o"
reboot = "r"
shutdown = "s"
suspend = "u"
```

---

# Backlog

# Changelog
