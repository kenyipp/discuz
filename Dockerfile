FROM rust:1.65.0

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release --bin discuz-server
