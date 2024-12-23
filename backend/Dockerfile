FROM rust:1.80.0-slim-bullseye AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./

RUN apt-get update && apt-get install -y curl
RUN curl --version
RUN mkdir src \
    && echo "fn main () {}" > src/main.rs \
    && cargo build --locked --release 

COPY . .
RUN cargo build --locked --release

FROM debian:bullseye-slim AS final
WORKDIR /app
EXPOSE 8080
COPY --from=builder /app/configuration configuration
COPY --from=builder /app/target/release/shor /usr/local/bin/shor

CMD shor --config "$CONFIG"
