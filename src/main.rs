use mail_newletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}