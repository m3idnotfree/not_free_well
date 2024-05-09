use axum::{http::StatusCode, response::IntoResponse};

pub async fn index() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found".to_string())
}
