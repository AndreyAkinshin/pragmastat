# Dockerfile for Pragmastat TypeScript implementation
FROM node:22-alpine

# Install bash for build scripts
RUN apk add --no-cache bash

# Set working directory to ts
WORKDIR /workspace/ts

# Non-interactive install (pnpm 11 skips the modules-purge confirmation)
ENV CI=true

# Copy only package files (the project uses pnpm, not npm)
COPY ts/package.json ts/pnpm-lock.yaml ./

# Install dependencies (cached in Docker layer)
RUN npm install -g pnpm@11 && pnpm install --frozen-lockfile

# Note: Source code will be mounted via volume at runtime
# This ensures dependencies are cached but source is always fresh

# Default command
CMD ["/bin/bash"]

