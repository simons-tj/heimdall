# Build stage
FROM rust:1.85-slim-bullseye AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    perl \
    perl-modules-5.32 \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:stable-slim

WORKDIR /usr/local/bin

# Copy the built binary from builder
COPY --from=builder /usr/src/app/target/release/heimdall .

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables
ENV RUST_LOG=info

# Expose the port the app runs on
EXPOSE 3000

# Run the application
CMD ["./heimdall"]
