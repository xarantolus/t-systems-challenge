FROM rust:latest AS rust-builder
WORKDIR /app/rust
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . ./
RUN cargo build --release

WORKDIR /app/rust

RUN cp -f /app/rust/target/release/t-systems-challenge /app/t-systems-challenge

WORKDIR /app/rust/target/release
ENTRYPOINT ["/app/rust/target/release/t-systems-challenge"]
