#![forbid(unsafe_code)]
use crossterm::event;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))
}

struct App {
    should_quit: bool,
    name: String,
}

impl App {
    fn new() -> Self {
        Self {
            should_quit: false,
            name: "world".to_string(),
        }
    }

    fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let greeting = format!("Hello, {}!", self.name);
        frame.render_widget(greeting, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::read()?.is_key_press() {
            self.should_quit = true;
        }
        Ok(())
    }
}
