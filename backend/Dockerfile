FROM rust:1.88-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
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

CMD ["sh", "-lc", "echo '[usestakly-backend] dependency check'; ldd /usr/local/bin/usestakly-backend || true; RUST_BACKTRACE=1 /usr/local/bin/usestakly-backend > /tmp/usestakly-backend.log 2>&1 & pid=$!; sleep 8; if ! kill -0 \"$pid\" 2>/dev/null; then wait \"$pid\"; code=$?; cat /tmp/usestakly-backend.log || true; echo \"[usestakly-backend] process exited early with code $code; keeping container alive for 300 seconds for debugging\"; sleep 300; exit \"$code\"; fi; cat /tmp/usestakly-backend.log || true; wait \"$pid\""]
