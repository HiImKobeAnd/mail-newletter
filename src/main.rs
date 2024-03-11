use mail_newletter::configuration::get_configuration;
use mail_newletter::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener =
        TcpListener::bind(address.clone()).expect(format!("Can't bind to {}", address).as_str());
    run(listener)?.await
}
