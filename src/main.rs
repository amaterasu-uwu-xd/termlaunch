use clap::Parser;
use fs4::fs_std::FileExt;

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
    let lock_file = "/tmp/termrun.lock";
    let file = std::fs::File::create(lock_file)?;
    let locked = file.try_lock_exclusive();
    if !locked.unwrap() {    
        std::process::exit(1);
    }
    let args = Args::parse();
    let _ = app::startup(args.config);
    Ok(())

}
