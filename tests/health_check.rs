use reqwest::StatusCode;
mod common;

#[tokio::test]
async fn route_health_check() {
    let app = common::spawn_app().await;

    let response = reqwest::get(&format!("http://{}/health_check", &app))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = response.text().await.unwrap();
    assert_eq!(body, r#"{"status":"OK"}"#);
}
