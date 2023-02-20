use std::sync::Arc;
use user_register::infrastructure::user::repository::UserRepository;
use user_register::domain::user;
use user_register::configuration;
use user_register::runner;
use std::net::TcpListener;
use user_register::telemetry;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get_configuration().expect("Failed to read configuration.");
    let subscriber = telemetry::get_subscriber("user_register".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let user_store = Arc::new(UserRepository::new());
    let user_service = user::service::Service::new(user_store);
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    runner::run_http_server(listener, user_service).expect("unable to bind the tcp listener").await?;
    Ok(())
}
