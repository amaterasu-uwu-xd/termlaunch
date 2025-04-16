use clap::Parser;
use fs4::fs_std::FileExt;
use std::io::Result;

mod config;
mod applications;
mod app;
mod image;

/// Open your desktop apps from the command line
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Alternative config file path. Defaults to $HOME/.config/termlaunch/config.toml or $XDG_CONFIG_HOME/termlaunch/config.toml
    #[arg(short, long)]
    config: Option<String>
}

fn main() -> Result<()> {
    let lock_file = "/tmp/termlaunch.lock";
    let file = std::fs::File::create(lock_file)?;
    let locked = file.try_lock_exclusive();
    if !locked.unwrap() {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Termlaunch is already running"))?;
    }
    let args = Args::parse();
    let _ = app::startup(args.config);

    Ok(())

}
