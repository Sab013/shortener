FROM rust:1.84.1 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/url_shortener /app/url_shortener

EXPOSE 8080

CMD ["/app/url_shortener"]
