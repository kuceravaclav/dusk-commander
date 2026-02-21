#![forbid(unsafe_code)]
use crossterm::event::{KeyCode, KeyEvent};                                                                                                                                                    
 
use crate::app::App;
 
pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.quit(),
        _ => {}
    }   
}
