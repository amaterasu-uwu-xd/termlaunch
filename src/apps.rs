use freedesktop_file_parser::{EntryType, parse};

pub struct App {
    pub name: String,
    pub command: String,
    pub icon: String,
    pub categories: Vec<String>,
    pub actions: Vec<Action>,
}

pub struct Action {
    pub name: String,
    pub command: String,
}

pub fn get_apps() -> Vec<App> {
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
                            for (_name,action)  in parsed.actions {
                                actions.push(Action {
                                    name: action.name.default,
                                    command: action.exec.unwrap()
                                });
                            }

                            apps.push(App {
                                name: parsed.entry.name.clone().default,
                                command: app.exec.as_ref().unwrap().to_string(),
                                icon: parsed.entry.icon.clone().unwrap_or_default().content,
                                categories: app.categories.clone().unwrap_or_default(),
                                actions
                            });
                        }
                    }
                }
            }
        }
    }
    // Remove empty strings
    apps.retain(|app| !app.name.is_empty());
    // Remove duplicates
    apps.dedup_by(|a, b| a.name == b.name);
    // Order the applications by name
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    return apps;
}
