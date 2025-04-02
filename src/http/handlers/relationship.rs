use std::collections::HashMap;
use std::sync::Arc;

use crate::db::{create_relationship::*, get_relationships::*, QueryNode};
use crate::http::{AppState, Error, Result};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

#[derive(serde::Deserialize)]
pub struct CreateRelationshipRequest {
    pub from: String,
    pub to: String,
    pub relationship: String,
    pub properties: HashMap<String, String>,
}

impl CreateRelationshipRequest {
    fn to_query_nodes(&self) -> CreateConnectionDetails {
        let from_node = QueryNode::fromRequest(&self.from);
        let to_node = QueryNode::fromRequest(&self.to);

        CreateConnectionDetails {
            from: from_node,
            to: to_node,
            relationship: self.relationship.clone(),
            properties: self.properties.clone(),
        }
    }
}
#[derive(serde::Serialize)]
pub struct CreateRelationshipResponse {
    pub status: String,
    pub message: String,
}

impl IntoResponse for CreateRelationshipResponse {
    fn into_response(self) -> axum::response::Response {
        let body = json!({
            "status": self.status,
            "message": self.message,
        });
        let body = serde_json::to_string(&body).unwrap();
        (axum::http::StatusCode::OK, body).into_response()
    }
}

pub async fn create_relationship(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateRelationshipRequest>,
) -> Result<CreateRelationshipResponse, Error> {
    let connection = payload.to_query_nodes();
    create_graph_relationship(&state.graph, connection).await;

    Ok(CreateRelationshipResponse {
        status: "success".to_string(),
        message: "Relationship created successfully".to_string(),
    })
}

#[derive(serde::Deserialize)]
pub struct GetRelationshipRequest {
    pub from: String,
    pub relationship: String,
}

impl GetRelationshipRequest {
    fn to_query_nodes(&self) -> GetConnectionDetails {
        let from_node = QueryNode::fromRequest(&self.from);
        GetConnectionDetails {
            from: from_node,
            relationship: self.relationship.clone(),
        }
    }
}

impl IntoResponse for RelationshipDetails {
    fn into_response(self) -> axum::response::Response {
        let body = json!(self);
        let body = serde_json::to_string(&body).unwrap();
        (axum::http::StatusCode::OK, body).into_response()
    }
}

pub async fn get_relationships_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<GetRelationshipRequest>,
) -> Result<Json<RelationshipDetails>, Error> {
    let connection = payload.to_query_nodes();
    let result = get_relationships(&state.graph, connection).await?;

    Ok(Json(result))
}
