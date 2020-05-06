use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge<'a> {
    nodes: (&'a str, &'a str),
    attrs: HashMap<String, String>,
}

impl<'a> Edge<'a> {
    pub fn new(node0: &'a str, node1: &'a str) -> Self {
        Self {
            nodes: (node0, node1),
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
