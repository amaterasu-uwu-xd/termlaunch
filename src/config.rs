use serde::{ Deserialize, Serialize };

#[derive(Serialize,Deserialize, Debug)]
pub struct Config {
    background_color: Option<String>,
    text_color: Option<String>,
    border_color: Option<String>,
    accent_color: Option<String>,
}

// Opem the config file from $HOME/.config/termrun/config.toml or $XDG_CONFIG_HOME/termrun/config.toml
pub fn load_config(path: Option<String>) -> Result<Config, toml::de::Error> {
    let config_path = match path {
        Some(p) => p.to_string(),
        _none => {
            let home = std::env::var("HOME").unwrap();
            let xdg_config_home = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home));
            format!("{}/termrun/config.toml", xdg_config_home)
        }
    };
    // Check if the file exists, if not create just a default config
    if !std::path::Path::new(&config_path).exists() {
        return Ok(Config {
            background_color: Some("#000000".to_string()),
            text_color: Some("#FFFFFF".to_string()),
            border_color: Some("#FFFFFF".to_string()),
            accent_color: Some("#FF0000".to_string()),
        });
    }
    let config_str = std::fs::read_to_string(config_path).unwrap();
    toml::de::from_str(&config_str)
}