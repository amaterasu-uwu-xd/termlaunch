use clap::Parser;

mod config;
mod applications;
mod app;
mod image;

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

    let _ = app::startup(args.config);

    Ok(())
}
