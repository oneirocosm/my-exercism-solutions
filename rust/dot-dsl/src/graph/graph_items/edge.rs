use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    nodes: (String, String),
    attrs: HashMap<String, String>,
}

impl Edge {
    pub fn new(node0: &str, node1: &str) -> Self {
        Self {
            nodes: (node0.into(), node1.into()),
            attrs: HashMap::new(),
        }
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for &(key, value) in attrs {
            self.attrs.insert(key.into(), value.into());
        }
        self
    }
}
