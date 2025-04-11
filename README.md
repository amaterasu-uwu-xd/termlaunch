# Termrun

This is a very WIP project that allows you to launch your desktop applications through a terminal emulator. It is designed with the worst developer practices.

https://raw.githubusercontent.com/amaterasu-uwu-xd/termrun/refs/heads/master/media/termrun-showcase.mp4

## Features
- Launch applications or some of its actions
- Icon preview support
- Customizable (WIP)
- Search (WIP)
  
## Limitations
- The search is not working yet
- Some arguments for FreeDesktop applications are not supported yet
- The actions are not working yet, only the default action is supported
- Some desktop entries could break the application
- If you have not configured a terminal emulator, it will use `kitty` as default

## Installation

### Using cargo
You can install it using cargo, the Rust package manager. You can install it with the following command:

```bash
cargo install --git <repository-url>
```

Make sure you have `$HOME/.cargo/bin` in your `$PATH`. 


### Cloning the repository
You can clone the repository using the following command:    
```bash
git clone <repository-url>
cd /path/to/your/clone
cargo build --release
```

This will create a binary in `target/release/termrun`. You can copy it to your `$PATH` or run it directly from the build folder.

## Integration with WM
The integration is very simple, you just need to add a keybinding to your window manager. Optionally, you can add some window rules.

This is an example for Hyprland:

```hyprlang
# This bind uses $terminal as the terminal emulator, if you want to use another, you can replace it with your preferred terminal.
bind = $mainMod, R, exec $terminal --class termrun -e /path/to/termrun

# Window rules
windowrule = float, class:(termrun)
windowrule = size 100x600, class:(termrun)
windowrule = stayfocused, class:(termrun)
```

## Configuration
The configuration file is located in `$XDG_CONFIG_HOME/termrun/config.toml` or `$HOME/.config/termrun/config.toml`. You can also use the `--config` flag to specify a different configuration file.

### Example configuration
```toml
# Actually only the terminal emulator and the icon theme are supported
terminal = "kitty"
icon_theme = "Papirus"
```
