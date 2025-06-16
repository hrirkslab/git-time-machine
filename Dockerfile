# Use the official Rust image as the build environment
FROM rust:1.75 as builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use a minimal runtime image
FROM debian:bookworm-slim

# Install git and other dependencies
RUN apt-get update && apt-get install -y \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false git-time-machine

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/git-time-machine /usr/local/bin/git-time-machine

# Set ownership and permissions
RUN chown root:root /usr/local/bin/git-time-machine && \
    chmod 755 /usr/local/bin/git-time-machine

# Create a directory for the workspace
WORKDIR /workspace

# Switch to non-root user
USER git-time-machine

# Expose the port
EXPOSE 3000

# Run the application
CMD ["git-time-machine"]
