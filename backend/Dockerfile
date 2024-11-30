FROM rust:bullseye AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
RUN cargo build --release --bin shor || true

COPY . .
RUN cargo build --release --bin shor

FROM alpine:latest
COPY --from=builder ./target/release/shor /usr/local/bin/shor
RUN ls
EXPOSE 8080
CMD ["shor"]
