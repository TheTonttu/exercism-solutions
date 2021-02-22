pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;
    use std::iter::FromIterator;

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
                attrs: HashMap::from_iter(
                    attributes
                        .iter()
                        .map(|(a, b)| (a.to_string(), b.to_string())),
                ),
            }
        }

        pub fn get_node(&self, node_name: &str) -> Option<Node> {
            unimplemented!("get_node")
        }
    }

    pub mod graph_items {

        pub mod node {
            use std::collections::HashMap;
            use std::iter::FromIterator;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(node_name: &str) -> Self {
                    Self {
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attributes: &[(&str, &str)]) -> Self {
                    Self {
                        attrs: HashMap::from_iter(
                            attributes
                                .iter()
                                .map(|(a, b)| (a.to_string(), b.to_string())),
                        ),
                    }
                }

                pub fn get_attr(&self, attribute_name: &str) -> Option<&str> {
                    unimplemented!("with_attrs")
                }
            }
        }

        pub mod edge {
            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                node_from: String,
                node_to: String,
            }

            impl Edge {
                pub fn new(node_from_name: &str, node_to_name: &str) -> Self {
                    Self {
                        node_from: node_from_name.to_string(),
                        node_to: node_to_name.to_string(),
                    }
                }

                pub fn with_attrs(&self, attributes: &[(&str, &str)]) -> Self {
                    unimplemented!("with_attrs")
                }
            }
        }
    }
}
