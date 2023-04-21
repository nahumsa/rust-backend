use std::net::TcpListener;

#[tokio::test]
async fn test_health_check() {
    let address: String = start_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn test_subscribe_return_200_for_valid_form_data() {
    let address: String = start_app();

    let client = reqwest::Client::new();

    let body = "name=nahum%20sa&email=nahumsa%40email.com";

    let response = client
        .post(&format!("{}/subscription", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn test_subscribe_return_400_when_data_is_missing() {
    let address: String = start_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=nahum%20sa", "missing the email"),
        ("email=nahumsa@email.com", "missing the name"),
        ("", "missing email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscription", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            error_message
        );
    };
}

fn start_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port");

    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.1:{}", port)
}
