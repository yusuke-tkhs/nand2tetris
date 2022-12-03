#[derive(Debug)]
pub struct FileContext {
    file_name: String,
    counter: usize,
}

impl FileContext {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
            counter: 0,
        }
    }
    pub fn file_name(&self) -> String {
        self.file_name.clone()
    }
    pub fn publish_unique_key(&mut self) -> String {
        let key = format!("{}_{}", self.file_name, self.counter);
        self.counter += 1;
        key
    }
}
