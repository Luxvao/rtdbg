use clap::Parser;
use color_eyre::eyre::Result;

use crate::repl;

#[derive(clap::Parser, Debug)]
#[command(version, about = "CLI for Rtdbg", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    #[command(about = "Launch the rtdbg repl")]
    Repl,
    // #[command(about = "Launch a program, injecting rtdbg_runtime.so using LD_PRELOAD")]
    // Launch { path: PathBuf },
    // #[command(about = "Load a script from PATH")]
    // LoadScript { pid: i32, path: PathBuf },
    // #[command(about = "Unload a script")]
    // UnloadScript { pid: i32, script_id: u64 },
    // #[command(about = "List all the functions of loaded script")]
    // ListFunctions { pid: i32, script_id: u64 },
    // #[command(about = "Run a remote rhai function associated with a script")]
    // RunFunction {
    //     pid: i32,
    //     script_id: u64,
    //     function: String,
    //     parameter: Option<String>,
    // },
}

pub fn cli() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Repl => repl::init()?,
        // Command::Launch { path } => launch(path)?,
        // Command::LoadScript { pid, path } => load_script(pid, &path)?,
        // Command::UnloadScript { pid, script_id } => {
        //     unload_script(pid, &librtdbg::script::ScriptId(script_id))?
        // }
        // Command::ListFunctions { pid, script_id } => {
        //     script_functions(pid, &librtdbg::script::ScriptId(script_id))?
        // }
        // Command::RunFunction {
        //     pid,
        //     script_id,
        //     function,
        //     parameter,
        // } => run_function(
        //     pid,
        //     &librtdbg::script::ScriptId(script_id),
        //     function,
        //     parameter.as_deref(),
        // )?,
    }

    Ok(())
}
