use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for &(key, value) in attrs {
            self.attrs.insert(key.into(), value.into());
        }
        self
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|val| val.as_str())
    }
}
