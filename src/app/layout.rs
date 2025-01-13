use tui::{Frame, backend::CrosstermBackend, widgets::{Block, Borders, Paragraph, List, ListItem}, layout::{Layout, Constraint, Direction}};
use crate::app::key_handler::AppState;
use crate::app::file_browser::FileBrowser;
use crate::app::game::Game;

pub fn draw_layout(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: tui::layout::Rect, current_state: AppState, file_browser: &FileBrowser, game: &Game) {
    // Define the layout with three areas: upper bar, side menu, and main content area
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Upper bar with a fixed height of 3 units
                Constraint::Percentage(100), // Remaining space for the side menu + main content
            ]
            .as_ref(),
        )
        .split(size);

    // Info Bar block for displaying useful information
    let info_bar_text = if let Some(ref selected_file) = file_browser.selected_file {
        // Display the full path (current directory + file name)
        format!("Selected file: {}/{}", file_browser.current_directory, selected_file)
    } else {
        String::from("Info Bar: No file selected")
    };

    // Upper bar block for displaying information (including selected file)
    let upper_bar = Paragraph::new(format!("Info Bar: {}", info_bar_text))
        .block(Block::default().borders(Borders::ALL).title("Upper Bar"))
        .alignment(tui::layout::Alignment::Center);

    // Render the upper bar at the top of the terminal
    f.render_widget(upper_bar, layout[0]);

    // Split the remaining space (layout[1]) horizontally into the side menu and the main content
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20), // Side menu takes 20% of the width
                Constraint::Percentage(80), // Main content takes 80% of the width
            ]
            .as_ref(),
        )
        .split(layout[1]);

    // Side menu block with items
    let side_menu = List::new(vec![
        ListItem::new("1. Start Game"),
        ListItem::new("2. Settings"),
        ListItem::new("3. Info"),
        ListItem::new("q. Exit"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Menu"));

    // Render the side menu
    f.render_widget(side_menu, horizontal_layout[0]);

    // Render the main content based on the current state
    match current_state {
        AppState::Welcome => {
            let welcome_message = Paragraph::new("Welcome to Trashcards!\nPress 's' key to start.")
                .block(Block::default().borders(Borders::ALL).title("Welcome"));
            f.render_widget(welcome_message, horizontal_layout[1]);
        }
        AppState::Settings => {
            let settings_message = Paragraph::new("Settings widget is here! Press 'l' to select the dataset you want to use")
                .block(Block::default().borders(Borders::ALL).title("Settings"));
            f.render_widget(settings_message, horizontal_layout[1]);
        }
        AppState::Info => {
            let info_message = Paragraph::new("Here is some info about the app!")
                .block(Block::default().borders(Borders::ALL).title("Info"));
            f.render_widget(info_message, horizontal_layout[1]);
        }
        AppState::FileBrowser => {
            // Only show file browser in the main content area
            file_browser.draw_popup(f, horizontal_layout[1]);
        }
        AppState::Game => {
            game.draw_popup(f, horizontal_layout[1]);
        }
        AppState::SelectFile => {
            let info_message = Paragraph::new("Select a file first in the settigns!")
                .block(Block::default().borders(Borders::ALL).title("Info"));
            f.render_widget(info_message, horizontal_layout[1]);
        }
        AppState::Exit => {
            // Do nothing or maybe render an "Exiting" message
            let exit_message = Paragraph::new("Exiting the application...")
                .block(Block::default().borders(Borders::ALL).title("Exit"));
            f.render_widget(exit_message, horizontal_layout[1]);
        }
    }
}
