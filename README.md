# Termlaunch

This is a very WIP project that allows you to launch your desktop applications through a terminal emulator. It is designed with the worst developer practices.

https://github.com/user-attachments/assets/8fbf3679-7065-43a9-8809-ce434fd53846


## Limitations
- If you have not configured a terminal emulator, it will try to use `kitty` as default.
- At some point, i would like to add support for prefixing the launch command with some like `sudo`, `prime-run`, `uwsm`, etc by adding a list of prefixes in the config file.

## Installation

### Using precompiled binaries

From the [releases section](https://github.com/amaterasu-uwu-xd/termlaunch/releases), you can download the latest release available.

The binaries and packages are available for `x86_64` and `aarch64` architectures. If you are using a different architecture, you should build it from source or use the cargo installation method.

#### With your package manager
You can install them using your package manager. Download the package for your architecture (`x86_64` or `aarch64` are available) and install it using your package manager:

```bash
# Debian/Ubuntu and derivatives
sudo apt install ./termlaunch_<version>_Linux_<arch>.deb

# Arch
sudo pacman -U ./termlaunch_<version>_Linux_<arch>.pkg.tar.zst

# Fedora - Also should works for other RPM-based distros
sudo dnf install ./termlaunch_<version>_Linux_<arch>.rpm

# Alpine
sudo apk add ./termlaunch_<version>_Linux_<arch>.apk
``` 

#### Manually
Download the `tar.gz` file for your architecture and extract it. You can use the following command:
```bash
tar -xvf termlaunch_<version>_Linux_<arch>.tar.gz
```
Make sure to replace `<version>` and `<arch>` with the version and architecture you downloaded.

Also, make sure to give the binary executable permissions:
```bash
chmod +x /path/to/termlaunch
```

You can move it to `/usr/bin` or `/usr/local/bin` to make it available system-wide:
```bash
sudo mv /path/to/termlaunch /usr/bin/termlaunch
```

Or you can run it directly from the extracted folder:
```bash
/path/to/termlaunch/termlaunch
```

> Note: The binary is statically linked, so it should work without any additional dependencies. If you have issues, please open an issue in the repository.

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


### Building from source
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
bind = Alt, Space, exec $terminal --class termlaunch -e /path/to/termlaunch

# Window rules
windowrule = float, class:(termlaunch)
windowrule = size 50% 50%, class:(termlaunch)
windowrule = stayfocused, class:(termlaunch)
```

And for Niri:

```kdl
window-rule {
    match app-id="termlaunch"
    open-floating true
    default-column-width { proportion 0.5; }
    default-window-height { proportion 0.5; }
}

binds {
    Alt+Space { spawn: "kitty" "--class" "termlaunch" "-e" "/path/to/termlaunch"; }
}
```

## Configuration
The configuration file is located in `$XDG_CONFIG_HOME/termlaunch/config.toml` or `$HOME/.config/termlaunch/config.toml`. You can also use the `--config` flag to specify a different configuration file.

### Example configuration
```toml
icon_theme = "Papirus-Dark"
terminal = "kitty"

# Lot of colors
[appearance]
search_input = "#f9e2af"
search_border = "#cba6f7"
text = "#f2cdcd"
subtext = "#f5e0dc"
help_text = "#ff5555"
selected_app = "#74c7ec"
selected_app_text = "#11111b"
applications_border = "#a6e3a1"
icon_border = "#55eebb"
info_border = "#89dceb"
actions_border = "#89b4fa"
help_border = "#000000"
```
