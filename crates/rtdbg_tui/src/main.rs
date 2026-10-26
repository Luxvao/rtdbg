use color_eyre::eyre::Result;

mod tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    tui::init()
}
