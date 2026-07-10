# Build stage
FROM rust:1.88-slim AS builder

WORKDIR /usr/src/app
COPY . .

# Build the release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install ca-certificates (useful if you ever need outbound HTTPS requests)
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
ENV DATA_DIR=/app/data
VOLUME ["/app/data"]

# Copy the compiled binary and static files from the builder stage
COPY --from=builder /usr/src/app/target/release/nodemates-scanner /app/nodemates-scanner
COPY --from=builder /usr/src/app/static /app/static

# Expose the web service port
EXPOSE 3000

# Run the binary
ENTRYPOINT ["/app/nodemates-scanner"]
