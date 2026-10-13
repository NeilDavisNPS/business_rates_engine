# =========================================================================
# Stage 1: Build the Svelte 5 Frontend
# =========================================================================
FROM node:24-slim AS frontend-builder
WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

COPY scripts ./scripts
COPY styles ./styles
COPY frontend ./frontend

RUN ./scripts/ensure-tailwind.sh && \
    ./bin/tailwindcss -i styles/input.css -o frontend/src/tailwind.css --minify

WORKDIR /app/frontend
RUN npm ci || npm install
RUN npm run build

# =========================================================================
# Stage 2: Build the Rust Backend Release Binary
# =========================================================================
FROM rust:1-slim-bookworm AS backend-builder
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
# Pre-build dependencies for caching
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && echo "" > src/lib.rs && cargo build --release && rm -rf src

COPY src ./src
COPY docs ./docs
COPY templates ./templates
COPY locales ./locales
COPY THIRD_PARTY_LICENSES.md ./
COPY --from=frontend-builder /app/frontend/dist ./frontend/dist

RUN touch src/main.rs src/lib.rs && cargo build --release

# =========================================================================
# Stage 3: Minimal Production Runtime
# =========================================================================
FROM debian:bookworm-slim AS runtime
WORKDIR /app

LABEL org.opencontainers.image.title="business_rates_engine" \
      org.opencontainers.image.description="Statutory Business Rates Engine (England, Wales, Scotland 1990-2027)" \
      org.opencontainers.image.licenses="MIT"

RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*

# Run as non-root unprivileged service account
RUN groupadd -g 1001 appgroup && useradd -u 1001 -g appgroup -s /bin/sh appuser

COPY --chown=appuser:appgroup LICENSE.md THIRD_PARTY_LICENSES.md /app/
COPY --chown=appuser:appgroup --from=backend-builder /app/target/release/business_rates_engine /app/business_rates_engine
COPY --chown=appuser:appgroup --from=frontend-builder /app/frontend/dist /app/frontend/dist

USER appuser

EXPOSE 8080
ENV PORT=8080
ENV RUST_LOG=info

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:8080/health || exit 1

ENTRYPOINT ["/app/business_rates_engine"]
