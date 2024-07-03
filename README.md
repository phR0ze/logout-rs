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
opacity = 0.8
icon_size = 200
font_size = 40
font_color = "#FFFFFF"

# Specifies the theme directory to be used for icon images
theme = "default"

# Action definitions
# - Ordered list of actions to be displaed from left to right in the UI.
# - 'name' is the name of the action to be displayed in the UI.
# - 'key' is a single case insensitive value to be pressed to trigger the action.
# - 'icon' is the icon filename to use from the themes/<theme> directory.
# - 'hover' is the icon filename to use from the themes/<theme> directory for the hover effect.
# - 'cmd' is the command to be executed when the action is triggered.
[[actions]]
name = "Lock"
key = "l"
icon = "lock.svg"
hover = "lock-hover.svg"
cmd = "xflock4"

[[actions]]
name = "Logout"
key = "o"
icon = "logout.svg"
hover = "logout-hover.svg"
cmd = "pkill -SIGTERM -f lxsession"

[[actions]]
name = "Reboot"
key = "r"
icon = "reboot.svg"
hover = "reboot-hover.svg"
cmd = "systemctl reboot"

[[actions]]
name = "Shutdown"
key = "s"
icon = "shutdown.svg"
hover = "shutdown-hover.svg"
cmd = "systemctl poweroff"

[[actions]]
name = "Suspend"
key = "u"
icon = "suspend.svg"
hover = "suspend-hover.svg"
cmd = "systemctl suspend"

[[actions]]
name = "Hibernate"
key = "h"
icon = "hibernate.svg"
hover = "hibernate-hover.svg"
cmd = "systemctl hibernate"
```

---

# Backlog

# Changelog
