mod inject;
mod start;

use clap::{ArgAction, Parser, Subcommand};
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(name = "rtdbg", version, about = "rtdbg - A realtime debugging toolkit", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Attach the rtdbg runtime or launch a program with it
    Start {
        /// Uses LD_PRELOAD for injection instead (for ptrace sensitive programs)
        #[arg(short, long, action = ArgAction::SetTrue)]
        preload: bool,

        /// PID of the process or the path of the program to inject into (if using LD_PRELOAD)
        #[arg(value_name = "PID|PROGRAM")]
        program: String,
    },

    /// Inject runtime scripts into programs that have the rtdbg runtime
    Inject {
        /// PID of the process that has rtdbg_runtime injected
        #[arg(value_name = "PID")]
        pid: String,

        /// The script to inject into the process
        #[arg(value_name = "SCRIPT")]
        script: String,
    },
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let command = args.command;

    match command {
        Commands::Start { preload, program } => start::start(preload, program),
        Commands::Inject { pid, script } => inject::inject(pid, script),
    }
}
