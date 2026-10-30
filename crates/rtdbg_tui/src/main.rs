#![feature(vec_try_remove)]

use color_eyre::eyre::Result;

mod cli;
mod components;
mod inject;
mod script;
mod socket;
mod tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    cli::init()
}
