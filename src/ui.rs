use ratatui::Frame;
use ratatui::widgets::{Paragraph, Block, Borders};
use ratatui::layout::{Constraint, Direction, Layout};


use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.area());

    let greeting = format!("Hello, {}!", app.name());
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
