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

    pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
        self.attrs.extend(
            attributes
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }
}
