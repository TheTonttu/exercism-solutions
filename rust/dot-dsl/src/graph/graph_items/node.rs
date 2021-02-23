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

    pub fn with_attrs(mut self, attributes: &[(&str, &str)]) -> Self {
        self.attrs.extend(
            attributes
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string())),
        );
        self
    }

    pub fn get_attr(&self, attribute_name: &str) -> Option<&str> {
        self.attrs.get(attribute_name).map(|attr| attr.as_str())
    }
}
