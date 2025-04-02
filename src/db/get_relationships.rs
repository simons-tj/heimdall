use crate::db::QueryNode;
use anyhow::Error;
use neo4rs::{query, Graph};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GetConnectionDetails {
    pub from: QueryNode,
    pub relationship: String,
}

#[derive(serde::Serialize)]
pub struct Relationship {
    pub id: String,
    pub properties: HashMap<String, String>,
}

#[derive(serde::Serialize)]
pub struct RelationshipDetails {
    pub relationships: Vec<Relationship>,
}

#[derive(Deserialize)]

struct RelationFromGraph {
    properties: RelationProperties,
}

/**
 * Get relationship details from the graph.
 * This is 90% working, I just need to fix mapping of graph -> rust types. Hackahton is over so I'm done
 */
pub async fn get_relationships(
    graph: &Graph,
    connection: GetConnectionDetails,
) -> Result<RelationshipDetails, Error> {
    /* neo4rs doesn't support bolt v5 yet, so we have to use string format to make this dynamic */
    /* The connection params should be validated to avoid injection attacks */
    let query_str = format!(
        r#"
        MATCH (u: {})-[r:{}]->(target)
        WHERE u.id = $from_id
        RETURN properties(r) AS relationshipProperties, target
      "#,
        connection.from.typ, connection.relationship
    );

    let mut result = graph
        .execute(query(&query_str).param("from_id", connection.from.id))
        .await
        .unwrap();
    let mut relationships: Vec<Relationship> = vec![];
    while let Ok(Some(row)) = result.next().await {
        println!("row: {:?}", row);
        let relationship: HashMap<String, String> = row.get("relationshipProperties").unwrap();
        let target_node: RelationFromGraph = row.get("target").unwrap();

        relationships.push(Relationship {
            id: target_node.properties.get("id"),
            properties: relationship,
        });
    }

    Ok(RelationshipDetails { relationships })
}
