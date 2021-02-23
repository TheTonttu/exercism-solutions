pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(&self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.to_vec(),
                edges: self.edges.to_vec(),
                attrs: self.attrs.clone(),
            }
        }

        pub fn with_edges(&self, edges: &[Edge]) -> Self {
            Self {
                nodes: self.nodes.to_vec(),
                edges: edges.to_vec(),
                attrs: self.attrs.clone(),
            }
        }

        pub fn with_attrs(&self, attributes: &[(&str, &str)]) -> Self {
            Self {
                nodes: self.nodes.to_vec(),
                edges: self.edges.to_vec(),
                attrs: attributes
                    .iter()
                    .map(|(a, b)| (a.to_string(), b.to_string()))
                    .collect(),
            }
        }

        pub fn get_node(&self, node_name: &str) -> Option<Node> {
            self.nodes.iter().find(|n| n.name == node_name).cloned()
        }
    }

    pub mod graph_items {

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(node_name: &str) -> Self {
                    Self {
                        name: node_name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attributes: &[(&str, &str)]) -> Self {
                    Self {
                        name: self.name.clone(),
                        attrs: attributes
                            .iter()
                            .map(|(a, b)| (a.to_string(), b.to_string()))
                            .collect(),
                    }
                }

                pub fn get_attr(&self, attribute_name: &str) -> Option<&str> {
                    // FIXME: Dios mio...
                    self.attrs.get(attribute_name).map(|attr| &attr[..])
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                node_from: String,
                node_to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(node_from_name: &str, node_to_name: &str) -> Self {
                    Self {
                        node_from: node_from_name.to_string(),
                        node_to: node_to_name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attributes: &[(&str, &str)]) -> Self {
                    Self {
                        node_from: self.node_from.clone(),
                        node_to: self.node_to.clone(),
                        attrs: attributes
                            .iter()
                            .map(|(a, b)| (a.to_string(), b.to_string()))
                            .collect(),
                    }
                }
            }
        }
    }
}
