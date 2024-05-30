pub enum InputMode {
    Normal,
    Editing,
}

pub struct Input {
    pub input: String, // displayed str
    pub cursor_index: usize, // position of cursor
    pub mode: InputMode, 
    pub messages: Vec<String>, // history
}

impl Input {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            mode: InputMode::Normal,
            messages: Vec::new(),
            cursor_index: 0,
        }
    }

    pub fn move_cursor_left(&mut self) -> String {
        let cursor_moved_left = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left);
        self.insert_cursor()
    }

    pub fn move_cursor_right(&mut self) -> String {
        let cursor_moved_right = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_right);
        self.insert_cursor()
    }

    pub fn enter_char(&mut self, new_char: char) -> String {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
        return self.insert_cursor();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) -> String {
        let is_not_cursor_leftmost = self.cursor_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();

        }
        
        return self.insert_cursor();
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_index = 0;
    }

    pub fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }

    pub fn insert_cursor(&self) -> String {
        let mut display_string = self.input.clone();
        let cursor_position = self.byte_index();
        display_string.insert(cursor_position, '|');
        display_string
    }
    
    pub fn remove_cursor(&mut self) -> String {
        // Remove the cursor by replacing it with an empty string
        self.input.retain(|c| c != '|');
        // Return the modified text
        self.input.clone()
    }
    
}