FROM rust:1.93.0-nightly

WORKDIR /pixl8-media

RUN cargo install diesel_cli --no-default-features --features=postgres
