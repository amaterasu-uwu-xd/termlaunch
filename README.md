# Termlaunch

This is a very WIP project that allows you to launch your desktop applications through a terminal emulator. It is designed with the worst developer practices.

https://github.com/user-attachments/assets/8fbf3679-7065-43a9-8809-ce434fd53846

## Features
- Launch applications or some of its actions
- Search
- Icon preview support
- Customizable (WIP)
  
## Limitations
- Some desktop entries could break the application
- If you have not configured a terminal emulator, it will use `kitty` as default

## Installation

### Using precompiled binaries

From the [releases section](https://github.com/amaterasu-uwu-xd/termlaunch/releases), you can download the latest release available.

Download the binary for your architecture and place it in your `$PATH`. Make sure to give it execute permissions:

```bash
chmod +x /path/to/termlaunch
```

### Using cargo
You can install it using cargo, the Rust package manager. You can install it with the following command:

```bash
cargo install termlaunch
```

Or you can follow the latest commit in the repository:

```bash
cargo install --git https://github.com/amaterasu-uwu-xd/termlaunch
```

Make sure you have `$HOME/.cargo/bin` in your `$PATH`. 


## Building from source
If you want to build it from source, you can clone the repository and build it using cargo. Make sure you have Rust and Cargo installed. You can install them using [rustup](https://rustup.rs/). Requires `1.85.0` or higher.
```bash
cd ~/Downloads
git clone https://github.com/amaterasu-uwu-xd/termlaunch
cd termlaunch
# Make sure you have Rust and Cargo installed, if not, you can install it using rustup
cargo build --release
```

This will create a binary in `target/release/`. You can copy it to your `$PATH` or run it directly from the build folder.

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
The configuration file is located in `$XDG_CONFIG_HOME/termlaunch/config.toml` or `$HOME/.config/termlaunch/config.toml`. You can also use the `--config` flag to specify a different configuration file.

### Example configuration
```toml
# Actually only the terminal emulator and the icon theme are supported
terminal = "kitty"
icon_theme = "Papirus"
```
