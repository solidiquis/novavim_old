struct TextCache {
    text: Vec<String>,
    history: Vec<Vec<String>>
}

impl Default for TextCache {
    fn default() -> Self {
        let mut text = Vec::new();
        let mut history = Vec::new();

        Self { text, history }
    }
}

impl TextCache {
    pub fn edit_line(&mut self, line_no: u8, col: u8, txt: &str) {
        if (self.text.len() as u8) < line_no {
            self.text.push("".to_string())
        };
        
        self.text[(line_no - 1) as usize].push_str(txt)
    }
}
