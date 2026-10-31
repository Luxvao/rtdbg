use std::path::{Path, PathBuf};

use clap::Parser;
use color_eyre::eyre::Result;

use crate::{script::load_from_disk, tui::App};

#[derive(clap::Parser, Debug)]
#[command(version, about = "RTDBG TUI")]
struct Cli {
    #[arg(short, long, help = "Provide a path to populate local scripts")]
    scripts: Option<PathBuf>,
}

pub fn init() -> Result<()> {
    let cli = Cli::parse();

    let mut app = App::default();

    app.output.read_only(true);

    if let Some(scripts_path) = cli.scripts {
        load_scripts(&mut app, &scripts_path)?;
    }

    ratatui::run(|t| App::app(&mut app, t))?;

    Ok(())
}

fn load_scripts(app: &mut App, path: &Path) -> Result<()> {
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;

        let path = entry.path();

        if path.extension().map(|s| s.to_str()) == Some(Some("rhai")) && path.is_file() {
            app.scripts_mgr.local_scripts.push(load_from_disk(&path)?);
        }
    }

    Ok(())
}
