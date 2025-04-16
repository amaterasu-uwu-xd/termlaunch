use serde::Deserialize;


/// Config struct for the application
/// This struct is used to load the config from the config file
#[derive(Deserialize, Debug)]
pub struct SerializeConfig {
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub border_color: Option<String>,
    pub accent_color: Option<String>,
    pub icon_theme: Option<String>,
    pub terminal: Option<String>,
}

/// Config struct for the application.
/// This struct is used for the rest of the application
#[derive(Debug)]
pub struct Config {
    pub background_color: String,
    pub text_color: String,
    pub border_color: String,
    pub accent_color: String,
    pub icon_theme: String,
    pub terminal: String,
}

// Opem the config file from $HOME/.config/termrun/config.toml or $XDG_CONFIG_HOME/termrun/config.toml
pub fn load_config(path: Option<String>) -> Config {
    let config_path = match path {
        Some(p) => p.to_string(),
        _none => {
            let home = std::env::var("HOME").unwrap();
            let xdg_config_home = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home));
            format!("{}/termlaunch/config.toml", xdg_config_home)
        }
    };

    let mut config_str = String::new();

    // Check if the config file exists
    if std::path::Path::new(&config_path).exists() {
        config_str = std::fs::read_to_string(config_path).unwrap();
    }

    let imported_conf: SerializeConfig= toml::de::from_str(&config_str).unwrap();

    let config = Config {
        background_color: imported_conf.background_color.unwrap_or("#000000".to_string()),
        text_color: imported_conf.text_color.unwrap_or("#FFFFFF".to_string()),
        border_color: imported_conf.border_color.unwrap_or("#FFFFFF".to_string()),
        accent_color: imported_conf.accent_color.unwrap_or("#FF0000".to_string()),
        icon_theme: imported_conf.icon_theme.unwrap_or("hicolor".to_string()),
        terminal: imported_conf.terminal.unwrap_or("kitty".to_string()),
    };
    
    config
}
