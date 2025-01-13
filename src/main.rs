use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, thread, time::Duration};
use crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use app::file_browser::{FileBrowser}; // Import FileBrowser module
use app::game::{Game};
use app::key_handler::{self, AppState};

mod app; // Assuming app contains key_handler, layout, and file_browser modules

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut current_state = AppState::Welcome;  // Start with the Welcome state
    let mut exit_flag = false; // Flag to signal when to exit the loop

    // Initialize FileBrowser instance
    let mut file_browser = FileBrowser::new();
    let mut game = Game::new();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Call the layout drawing function from app/layout.rs
            app::layout::draw_layout(f, size, current_state, &file_browser, &game);

        })?;

        // Wait for a key press and handle input
        if let Event::Key(key) = event::read()? {
            // Update the state based on key press
            current_state = key_handler::handle_keypress(key.code, current_state, &mut file_browser, &mut game); 

            // Handle file browser state and keypresses
            if current_state == AppState::FileBrowser {
                file_browser.handle_keypress(key.code);  // Handle file browser navigation
            }

            // Handle game state and keypress
            if current_state == AppState::Game {
                game.handle_keypress(key.code);
            }
            if matches!(current_state, AppState::Exit) {
                exit_flag = true; // Set the flag to exit the loop
            }
        }

        // If exit_flag is true, break out of the loop
        if exit_flag {
            break;
        }
    }

    // Restore terminal state before exiting
    thread::sleep(Duration::from_millis(500));

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
