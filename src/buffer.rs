
pub struct Buffer {
    pub text: Vec<String>,
}

impl Buffer {
    pub fn default() -> Self {
        Buffer{ text: vec!["Hello, world!".to_string()] }
    }
}