pub mod graph {
    pub use super::Graph;

    pub mod graph_items {

        pub mod edge {
            pub use super::super::super::Edge;
        }

        pub mod node {
            pub use super::super::super::Node;
        }
    }
}


use std::collections::HashMap;

macro_rules! attrs {
    () => {
        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            Self {
                attrs: attrs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
                ..self
            }
        }

        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|v| v.as_str())
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Node {
    pub node: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(node: &str) -> Self {
        Node {
            node: node.to_string(),
            attrs: HashMap::new(),
        }
    }

    attrs!();
}


#[derive(PartialEq, Clone, Debug)]
pub struct Edge {
    pub right: String, 
    pub left: String,
    pub attrs: HashMap<String, String>,
}


impl Edge {
    pub fn new(right: &str, left: &str) -> Self {
        Edge {
            right: right.to_string(),
            left: left.to_string(),
            attrs: HashMap::new(),
        }
    }
    
    attrs!();
}




#[derive(PartialEq, Clone, Debug, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}


impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(self, nodes: &[Node]) -> Self {
        Self {
            nodes: nodes.to_vec(),
            ..self
        }
    }

    pub fn with_edges(self, edges: &[Edge]) -> Self {
        Self {
            edges: edges.to_vec(),
            ..self
        }
    }

    pub fn get_node(&self, node: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.node == node)
    }
    
    attrs!();
}




