pub mod graph_items;

use graph_items::{edge::Edge, node::Node};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.into();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.into();
        self
    }

    pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
        for &(key, value) in attrs {
            self.attrs.insert(key.into(), value.into());
        }
        self
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.name == name)
    }

    pub fn get_attr(&self, key: &str) -> Option<&str> {
        self.attrs.get(key.into()).map(|val| val.as_str())
    }
}
