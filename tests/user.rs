use actix_web::dev::Server;
// Since this is in tests folder, it will apply blackbox test to user_register application
use once_cell::sync::Lazy;
use user_register::runner;
use std::net::TcpListener;
use std::sync::Arc;
use user_register::telemetry;
use user_register::infrastructure::user::repository::UserRepository;
use user_register::domain::user;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use reqwest::StatusCode;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        telemetry::init_subscriber(subscriber);
    } else {
        let subscriber = telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        telemetry::init_subscriber(subscriber);
    };
});

pub struct TestApp {
    pub address: String,
}

async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    //let mut configuration = configuration::get_configuration().expect("Failed to read configuration.");    
    let user_store = Arc::new(UserRepository::new());
    let user_service = user::service::Service::new(user_store);
    let server: Server = runner::run_http_server(listener, user_service).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
    }
}


#[derive(Serialize)]
struct UserCreateRequest {
    name: String,
}

#[derive(Deserialize)]
struct UserCreatedResponse {
    name: String,
    id: String
}

#[tokio::test]
async fn create_user_with_200() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let user = UserCreateRequest{name: "some name".into()};

    // Act
    let response = client
        // Use the returned application address
        .post(&format!("{}/user", &app.address))
        .json(&user)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status() == StatusCode::CREATED);
    assert_ne!(Some(0), response.content_length());
    let user_created_response: UserCreatedResponse = response.json().await.expect("unable to get user created response");
    assert_eq!(user.name, user_created_response.name);
    assert!(Uuid::parse_str(&user_created_response.id).is_ok());
}