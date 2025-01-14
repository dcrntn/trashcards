use crossterm::event::{KeyCode};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::file_browser::FileBrowser;


pub struct Game {
    pub is_open: bool,
    current_row: usize,
    current_col: usize,
    data: Vec<Vec<String>>, // Stores the rows from the CSV
    headers: Vec<String>,   // Stores the column names
    answer_visible: bool,   // Tracks if the answer is visible
}

impl Game {
    pub fn new() -> Self {
        Game {
            is_open: false,
            current_row: 0,
            current_col: 0, // Start at column 0, waiting for space to reveal answers
            data: vec![],
            headers: vec![], // Initialize headers
            answer_visible: false, // Start with answer hidden
        }
    }
    

    pub fn load_csv(&mut self, file_browser: &FileBrowser) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref selected_file) = file_browser.selected_file {
            // Get the full file path from the file browser
            let file_path = format!("{}/{}", file_browser.current_directory, selected_file);
            let num_rows = 5;

            // Process the CSV file
            let (headers, rows) = crate::app::file_handler::read_and_process_csv_with_headers(&file_path, num_rows)?;
            self.headers = headers;
            self.data = rows;
            Ok(())
        } else {
            Err("No file selected".into())
        }
    }

    pub fn toggle(&mut self, file_browser: &FileBrowser) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.load_csv(file_browser).unwrap_or_else(|err| eprintln!("Error loading CSV: {}", err));
        }
    }

    pub fn handle_keypress(&mut self, key: KeyCode) {
        if key == KeyCode::Char(' ') {
            self.next();
        }
    }

    pub fn next(&mut self) {
        if self.current_row < self.data.len() {
            if self.answer_visible {
                // If the answer is currently visible, hide it
                self.answer_visible = false;
            } else {
                // Show the answer or move to the next column
                self.answer_visible = true;
                if self.current_col >= self.data[self.current_row].len() - 1 {
                    self.current_col = 0; // Reset to the question column
                    self.answer_visible = false;
                    self.current_row += 1; // Move to the next row
                    if self.current_row >= self.data.len() {
                        self.current_row = 0; // Loop back to the beginning
                    }
                } else {
                    self.current_col += 1; // Move to the next column
                }
            }
        }
    }

    pub fn draw_popup(&self, f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: Rect) {
        // Question content with header
        let question = if self.current_row < self.data.len() {
            format!("{}\n\n{}", self.headers[0], self.data[self.current_row][0]) // Always show the first column as the question
        } else {
            "".to_string()
        };
    
        // Answer content or header only
        let answer_text = if self.current_row < self.data.len() && self.current_col > 0 {
            if self.answer_visible {
                // Show header and answer
                format!("{}\n\n{}", self.headers[self.current_col], self.data[self.current_row][self.current_col])
            } else {
                // Show header only
                let mut next_header = self.current_col + 1;
                if next_header > self.headers.len() - 1 {
                    next_header = 0;
                }
                if next_header == 0 {
                    "Press space for next question!".to_string()
                }
                else {
                    self.headers[next_header].clone()
                }
            }
        } else {
            self.headers[self.current_col + 1].clone()
        };
    
        // Create the question widget
        let question_widget = Paragraph::new(question)
            .block(Block::default().title("Question").borders(Borders::ALL))
            .alignment(Alignment::Center);
    
        // Create the answer widget
        let answer_widget = Paragraph::new(answer_text)
            .block(Block::default().title("Answer").borders(Borders::ALL))
            .alignment(Alignment::Center);
    
        // Split the layout
        let chunks = tui::layout::Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    tui::layout::Constraint::Percentage(50),
                    tui::layout::Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(size);
    
        // Render the widgets
        f.render_widget(question_widget, chunks[0]);
        f.render_widget(answer_widget, chunks[1]);
    }
    
    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_initialization() {
        let game = Game::new();
        assert_eq!(game.is_open, false);
        assert_eq!(game.current_row, 0);
        assert_eq!(game.current_col, 0);
        assert_eq!(game.data.len(), 0);
        assert_eq!(game.headers.len(), 0);
        assert_eq!(game.answer_visible, false);
    }

    #[test]
    fn test_game_toggle() {
        let mut game = Game::new();
        let file_browser = FileBrowser::new(); // Mock file browser

        game.toggle(&file_browser);
        assert!(game.is_open);

        game.toggle(&file_browser);
        assert!(!game.is_open);
    }

    #[test]
    fn test_next_question() {
        let mut game = Game::new();
        game.headers = vec!["Question".to_string(), "Answer".to_string()];
        game.data = vec![
            vec!["What is Rust?".to_string(), "A systems programming language.".to_string()],
            vec!["What is Cargo?".to_string(), "Rust's package manager.".to_string()],
        ];

        assert_eq!(game.current_row, 0);
        assert_eq!(game.current_col, 0);

        // Simulate showing the first question's answer
        game.next();
        assert!(game.answer_visible);
        assert_eq!(game.current_col, 1);

        // Simulate moving to the next question
        // Needs double press cuz at the end there is a msg that indicates the next question is coming up!
        game.next();
        game.next();
        assert!(!game.answer_visible);
        assert_eq!(game.current_row, 1);
        assert_eq!(game.current_col, 0);
    }

    #[test]
    fn test_answer_visibility_toggle() {
        let mut game = Game::new();
        game.headers = vec!["Question".to_string(), "Answer".to_string()];
        game.data = vec![vec!["What is Rust?".to_string(), "A systems programming language.".to_string()]];

        assert!(!game.answer_visible);

        // Show answer
        game.next();
        assert!(game.answer_visible);

        // Hide answer
        game.next();
        assert!(!game.answer_visible);
    }

    #[test]
    fn test_loop_back_to_start() {
        let mut game = Game::new();
        game.headers = vec!["Question".to_string(), "Answer".to_string()];
        game.data = vec![vec!["What is Rust?".to_string(), "A systems programming language.".to_string()]];

        // Move through the single question-answer pair
        game.next();
        // Needs double press cuz at the end there is a msg that indicates the next question is coming up!
        game.next();
        game.next();

        // Should loop back to the first question
        assert_eq!(game.current_row, 0);
        assert_eq!(game.current_col, 0);
        assert!(!game.answer_visible);
    }
}
