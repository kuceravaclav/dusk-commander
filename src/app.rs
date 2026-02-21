#![forbid(unsafe_code)]
use crossterm::event::{Event, read as crossterm_read};

use crate::ui;
use crate::event;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct App {
    should_quit: bool,
    name: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            name: "world".to_string(),
        }
    }

    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn name(&mut self) -> String {
        self.name.clone()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        ui::render(self, frame);
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = crossterm_read()? {
            event::handle_key_event(self, key);
        }
        Ok(())
    }
}
