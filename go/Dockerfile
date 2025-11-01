# Dockerfile for Pragmastat Go implementation
FROM golang:1.21-alpine

# Install git (required for some Go operations)
RUN apk add --no-cache git bash

# Set working directory to go
WORKDIR /workspace/go

# Set Go cache directories to writable locations for non-root users
ENV GOCACHE=/tmp/.cache/go-build \
    GOMODCACHE=/tmp/go/pkg/mod

# Copy only Go module files
COPY go/go.mod ./

# Download dependencies (cached in Docker layer)
# Note: go.sum will be generated if it doesn't exist
RUN go mod download

# Note: Source code will be mounted via volume at runtime
# This ensures dependencies are cached but source is always fresh

# Default command
CMD ["/bin/bash"]

