use color_eyre::eyre::Result;

mod socket;
mod style;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;

    ui::init()
}
