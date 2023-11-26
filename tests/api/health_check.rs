use crate::helpers::launch_app;

#[tokio::test]
async fn health_check_works() {
    let app = launch_app().await;
    let client = reqwest::Client::new();
    let url = &format!("{}/health_check", app.address);
    let path = url.as_str();
    let response = client
        .get(path)
        .send()
        .await
        .expect("Failed to ");

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(Some(0), response.content_length());
}