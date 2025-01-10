use tui::{
    layout::{Layout, Constraint, Direction},
    widgets::{Paragraph, List, ListItem, Block, Borders},
    Frame,
};
use tui::backend::CrosstermBackend;

pub fn draw_layout(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: tui::layout::Rect, main_message: &str) {
    // Define the layout with three areas: upper bar, side menu, and main content area
    let layout = Layout::default()
        .direction(Direction::Vertical) // Overall layout is vertical
        .constraints(
            [
                Constraint::Length(3), // Upper bar with a fixed height of 3 units
                Constraint::Percentage(100), // Remaining space for the side menu + main content
            ]
            .as_ref(),
        )
        .split(size);

    // Upper bar block for displaying information
    let upper_bar = Paragraph::new("Info Bar: Some useful information here...")
        .block(Block::default().borders(Borders::ALL).title("Upper Bar"))
        .alignment(tui::layout::Alignment::Center);

    // Render the upper bar at the top of the terminal
    f.render_widget(upper_bar, layout[0]);

    // Split the remaining space (layout[1]) horizontally into the side menu and the main content
    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal) // Split horizontally for side menu and content
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

    // Main content block with dynamic message
    let welcome_message = Paragraph::new(main_message)
        .block(Block::default().borders(Borders::ALL).title("Welcome"))
        .alignment(tui::layout::Alignment::Center);

    // Render side menu and welcome message in their respective areas
    f.render_widget(side_menu, horizontal_layout[0]);
    f.render_widget(welcome_message, horizontal_layout[1]);
}
