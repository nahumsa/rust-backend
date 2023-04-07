#[tokio::test]
async fn test_health_check() {
    start_app();

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.01:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn start_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);

}
