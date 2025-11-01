# Dockerfile for Pragmastat image generation
FROM python:3.10-slim

# Set working directory to img
WORKDIR /workspace/img

# Copy only requirements file
COPY img/requirements.txt ./

# Install Python dependencies (cached in Docker layer)
RUN pip install --no-cache-dir -r requirements.txt

# Note: Source code will be mounted via volume at runtime
# This ensures dependencies are cached but source is always fresh

# Default command
CMD ["/bin/bash"]

