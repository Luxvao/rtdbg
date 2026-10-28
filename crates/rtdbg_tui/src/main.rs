use color_eyre::eyre::Result;

mod components;
mod inject;
mod script;
mod socket;
mod tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    tui::init()
}
