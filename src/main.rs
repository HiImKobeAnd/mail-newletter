use mail_newletter::configuration::get_configuration;
use mail_newletter::startup::run;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener =
        TcpListener::bind(address.clone()).expect(format!("Can't bind to {}", address).as_str());
    run(listener, connection)?.await
}
