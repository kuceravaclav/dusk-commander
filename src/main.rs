#![forbid(unsafe_code)]
mod app;
mod event;
mod ui;

use app::App;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))
}
