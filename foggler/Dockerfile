FROM rust:1.85 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN cargo build --release

FROM debian:12.10-slim

WORKDIR /app

COPY --from=builder /app/target/release/foggler /app/foggler
RUN chmod +x /app/foggler

CMD ["/app/foggler", "--servers", "/app/servers.toml", "--timeout", "3", "--port", "443", "--wait", "60", "--logs", "logs"]
