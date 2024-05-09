use reqwest::StatusCode;
mod common;

#[tokio::test]
async fn route_index() {
    let app = common::spawn_app().await;

    let response = reqwest::get(&format!("http://{}/", &app)).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body = response.text().await.unwrap();
    assert_eq!(body, "Not Found");
}
