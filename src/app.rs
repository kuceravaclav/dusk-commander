#![forbid(unsafe_code)]
use crossterm::event;
use ratatui::widgets::{Paragraph, Block, Borders};
use ratatui::layout::{Constraint, Direction, Layout};

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
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(frame.area());

        let greeting = format!("Hello, {}!", self.name);
        let left_panel = Paragraph::new(greeting.clone())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Left panel"));
        
        let right_panel = Paragraph::new(greeting)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Right panel"));

        frame.render_widget(left_panel, chunks[0]);
        frame.render_widget(right_panel, chunks[1]);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::read()?.is_key_press() {
            self.should_quit = true;
        }
        Ok(())
    }
}
