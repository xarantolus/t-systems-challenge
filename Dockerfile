FROM node:21 AS frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build -- --outDir dist

FROM rust:latest AS rust-builder
WORKDIR /app/rust
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . ./
RUN cargo build --release

FROM alpine:latest
RUN apk add --no-cache libc6-compat
WORKDIR /app

COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist
COPY --from=rust-builder /app/rust/target/release/t-systems-challenge /app/t-systems-challenge

ENTRYPOINT ["/app/t-systems-challenge"]
