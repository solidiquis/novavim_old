pub struct TextCache {
    pub text: Vec<String>,
    pub history: Vec<Vec<String>>
}

impl Default for TextCache {
    fn default() -> Self {
        let mut text = Vec::new();
        let history = Vec::new();

        text.push("".to_string());

        Self { text, history }
    }
}

