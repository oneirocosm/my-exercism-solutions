use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node<'a> {
    pub name: &'a str,
    pub attrs: HashMap<&'a str, &'a str>,
}

impl<'a> Node<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
        for &(key, value) in attrs {
            self.attrs.insert(key, value);
        }
        self
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key).map(|val| *val)
    }
}
