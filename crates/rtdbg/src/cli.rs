use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;

use crate::{launch::launch, socket_interaction::load_script};

#[derive(clap::Parser, Debug)]
#[command(version, about = "CLI for Rtdbg", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    #[command(about = "Launch a program, injecting rtdbg_runtime.so using LD_PRELOAD")]
    Launch { path: PathBuf },
    #[command(about = "Load a script from PATH")]
    LoadScript { pid: i32, path: PathBuf },
}

pub fn cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Launch { path } => launch(path)?,
        Command::LoadScript { pid, path } => load_script(pid, path)?,
    }

    Ok(())
}
