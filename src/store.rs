use std::collections::HashMap;

pub struct Store {
    pub hash: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            hash: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.hash.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.hash.get(key)
    }

    pub fn len(&self) -> usize {
        self.hash.len()
    }
}
