FROM rust:1.79.0

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
RUN SQLX_OFFLINE true
RUN cargo build --release
ENTRYPOINT [ "./target/release/mail_newletter" ]