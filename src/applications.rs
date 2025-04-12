use std::path::PathBuf;

use freedesktop_file_parser::{EntryType, parse};
use freedesktop_icons::lookup;

use fork::{daemon, Fork};
use std::process::Command;

use crate::config;

#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub comment: String,
    pub icon: String,
    pub terminal: bool,
    pub categories: Vec<String>,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub command: String,
}

pub fn get_apps() -> Vec<Application> {
    let binding = std::env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/local/share:/usr/share".to_string());
    let mut dirs = binding.split(':').collect::<Vec<&str>>();
    let binding = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| format!("{}/.local/share", std::env::var("HOME").unwrap()));
    dirs.push(&binding);

    let mut apps = Vec::new();

    // Iterate over the directories
    for dir in dirs {
        // Check the applications directory
        let app_dir = format!("{}/applications", dir);
        if let Ok(entries) = std::fs::read_dir(app_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if entry
                        .path()
                        .extension()
                        .map(|s| s == "desktop")
                        .unwrap_or(false)
                    {
                        // Get the text of the file
                        let file_content = std::fs::read_to_string(entry.path())
                            .unwrap_or_else(|_| "".to_string());
                        // Parse the file
                        let parsed = parse(&file_content).unwrap();
                        // Check if the entry should be visible
                        if let EntryType::Application(app) = &parsed.entry.entry_type {
                            // Check if the entry is visible
                            if parsed.entry.no_display.unwrap_or(false) == true {
                                continue
                            };
                            // Get the actions 
                            let mut actions = Vec::new();

                            // add app.exec.as_ref().unwrap() to the actions
                            if let Some(exec) = &app.exec {
                                actions.push(Action {
                                    name: "Run".to_string(),
                                    command: exec.to_string()
                                });
                            }
                            for (_name,action)  in parsed.actions {
                                actions.push(Action {
                                    name: action.name.default,
                                    command: action.exec.unwrap()
                                });
                            }

                            apps.push(Application {
                                name: parsed.entry.name.default,
                                icon: parsed.entry.icon.unwrap_or_default().content,
                                terminal: app.terminal.unwrap_or(false),
                                comment: parsed.entry.comment.unwrap_or_default().default,
                                categories: app.categories.clone().unwrap_or_default(),
                                actions
                            });
                        }
                    }
                }
            }
        }
    }
    // Order the applications by name, case insensitive
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    return apps;
}

pub fn get_app_icon(name: String, config: &config::Config) -> Option<PathBuf>
{
    lookup(name.as_str())
        .with_size(1024)
        .with_theme(&config.icon_theme)
        .find()
}

pub fn spawn_app(command: String, terminal: bool, config: &config::Config) {
    // Split the command into arguments, if it contains spaces, except if its in quotes
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut skip_next = false;
    for c in command.chars() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if c == '\\' {
            skip_next = true;
            continue;
        }
        if c == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if c == ' ' && !in_quotes {
            args.push(current_arg.clone());
            current_arg.clear();
            continue;
        }
        current_arg.push(c);
    }
    if !current_arg.is_empty() {
        args.push(current_arg);
    }

    // Remove the arguments that starts with %
    args.retain(|arg| !arg.starts_with('%'));

    let program = args[0].clone();
    // remove the first argument
    args.remove(0);

    let mut command_builder: Command;

    command_builder = if terminal {
        let mut cmd = Command::new(&config.terminal);
        cmd.arg("-e").arg(program);
        cmd
    } else {
        Command::new(program)
    };

    for arg in args {
        command_builder.arg(arg);
    }

    if let Ok(Fork::Child) = daemon(false, false) {
        _ = command_builder
            .spawn();
    }

}