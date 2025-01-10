use tui::{Frame, backend::CrosstermBackend};
use crossterm::event::{KeyCode};
use std::fs::{self};
use tui::widgets::{List, ListItem, Paragraph};
use tui::layout::{Rect, Alignment};
use std::path::{Path, PathBuf};

pub struct FileBrowser {
    pub is_open: bool,
    pub current_directory: String,
    pub selected_index: usize, // Track the currently selected file in the list
    file_list: Vec<String>,  // Store file list as a vector of Strings, which can include ".."
    pub selected_file: Option<String>, // Store the selected file here
    pub prompt_user: bool, // Flag to display the prompt to start playing or continue browsing
}

impl FileBrowser {
    // Constructor to create a new FileBrowser instance
    pub fn new() -> Self {
        let current_directory = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .to_str()
            .unwrap_or("/").to_string();

        FileBrowser {
            is_open: false,
            current_directory,
            selected_index: 0, // Default to the first file in the list
            file_list: Vec::new(),
            selected_file: None, // Initially, no file is selected
            prompt_user: false, // Initially no prompt
        }
    }

    // Toggle function to switch between file browser states
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
        if self.is_open {
            self.load_file_list(); // Load the file list when opening the file browser
        }
    }

    // Function to load the file list from the current directory
    fn load_file_list(&mut self) {
        let mut entries = fs::read_dir(&self.current_directory)
            .unwrap_or_else(|_| fs::read_dir("/").unwrap()) // Handle error by falling back to root directory
            .filter_map(|entry| entry.ok()) // Filter out errors
            .map(|entry| entry.file_name().into_string().unwrap_or_else(|_| String::from("Unknown")))
            .collect::<Vec<String>>();

        // Add ".." to navigate to the parent directory if we are not at the root
        if self.current_directory != "/" {
            let _parent_dir = Path::new(&self.current_directory).parent().unwrap_or_else(|| Path::new("/"));
            entries.insert(0, String::from("..")); // Add ".." as a string
        }

        self.file_list = entries;
    }

    // Function to draw the file browser as a popup in the main window
    pub fn draw_popup(&self, f: &mut Frame<CrosstermBackend<std::io::Stdout>>, size: Rect) {
        if self.is_open {
            // Ensure the selected index is within bounds
            let items: Vec<ListItem> = self.file_list.iter().enumerate().map(|(i, file)| {
                // Check if the item is a directory and append '/' to the name
                let display_name = if file == ".." {
                    String::from("..")  // Special case for ".."
                } else {
                    let full_path = Path::new(&self.current_directory).join(file);
                    if full_path.is_dir() {
                        format!("{}/", file)  // Add '/' for directories
                    } else {
                        file.clone() // Just the file name for regular files
                    }
                };

                let mut item = ListItem::new(display_name);
                if i == self.selected_index {
                    item = item.style(tui::style::Style::default().fg(tui::style::Color::Yellow)); // Highlight the selected file
                }
                item
            }).collect();

            let list = List::new(items)
                .block(tui::widgets::Block::default().title("File Browser").borders(tui::widgets::Borders::ALL));

            f.render_widget(list, size);

            // If a file is selected, prompt the user
            if self.prompt_user {
                let prompt_message = Paragraph::new("Press '1' to start playing, or any other key to continue browsing.")
                    .block(tui::widgets::Block::default().title("Prompt").borders(tui::widgets::Borders::ALL))
                    .alignment(Alignment::Center);

                f.render_widget(prompt_message, size);
            }
        }
    }

    // Function to handle keypresses for file navigation (navigate directories and select files)
    pub fn handle_keypress(&mut self, key: KeyCode) {
        match key {
            // Navigate through the file list with arrow keys
            KeyCode::Down => {
                self.selected_index = (self.selected_index + 1) % self.get_file_count();
            }
            KeyCode::Up => {
                if self.selected_index == 0 {
                    self.selected_index = self.get_file_count() - 1;
                } else {
                    self.selected_index -= 1;
                }
            }
            // Enter key to "select" or enter a directory
            KeyCode::Enter => {
                // First, get the selected file
                if let Some(selected_file) = self.get_selected_file() {
                    // Temporarily hold the selected file in a variable
                    let file_to_select = selected_file.clone();
    
                    // If ".." is selected, navigate to the parent directory
                    if file_to_select == ".." {
                        self.navigate_up();
                    } else {
                        let full_path = Path::new(&self.current_directory).join(&file_to_select);
                        if full_path.is_dir() {
                            // If it's a directory, enter it
                            self.current_directory = full_path.to_str().unwrap_or("/").to_string();
                            self.selected_index = 0; // Reset selection when entering a new directory
                            self.load_file_list(); // Reload the file list when entering a new directory
                        } else {
                            // Now assign the selected file to `selected_file` after other logic
                            self.selected_file = Some(file_to_select);
            
                            // Set prompt_user to true to show the prompt to start playing
                            self.prompt_user = true;
                            // We do not close the file browser yet, so the user can choose
                        }
                    }
    
                }
                
            }
            // Handle any other keypress when prompt is shown (continue browsing)
            KeyCode::Esc | KeyCode::Char(_) => {
                if self.prompt_user {
                    self.prompt_user = false;  // Hide the prompt and let the user continue browsing
                }
            }
            _ => {}
        }
    }

    // Helper function to get the total count of files (and directories)
    fn get_file_count(&self) -> usize {
        self.file_list.len() // We now just return the length of file_list
    }

    // Helper function to get the currently selected file or directory
    pub fn get_selected_file(&self) -> Option<&String> {
        if !self.file_list.is_empty() {
            Some(&self.file_list[self.selected_index]) // Return a reference to the selected file name
        } else {
            None // Return None if the file list is empty
        }
    }

    // Function to navigate up to the parent directory
    fn navigate_up(&mut self) {
        if let Some(parent) = Path::new(&self.current_directory).parent() {
            self.current_directory = parent.to_str().unwrap_or("/").to_string();
            self.selected_index = 0; // Reset selection when going back to a parent directory
            self.load_file_list(); // Reload the file list after navigating up
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode};
    use std::env;
    use std::fs::{self};
    use std::path::Path;

    // Test case for initializing a new FileBrowser instance
    #[test]
    fn test_new() {
        let file_browser = FileBrowser::new();

        // Verify that the initial state is correct
        assert_eq!(file_browser.is_open, false);
        assert_eq!(file_browser.current_directory, env::current_dir().unwrap().to_str().unwrap());
        assert_eq!(file_browser.selected_index, 0);
        assert_eq!(file_browser.file_list.len(), 0); // The file list should be empty at the start
        assert_eq!(file_browser.selected_file, None);
        assert_eq!(file_browser.prompt_user, false);
    }

    // Test case for toggling the file browser
    #[test]
    fn test_toggle() {
        let mut file_browser = FileBrowser::new();

        // Initially, the file browser should be closed
        assert_eq!(file_browser.is_open, false);

        // Toggle it to open
        file_browser.toggle();
        assert_eq!(file_browser.is_open, true);

        // Toggle it again to close
        file_browser.toggle();
        assert_eq!(file_browser.is_open, false);
    }

    // Test case for loading a file list from a directory
    #[test]
    fn test_load_file_list() {
        let mut file_browser = FileBrowser::new();

        // Using a directory that we know exists, e.g., "/tmp" on Unix-based systems.
        // On Windows, you can substitute this with a known directory like "C:\\Windows".
        file_browser.current_directory = String::from("/tmp"); 
        file_browser.load_file_list();

        // Verify that the file list is populated (assuming the directory exists).
        assert!(!file_browser.file_list.is_empty(), "File list should not be empty");
    }

    // Test case for selecting a file
    #[test]
    fn test_select_file() {
        let mut file_browser = FileBrowser::new();

        // Set a specific directory for testing
        file_browser.current_directory = String::from(".."); // Or use a path suitable for your testing environment
        file_browser.load_file_list();

        // Now, simulate a file that exists within the loaded directory
        // For testing purposes, let's simulate that "test_file.txt" exists in the current directory
        let test_file = "huhcat.gif";  // Set the name of a test file here
        
        // Make sure the file is in the file list for testing (add manually if needed)
        if !file_browser.file_list.contains(&test_file.to_string()) {
            file_browser.file_list.push(test_file.to_string());
        }

        // Set the selected index to point to the specific file we want to test
        file_browser.selected_index = file_browser.file_list.iter().position(|x| x == test_file).unwrap();

        // Simulate selecting the file
        file_browser.handle_keypress(KeyCode::Enter);

        // Check that the selected file matches the one we were testing
        assert_eq!(file_browser.selected_file, Some(test_file.to_string()), "The selected file should be the one we tested for.");
        assert_eq!(file_browser.prompt_user, true, "Prompt user should be true after file selection");
    }
    // Test case for navigating up a directory
    #[test]
    fn test_navigate_up() {
        let mut file_browser = FileBrowser::new();
        file_browser.current_directory = String::from("/tmp/subdir"); // Example directory
        let original_directory = file_browser.current_directory.clone();

        // Navigate up one directory
        file_browser.navigate_up();
        assert_ne!(file_browser.current_directory, original_directory, "Current directory should change after navigation");
        assert_eq!(file_browser.selected_index, 0, "Selected index should be reset after navigating up");
    }

    // Test case for handling navigation keys (Up/Down)
    #[test]
    fn test_navigation_keys() {
        let mut file_browser = FileBrowser::new();

        // Simulate a directory with files (mock or use an existing directory)
        file_browser.current_directory = String::from("/tmp"); // Example path, adapt as necessary
        file_browser.load_file_list();

        // Initial selected index should be 0
        assert_eq!(file_browser.selected_index, 0, "Selected index should start at 0");

        // Simulate pressing Down key to navigate
        file_browser.handle_keypress(KeyCode::Down);
        assert_eq!(file_browser.selected_index, 1, "Selected index should move down");

        // Simulate pressing Up key to navigate back
        file_browser.handle_keypress(KeyCode::Up);
        assert_eq!(file_browser.selected_index, 0, "Selected index should move up");
    }

    // Test case for handling other key presses while the prompt is shown
    #[test]
    fn test_handle_prompt_other_keypress() {
        let mut file_browser = FileBrowser::new();

        // Simulate selecting a file
        file_browser.selected_file = Some(String::from("test_file"));
        file_browser.prompt_user = true;

        // Simulate pressing a key other than '1' to continue browsing
        file_browser.handle_keypress(KeyCode::Char('x'));
        assert_eq!(file_browser.prompt_user, false, "Prompt should disappear after pressing a different key");
    }
}
