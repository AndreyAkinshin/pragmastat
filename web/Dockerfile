# Dockerfile for Pragmastat web/Hugo build
# Using Debian instead of Alpine to avoid glibc/musl compatibility issues with Hugo/Tailwind
FROM debian:12-slim

# Install bash, curl, and other required tools
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    bash \
    curl \
    tar \
    gzip \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory to web
WORKDIR /workspace/web

# Note: Hugo and Tailwind are downloaded by build scripts
# Source files will be mounted via volume at runtime

# Default command
CMD ["/bin/bash"]

