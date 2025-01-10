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
