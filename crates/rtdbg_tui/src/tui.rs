use color_eyre::eyre::Result;
use ratatui::{DefaultTerminal, Frame};

pub fn init() -> Result<()> {
    ratatui::run(|t| App::app(&mut App::default(), t))?;

    Ok(())
}

#[derive(Debug, Default)]
struct App {
    exit: bool,
}

impl App {
    fn app(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;

            if self.exit {
                break Ok(());
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget("hello world", frame.area());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
