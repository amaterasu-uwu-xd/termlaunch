use std::{os::unix::process::CommandExt, path::PathBuf, process::Stdio};

use color_eyre::eyre::Error;
use freedesktop_file_parser::{EntryType, parse};
use freedesktop_icons::lookup;

use std::process::Command;

use crate::config;

#[derive(Debug, Clone)]
pub struct Application {
    pub entry: String,
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
    // system entries, should be $XDG_DATA_DIRS/applications or /usr/local/share/applications:/usr/share/applications
    let system_entries = std::env::var("XDG_DATA_DIRS")
        .unwrap_or_else(|_| "/usr/local/share:/usr/share".to_string());

    // user entries, should be $XDG_DATA_HOME/applications or $HOME/.local/share/applications
    let user_entries = std::env::var("XDG_DATA_HOME")
        .unwrap_or_else(|_| format!("{}/.local/share", std::env::var("HOME").unwrap()));

    let mut apps: Vec<Application> = Vec::new();

    for dir in system_entries.split(':') {
        // Check if the directory exists
        if std::path::Path::new(dir).exists() {
            // Get the desktop entries
            get_desktop_entries(false, dir.to_string(), &mut apps);
        }
    }

    // Check if the user directory exists
    if std::path::Path::new(&user_entries).exists() {
        // Get the desktop entries
        get_desktop_entries(true, user_entries, &mut apps);
    }

    // Order the applications by name, case insensitive
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn get_desktop_entries(is_user: bool, path: String, apps: &mut Vec<Application>) {
    // Check the applications directory
    let app_dir = format!("{}/applications", path);
    if let Ok(entries) = std::fs::read_dir(app_dir) {
        for entry in entries.flatten() {
            if entry
                .path()
                .extension()
                .map(|s| s == "desktop")
                .unwrap_or(false)
            {
                // Get the text of the file
                let file_content =
                    std::fs::read_to_string(entry.path()).unwrap_or_else(|_| "".to_string());
                // Parse the file
                let parsed = parse(&file_content);

                if parsed.is_err() {
                    continue;
                }
                let parsed = parsed.unwrap();

                if let EntryType::Application(app) = &parsed.entry.entry_type {
                    if parsed.entry.no_display.unwrap_or(false) {
                        continue;
                    };
                    let mut actions = Vec::new();

                    if let Some(exec) = &app.exec {
                        actions.push(Action {
                            name: "Run".to_string(),
                            command: exec.to_string(),
                        });
                    }

                    for (_name, action) in parsed.actions {
                        actions.push(Action {
                            name: action.name.default,
                            command: action.exec.unwrap(),
                        });
                    }

                    let app = Application {
                        entry: entry.file_name().to_str().unwrap_or("").to_string(),
                        name: parsed.entry.name.default,
                        icon: parsed.entry.icon.unwrap_or_default().content,
                        terminal: app.terminal.unwrap_or(false),
                        comment: parsed.entry.comment.unwrap_or_default().default,
                        categories: app.categories.clone().unwrap_or_default(),
                        actions,
                    };
                    // if the entry is user, first check if it already exists in the entries, if it does, replace it, if not, push it
                    if is_user {
                        if let Some(index) = apps.iter().position(|x| x.entry == app.entry) {
                            apps[index] = app;
                        } else {
                            apps.push(app);
                        }
                    } else {
                        // if the entry is system, just push it
                        apps.push(app);
                    }
                }
            }
        }
    }
}

pub fn get_app_icon(name: String, config: &config::Config) -> Option<PathBuf> {
    lookup(name.as_str())
        .with_size(1024)
        .with_theme(&config.icon_theme)
        .find()
}

pub fn spawn_app(command: String, terminal: bool, config: &config::Config) -> Result<(), Error> {
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

    unsafe {
        command_builder
            .current_dir(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .pre_exec(|| {
                rustix::process::setsid().ok();
                Ok(())
            });
    }

    let _ = command_builder.spawn();

    Ok(())
}
