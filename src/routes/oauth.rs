use axum::{extract::Query, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthReqest {
    pub code: String,
    pub state: String,
}

pub async fn oauth(Query(query): Query<AuthReqest>) -> impl IntoResponse {
    (StatusCode::OK, serde_json::to_string(&query).unwrap())
}
