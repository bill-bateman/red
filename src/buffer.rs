use std::io::Error;

pub struct Buffer {
    pub text: Vec<String>,
}

impl Buffer {
    pub fn default() -> Self {
        Buffer{ text: vec![] }
    }

    pub fn new(filename: &str) -> Result<Self, Error> {
        let file_contents = std::fs::read_to_string(filename)?;
        let text = file_contents.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        Ok(Buffer{ text })
    }

    pub fn is_empty(&self) -> bool {
        return self.text.len() == 0
    }
}