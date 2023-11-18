FROM rust:1.74

WORKDIR /app

COPY ./src ./src
COPY Cargo.toml .
COPY Cargo.lock .

VOLUME [ "/app/target"]

CMD cargo run