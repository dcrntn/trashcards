use crossterm::event::{KeyCode};

pub fn handle_keypress(key: KeyCode, main_message: &mut String) -> bool {
    match key {
        KeyCode::Char('1') => {
            *main_message = "You pressed 1: Start!".to_string();
        }
        KeyCode::Char('2') => {
            *main_message = "You pressed 2: Settings.".to_string();
        }
        KeyCode::Char('3') => {
            *main_message = 
r#"You can set the used dataset in the settings etc..
You can start by pressing the 1 key.
Press q to exit
                    "#.to_string();
        }
        KeyCode::Char('q') => {
            *main_message = "You pressed q: Exiting...".to_string();
            return true; // Return true to indicate that the application should exit
        }
        _ => {}
    }
    false // Return false to indicate that the application should continue
}
