use std::net::TcpListener;

use mail_newletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Can't bind to port 8000");
    run(listener)?.await
}
