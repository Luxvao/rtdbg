use color_eyre::eyre::Result;

mod argkind;
mod cli;
mod completer;
mod hinter;
mod launch;
mod repl;
mod socket;

fn main() -> Result<()> {
    color_eyre::install()?;

    cli::cli()
}
