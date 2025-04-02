use std::collections::HashMap;
pub mod create_relationship;
pub mod get_relationships;

pub struct QueryNode {
    typ: String,
    id: String,
    properties: HashMap<String, String>,
}

impl QueryNode {
    pub fn new(key: String, properties: HashMap<String, String>) -> Self {
        // split key on :: into typ and id
        let mut parts = key.split("::");
        let typ = parts.next().unwrap_or("").to_string();
        let id = parts.next().unwrap_or("").to_string();
        Self {
            typ,
            id,
            properties,
        }
    }
}
