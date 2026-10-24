use color_eyre::eyre::Result;

pub mod cli;
pub mod launch;
pub mod socket_interaction;

fn main() -> Result<()> {
    color_eyre::install()?;

    cli::cli()
}
