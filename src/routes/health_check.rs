use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheck {
    pub status: String,
}

pub async fn health_check() -> Json<HealthCheck> {
    let result = HealthCheck {
        status: "OK".to_string(),
    };

    Json(result)
}
