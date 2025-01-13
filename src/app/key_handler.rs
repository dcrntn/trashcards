use crossterm::event::KeyCode;
use crate::app::file_browser::{FileBrowser};
use crate::app::game::{Game};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    Welcome,
    Settings,
    Info,
    FileBrowser,  // New state for file browser
    Exit,
    Game,
    SelectFile
}

pub fn handle_keypress(key: KeyCode, current_state: AppState, file_browser: &mut FileBrowser, game: &mut Game) -> AppState {
    match key {
        KeyCode::Char('1') => AppState::Welcome,
        KeyCode::Char('2') => {
            // Force close the fb so it works normally on reopen
            file_browser.is_open = false;

            AppState::Settings
        }
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
        KeyCode::Char('s') => {
            if current_state == AppState::Welcome {
                game.toggle(file_browser);
                if let Some(ref selected_file) = file_browser.selected_file{
                    if game.is_open {
                        AppState::Game
                    } else {
                        AppState::Welcome
                    }
                } else {
                    AppState::SelectFile
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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;
    use crate::app::file_browser::FileBrowser; // Make sure this path is correct for your project

    // Test case for pressing '1' to transition to the Welcome state
    #[test]
    fn test_handle_keypress_welcome() {
        let mut file_browser = FileBrowser::new();

        // Initial state is Settings
        let current_state = AppState::Settings;

        // Simulate pressing '1'
        let new_state = handle_keypress(KeyCode::Char('1'), current_state, &mut file_browser);

        // Assert that the state transitions to Welcome
        assert_eq!(new_state, AppState::Welcome);
    }

    // Test case for pressing '2' to transition to the Settings state
    #[test]
    fn test_handle_keypress_settings() {
        let mut file_browser = FileBrowser::new();

        // Initial state is Welcome
        let current_state = AppState::Welcome;

        // Simulate pressing '2'
        let new_state = handle_keypress(KeyCode::Char('2'), current_state, &mut file_browser);

        // Assert that the state transitions to Settings
        assert_eq!(new_state, AppState::Settings);
    }

    // Test case for pressing '3' to transition to the Info state
    #[test]
    fn test_handle_keypress_info() {
        let mut file_browser = FileBrowser::new();

        // Initial state is Settings
        let current_state = AppState::Settings;

        // Simulate pressing '3'
        let new_state = handle_keypress(KeyCode::Char('3'), current_state, &mut file_browser);

        // Assert that the state transitions to Info
        assert_eq!(new_state, AppState::Info);
    }

    // Test case for pressing 'q' to exit the app
    #[test]
    fn test_handle_keypress_exit() {
        let mut file_browser = FileBrowser::new();

        // Initial state is Info
        let current_state = AppState::Info;

        // Simulate pressing 'q'
        let new_state = handle_keypress(KeyCode::Char('q'), current_state, &mut file_browser);

        // Assert that the state transitions to Exit
        assert_eq!(new_state, AppState::Exit);
    }

    // Test case for pressing 'l' to toggle the FileBrowser state from Settings
    #[test]
    fn test_handle_keypress_toggle_file_browser_from_settings() {
        let mut file_browser = FileBrowser::new();

        // Initial state is Settings
        let current_state = AppState::Settings;

        // Simulate pressing 'l' to toggle the file browser
        let new_state = handle_keypress(KeyCode::Char('l'), current_state, &mut file_browser);

        // Assert that the state transitions to FileBrowser
        assert_eq!(new_state, AppState::FileBrowser);
        assert!(file_browser.is_open, "File browser should be open after pressing 'l'");

        // Simulate pressing '2' to go to the Settings
        let new_state_after_close = handle_keypress(KeyCode::Char('2'), new_state, &mut file_browser);

        // Assert that the file browser is closed and the state remains Settings
        assert_eq!(new_state_after_close, AppState::Settings);
        assert!(!file_browser.is_open, "File browser should be closed after pressing 'Esc'");
    }

    // Test case for pressing any other key (no transition)
    #[test]
    fn test_handle_keypress_no_transition() {
        let mut file_browser = FileBrowser::new();

        // Initial state is Settings
        let current_state = AppState::Settings;

        // Simulate pressing an unhandled key (e.g., 'x')
        let new_state = handle_keypress(KeyCode::Char('x'), current_state, &mut file_browser);

        // Assert that the state remains the same
        assert_eq!(new_state, AppState::Settings);
    }
}

