# Build Stage
FROM rust:1.89-slim AS builder

WORKDIR /usr/src/recon-agent
COPY . .

# Install pkg-config and openssl (required for reqwest and sqlx)
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev

RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/recon-agent/target/release/recon-agent /app/recon-agent
COPY --from=builder /usr/src/recon-agent/frontend /app/frontend
COPY --from=builder /usr/src/recon-agent/migrations /app/migrations
COPY --from=builder /usr/src/recon-agent/.env.example /app/.env

# Create unprivileged user
RUN useradd -m -s /bin/bash appuser && \
    chown -R appuser:appuser /app
USER appuser

EXPOSE 8080

CMD ["./recon-agent"]
