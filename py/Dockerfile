# Dockerfile for Pragmastat Python implementation
FROM python:3.10-slim

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    gcc \
    g++ \
    make \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /workspace/py

# Install Python build tools (cached in Docker layer)
RUN pip install --no-cache-dir --upgrade pip && \
    pip install --no-cache-dir pytest build twine

# Note: Source code will be mounted via volume at runtime
# Package will be installed in editable mode by build scripts

# Default command
CMD ["/bin/bash"]

