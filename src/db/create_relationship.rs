use crate::db::QueryNode;
use neo4rs::{query, Graph};
use std::collections::HashMap;

pub struct CreateConnectionDetails {
    pub from: QueryNode,
    pub to: QueryNode,
    pub relationship: String,
    pub properties: HashMap<String, String>,
}

/**
 * Create a relationship between two nodes in the graph.
 * The nodes will be created if they do not exist.
 */
pub async fn create_graph_relationship(graph: &Graph, connection: CreateConnectionDetails) {
    /* neo4rs doesn't support bolt v5 yet, so we have to use string format to make this dynamic */
    let query_str = format!(
        r#"
      MERGE (f:{} {{id: $from_id}})
      MERGE (t:{} {{id: $to_id}})
      MERGE (f)-[rf:{}]->(t)
      MERGE (t)-[rt:{}]->(f)
      SET rf += $properties
      SET rt += $properties
      "#,
        connection.from.typ, connection.to.typ, connection.relationship, connection.relationship
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
