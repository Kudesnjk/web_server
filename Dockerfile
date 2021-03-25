FROM rust:1.50 as builder

WORKDIR web_server
COPY . .

RUN cargo build --release --bin web_server

EXPOSE 80

CMD ["./target/release/web_server", "0.0.0.0", "80"]