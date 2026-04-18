FROM rust:1.88-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

COPY src ./src
COPY migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/usestakly-backend /usr/local/bin/usestakly-backend
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
COPY migrations ./migrations

RUN sed -i 's/\r$//' /usr/local/bin/docker-entrypoint.sh \
    && chmod +x /usr/local/bin/docker-entrypoint.sh

ENV APP_HOST=0.0.0.0
ENV APP_PORT=4000

EXPOSE 4000

CMD ["/usr/local/bin/docker-entrypoint.sh"]
