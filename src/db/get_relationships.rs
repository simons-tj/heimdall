use crate::db::QueryNode;
use neo4rs::{query, Graph};
use std::collections::HashMap;

pub struct GetConnectionDetails {
    pub from: QueryNode,
    pub relationship: String,
}

pub struct Relationship {
    pub id: String,
    pub properties: HashMap<String, String>,
}

pub struct RelationshipDetails {
    pub relationships: Vec<Relationship>,
}

/**
 * Create a relationship between two nodes in the graph.
 * The nodes will be created if they do not exist.
 */
pub async fn get_relationships(graph: &Graph, connection: GetConnectionDetails) {
    /* neo4rs doesn't support bolt v5 yet, so we have to use string format to make this dynamic */
    let query_str = format!(
        r#"
        MATCH (f:{} {{id: $from_id}})-[r:{}]->(t)
        RETURN r, t
      "#,
        connection.from.typ, connection.relationship
    );

    graph
        .run(
            query(&query_str)
                .param("from_id", connection.from.id)
                .param("to_id", connection.to.id)
                .param("properties", connection.properties),
        )
        .await
        .unwrap();
}
