pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::node::Node;
    use crate::graph::graph_items::edge::Edge;
    use std::iter::FromIterator;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
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
            unimplemented!("with_edges")
        }

        pub fn with_attrs(&self, attributes: &[(&str, &str)]) -> Self {
            unimplemented!("with_attrs")
        }

        pub fn get_node(&self, node_name: &str) -> Option<Node> {
            unimplemented!("get_node")
        }
    }

    pub mod graph_items {

        pub mod node {
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct Node {}

            impl Node {
                pub fn new(node_name: &str) -> Self {
                    Node {
                    }
                }

                pub fn with_attrs(&self, attributes: &[(&str, &str)]) -> Self {
                    unimplemented!("with_attrs")
                }

                pub fn get_attr(&self, attribute_name: &str) -> Option<&str> {
                    unimplemented!("with_attrs")
                }
            }
        }

        pub mod edge {
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub struct Edge {}

            impl Edge {
                pub fn new(node_from_name: &str, node_to_name: &str) -> Self {
                    unimplemented!("new node");
                }

                pub fn with_attrs(&self, attributes: &[(&str, &str)]) -> Self {
                    unimplemented!("with_attrs")
                }
            }
        }
    }
}
