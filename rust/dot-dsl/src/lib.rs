pub mod graph {

    pub mod graph_items;

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
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes.extend(nodes.iter().cloned());
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }

        pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
            self.attrs.extend(
                attributes
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string())),
            );
            self
        }

        pub fn get_node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|n| n.name == node_name)
        }
    }

    impl Default for Graph {
        fn default() -> Self {
            Self::new()
        }
    }
}
