use core::str;
use std::str::FromStr;
use std::fs;
use std::path::Path;
use std::env;
use toml::de::Error as TomlError;

use serde::Deserialize;
use ratatui::style::Color;

/// Config struct for the application
/// This struct is used to load the config from the config file
#[derive(Deserialize, Debug, Clone)]
struct SerializeConfig {
    icon_theme: Option<String>,
    terminal: Option<String>,
    appearance: Option<SerializeAppearance>,
}

#[derive(Deserialize, Debug, Clone)]
struct SerializeAppearance {
    search_input: Option<String>,
    text: Option<String>,
    subtext: Option<String>,
    help_text: Option<String>,
    selected_app: Option<String>,
    selected_app_text: Option<String>,
    search_border: Option<String>,
    applications_border: Option<String>,
    icon_border: Option<String>,
    info_border: Option<String>,
    actions_border: Option<String>,
    help_border: Option<String>,
}

/// Config struct for the application.
/// This struct is used for the rest of the application
#[derive(Clone)]
pub struct Config {
    pub icon_theme: String,
    pub terminal: String,
    pub appearance: Appearance,
}

#[derive(Clone)]
pub struct Appearance {
    pub search_input: Color,
    pub text: Color,
    pub subtext: Color,
    pub help_text: Color,
    pub selected_app: Color,
    pub selected_app_text: Color,
    pub search_border: Color,
    pub applications_border: Color,
    pub icon_border: Color,
    pub info_border: Color,
    pub actions_border: Color,
    pub help_border: Color,
}

pub fn load_config(path: Option<String>) -> Result<Config, String> {
    let config_path = resolve_config_path(path)?;

    if !Path::new(&config_path).exists() {
        return Ok(Config {
            icon_theme: "hicolor".to_string(),
            terminal: "kitty".to_string(),
            appearance: Appearance {
                search_input: Color::White,
                text: Color::White,
                subtext: Color::Gray,
                help_text: Color::Gray,
                selected_app: Color::Blue,
                selected_app_text: Color::Black,
                search_border: Color::Gray,
                applications_border: Color::Gray,
                icon_border: Color::Gray,
                info_border: Color::White,
                actions_border: Color::White,
                help_border: Color::White,
            }
        })
    }
    
    let config_str = read_config_file(&config_path)?;
    let imported_conf: SerializeConfig = parse_config(&config_str)?;

    Ok(Config {
        icon_theme: imported_conf.icon_theme.unwrap_or_else(|| "hicolor".to_string()),
        terminal: imported_conf.terminal.unwrap_or_else(|| "kitty".to_string()),
        appearance: Appearance {
            search_input: parse_color(imported_conf.appearance.clone().and_then(|a| a.search_input), Color::White),
            text: parse_color(imported_conf.appearance.clone().and_then(|a| a.text), Color::White),
            subtext: parse_color(imported_conf.appearance.clone().and_then(|a| a.subtext), Color::Gray),
            help_text: parse_color(imported_conf.appearance.clone().and_then(|a| a.help_text), Color::Gray),
            selected_app: parse_color(imported_conf.appearance.clone().and_then(|a| a.selected_app), Color::Blue),
            selected_app_text: parse_color(imported_conf.appearance.clone().and_then(|a| a.selected_app_text), Color::Black),
            search_border: parse_color(imported_conf.appearance.clone().and_then(|a| a.search_border), Color::Gray),
            applications_border: parse_color(imported_conf.appearance.clone().and_then(|a| a.applications_border), Color::Gray),
            icon_border: parse_color(imported_conf.appearance.clone().and_then(|a| a.icon_border), Color::Gray),
            info_border: parse_color(imported_conf.appearance.clone().and_then(|a| a.info_border), Color::White),
            actions_border: parse_color(imported_conf.appearance.clone().and_then(|a| a.actions_border), Color::White),
            help_border: parse_color(imported_conf.appearance.clone().and_then(|a| a.help_border), Color::White),
        },
    })
}

fn resolve_config_path(path: Option<String>) -> Result<String, String> {
    if let Some(p) = path {
        return Ok(p);
    }

    let home = env::var("HOME").map_err(|_| "HOME environment variable not set".to_string())?;
    let xdg_config_home = env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| format!("{}/.config", home));
    Ok(format!("{}/termlaunch/config.toml", xdg_config_home))
}

fn read_config_file(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))
    } else {
        Err(format!("Config file not found at path: {}", path))
    }
}

fn parse_config(config_str: &str) -> Result<SerializeConfig, String> {
    toml::from_str(config_str).map_err(|e: TomlError| format!("Failed to parse config file: {}", e))
}

fn parse_color(color: Option<String>, default: Color) -> Color {
    color
        .filter(|c| is_valid_color(c))
        .and_then(|c| Color::from_str(&c).ok())
        .unwrap_or(default)
}


fn is_valid_color(color: &str) -> bool {
    color.len() == 7 && color.starts_with('#') && color.chars().skip(1).all(|c| c.is_ascii_hexdigit())
}