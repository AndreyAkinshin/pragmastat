# Dockerfile for Pragmastat web/Astro build
FROM node:22-alpine

# Install bash and pnpm
RUN apk add --no-cache bash
RUN corepack enable && corepack prepare pnpm@latest --activate

# Set working directory to web
WORKDIR /workspace/web

# Set pnpm store to a writable directory for non-root users
ENV PNPM_HOME=/tmp/.pnpm
ENV PATH="$PNPM_HOME:$PATH"

# Copy only package files
COPY web/package.json web/pnpm-lock.yaml ./

# Install dependencies (cached in Docker layer)
RUN pnpm install --frozen-lockfile

# Note: Source code and generated content will be mounted via volume at runtime
# Run `mise run web:gen` on host before building to generate MDX content from Typst

# Default command
CMD ["/bin/bash"]
