# Dockerfile for Pragmastat Rust implementation
FROM rust:1.90-slim

# Install additional components
RUN rustup component add rustfmt clippy

# Set working directory to rs (not rs/pragmastat, for build.sh access)
WORKDIR /workspace/rs

# Set Cargo home to a writable location for non-root users
ENV CARGO_HOME=/tmp/.cargo

# Copy only dependency manifest files
COPY rs/pragmastat/Cargo.toml rs/pragmastat/Cargo.lock pragmastat/

# Fetch dependencies (cached in Docker layer)
RUN cd pragmastat && cargo fetch || true

# Note: Source code will be mounted via volume at runtime
# This ensures dependencies are cached but source is always fresh

# Default command
CMD ["/bin/bash"]

