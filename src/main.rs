use clap::Parser;

mod config;
mod apps;

/// Open your desktop apps from the command line
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Alternative config file path. Defaults to $HOME/.config/termrun/config.toml or $XDG_CONFIG_HOME/termrun/config.toml
    #[arg(short, long)]
    config: Option<String>
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let load_conf = config::load_config(args.config);

    if let Err(e) = load_conf {
        eprintln!("Error loading config: {}", e);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to load config"));
    }

    let config = load_conf.unwrap();

    // Getting the applications
    let apps = apps::get_apps();
    if apps.is_empty() {
        eprintln!("No applications found");
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No applications found"));
    }

    // Search for dolphin
    let search = "dolphin";
    let mut found = false;
    for app in &apps {
        if app.name.to_lowercase().contains(&search.to_lowercase()) {
            println!("Found: {}", app.name);
            println!("Command: {}", app.command);
            println!("Icon: {}", app.icon);
            println!("Categories: {:?}", app.categories);
            found = true;
            break;
        }
    }
    if !found {
        println!("No application found with the name: {}", search);
    }

    Ok(())
}
