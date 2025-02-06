FROM rust:1.84.1 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/shortener /app/shortener

EXPOSE 8080

CMD ["/app/shortener"]
