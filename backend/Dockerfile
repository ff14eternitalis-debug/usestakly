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
COPY migrations ./migrations

ENV APP_HOST=0.0.0.0
ENV APP_PORT=4000

EXPOSE 4000

CMD ["sh", "-lc", "/usr/local/bin/usestakly-backend > /tmp/usestakly-backend.log 2>&1; code=$?; cat /tmp/usestakly-backend.log; if [ \"$code\" -ne 0 ]; then echo \"[usestakly-backend] exited with code $code; keeping container alive for 300 seconds for debugging\"; sleep 300; fi; exit \"$code\""]
