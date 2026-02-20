#![forbid(unsafe_code)]
use crossterm::event;
use ratatui::widgets::{Paragraph, Block, Borders};

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

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let greeting = format!("Hello, {}!", self.name);
        let paragraph = Paragraph::new(greeting)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Dusk commander"));
        frame.render_widget(paragraph, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::read()?.is_key_press() {
            self.should_quit = true;
        }
        Ok(())
    }
}
