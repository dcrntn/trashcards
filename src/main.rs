use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
mod app;  // Import core
//mod misc // Import miscellaneous    

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initial message to display in the main content area
    let mut main_message = "Welcome to Trashcards!\nPress a key to start.".to_string();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Call the draw_layout function from the layout module
            app::layout::draw_layout(f, size, &main_message); // f is a Frame reference
        })?;

        // Wait for a key press and handle input
        if let Event::Key(key) = event::read()? {
            // Use the key_handler to handle keypress
            if app::key_handler::handle_keypress(key.code, &mut main_message) {
                break; // Exit the loop to end the application if the key handler signals to exit
            }
        }
    }

    // Restore terminal state before exiting
    thread::sleep(Duration::from_millis(500));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
