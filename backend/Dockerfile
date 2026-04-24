FROM rust:1.91-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY scoring ./scoring

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/usestakly-backend /usr/local/bin/usestakly-backend
COPY migrations ./migrations

ENV APP_HOST=0.0.0.0
ENV APP_PORT=4000

EXPOSE 4000

HEALTHCHECK --interval=10s --timeout=5s --start-period=15s --retries=5 \
    CMD curl -fsS http://localhost:4000/health || exit 1

CMD ["usestakly-backend"]
