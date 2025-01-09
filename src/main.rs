use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph, List, ListItem},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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

            // Define the layout with two areas: a side menu and a main content area
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(25), // Side menu takes 25% of the screen width
                        Constraint::Percentage(75), // Main content takes 75% of the screen width
                    ]
                    .as_ref(),
                )
                .split(size);

            // Side menu block with items
            let side_menu = List::new(vec![
                ListItem::new("1. Start Game"),
                ListItem::new("2. Settings"),
                ListItem::new("3. Info"),
                ListItem::new("q. Exit"),
            ])
            .block(Block::default().borders(Borders::ALL).title("Menu"));

            // Main content block with dynamic message
            let block = Block::default()
                .title("Trashcards")
                .borders(Borders::ALL);
            let welcome_message = Paragraph::new(main_message.clone())
                .block(Block::default().borders(Borders::ALL).title("Welcome"))
                .alignment(tui::layout::Alignment::Center);

            // Render side menu and welcome message
            f.render_widget(side_menu, layout[0]);
            f.render_widget(welcome_message, layout[1]);
            f.render_widget(block, layout[1]);
        })?;

        // Wait for a key press and handle input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => {
                    main_message = "You pressed 1: Start!".to_string();
                }
                KeyCode::Char('2') => {
                    main_message = "You pressed 2: Settings.".to_string();
                }
                KeyCode::Char('3') => {
                    main_message = r#"
                    You can set the used dataset in the settings etc.. 
                    You can start by pressing the 1 key.
                    Press q to exit
                    "#.to_string();
                }
                KeyCode::Char('q') => {
                    main_message = "You pressed q: Exiting...".to_string();
                    break; // Exit the loop to end the application
                }
                _ => {}
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
