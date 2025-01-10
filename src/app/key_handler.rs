use crossterm::event::KeyCode;
use crate::app::file_browser::{FileBrowser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Welcome,
    Settings,
    Info,
    FileBrowser,  // New state for file browser
    Exit,
}

pub fn handle_keypress(key: KeyCode, current_state: AppState, file_browser: &mut FileBrowser) -> AppState {
    match key {
        KeyCode::Char('1') => AppState::Welcome,
        KeyCode::Char('2') => AppState::Settings,
        KeyCode::Char('3') => AppState::Info,
        
        // 'l' key toggles file browser state when in Settings
        KeyCode::Char('l') => {
            if current_state == AppState::Settings {
                // Toggle file browser when in Settings state
                file_browser.toggle();
                if file_browser.is_open {
                    AppState::FileBrowser  // Transition to the File Browser state
                } else {
                    AppState::Settings  // Stay in Settings if file browser is closed
                }
            } else {
                current_state
            }
        }
        

        // Close the app with 'q' key
        KeyCode::Char('q') => AppState::Exit,

        // Keep the current state if any other key is pressed
        _ => current_state,
    }
}
