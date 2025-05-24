use std::collections::HashMap;

/// RPSL object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    attributes: HashMap<String, Vec<String>>,
}

impl Object {
    /// Create a new RPSL object
    pub fn new() -> Self {
        Object {
            attributes: HashMap::new(),
        }
    }

    /// Add an attribute to the object
    pub fn add(&mut self, key: String, value: String) {
        self.attributes
            .entry(key)
            .or_insert_with(Vec::new)
            .push(value);
    }

    pub fn get(&self, key: &str) -> Option<&[String]> {
        self.attributes.get(key).map(|v| v.as_slice())
    }

    /// Get a reference to the underlying attribute map
    pub fn attributes(&self) -> &HashMap<String, Vec<String>> {
        &self.attributes
    }

    /// Consume the object and return the underlying attribute map
    pub fn into_attributes(self) -> HashMap<String, Vec<String>> {
        self.attributes
    }

    /// Create an object from an attribute map
    pub fn from_attributes(map: HashMap<String, Vec<String>>) -> Self {
        Object { attributes: map }
    }
}
