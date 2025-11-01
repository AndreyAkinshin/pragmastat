# Dockerfile for Pragmastat TypeScript implementation
FROM node:20-alpine

# Install bash for build scripts
RUN apk add --no-cache bash

# Set working directory to ts
WORKDIR /workspace/ts

# Set npm cache to a writable directory for non-root users
ENV npm_config_cache=/tmp/.npm

# Copy only package files
COPY ts/package.json ts/package-lock.json ./

# Install dependencies (cached in Docker layer)
RUN npm ci

# Note: Source code will be mounted via volume at runtime
# This ensures dependencies are cached but source is always fresh

# Default command
CMD ["/bin/bash"]

