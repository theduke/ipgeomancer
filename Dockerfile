# Stage 1: build the ipgeom CLI
FROM rust AS builder
WORKDIR /app
COPY . .
# RUN apt-get update && apt-get install -y --no-install-recommends \
#     ca-certificates \
#     libssl-dev \
#     pkg-config
RUN cargo build --release -p ipgeom_cli

# Stage 2: runtime image with required libraries
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    openssl \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/ipgeom /usr/local/bin/ipgeom

EXPOSE 8080
ENV IPGEOMANCER_LISTEN="0.0.0.0:8080"
ENV RUST_LOG="info"

ENTRYPOINT ["/usr/local/bin/ipgeom"]
CMD ["server"]
