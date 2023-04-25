use std::net::TcpListener;

use sqlx::{Connection, PgConnection, PgPool};
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::test]
async fn test_health_check() {
    let app: TestApp = start_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn test_subscribe_return_200_for_valid_form_data() {
    // setup
    let app: TestApp = start_app().await;
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    let client = reqwest::Client::new();
    let body = "name=nahum%20sa&email=nahumsa%40email.com";

    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    // Verify if the data was persisted in the database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions;",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "nahumsa@email.com");
    assert_eq!(saved.name, "nahum sa");
}

#[tokio::test]
async fn test_subscribe_return_400_when_data_is_missing() {
    let app: TestApp = start_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=nahum%20sa", "missing the email"),
        ("email=nahumsa@email.com", "missing the name"),
        ("", "missing email and name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
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
    }
}

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn start_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind a random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
    .await
    .expect("Failed to connect to Postgres.");

    let server = run(listener, connection_pool.clone())
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp{
        address,
        db_pool: connection_pool,
    }

}
