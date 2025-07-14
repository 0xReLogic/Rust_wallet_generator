# --- Stage 1: Build Stage ---
# Use the official Rust image as a builder
FROM rust:1.78 as builder

# Create a new empty shell project
WORKDIR /usr/src/rust_wallet_gen
COPY . .

# Build the release binary
# This will cache dependencies
RUN cargo build --release --locked

# --- Stage 2: Final Stage ---
# Use a slim, secure base image
FROM debian:buster-slim

# Install necessary dependencies for GUI (if needed)
RUN apt-get update && apt-get install -y \
    libx11-6 \
    libxcb1 \
    libxrandr2 \
    libxinerama1 \
    libxcursor1 \
    libxcomposite1 \
    libxdamage1 \
    libxfixes3 \
    libxss1 \
    libxtst6 \
    libnss3 \
    libcups2 \
    libdrm2 \
    libxkbcommon0 \
    libatspi2.0-0 \
    libx11-xcb1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/rust_wallet_gen/target/release/rust_wallet_genenerator /usr/local/bin/rust_wallet_genenerator

# Create a non-root user for security
RUN useradd -m -u 1000 walletuser
USER walletuser

# Set the binary as the entrypoint
ENTRYPOINT ["rust_wallet_genenerator"] 