pub struct TextCache {
    pub text: Vec<String>,
    pub history: Vec<Vec<String>>
}

impl Default for TextCache {
    fn default() -> Self {
        let mut text = Vec::new();
        let mut history = Vec::new();

        Self { text, history }
    }
}

