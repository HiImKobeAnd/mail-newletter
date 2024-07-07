use mail_newsletter::configuration::get_configuration;
use mail_newsletter::email_client::EmailClient;
use mail_newsletter::startup::run;
use mail_newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("mail_newsletter".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration.email_client.sender().expect("Invalid sender email address.");
    let email_client = EmailClient::new(configuration.email_client.base_url, sender_email);

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener =
        TcpListener::bind(address.clone()).unwrap_or_else(|_| panic!("Can't bind to {}", address));
    run(listener, connection_pool, email_client)?.await
}
