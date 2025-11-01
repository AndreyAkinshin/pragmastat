# Dockerfile for Pragmastat R implementation
# Note: rocker/verse doesn't support ARM64. On Apple Silicon, this runs via x86_64 emulation (Rosetta 2)
# The platform is specified in docker-compose.yml as linux/amd64
FROM rocker/verse:latest

# Update TeX Live packages
RUN tlmgr update --self && \
    tlmgr install amsfonts

# Install R package dependencies (these are system-level R packages, not project-specific)
RUN R -e "install.packages(c('devtools', 'testthat', 'styler', 'lintr'), repos='https://cloud.r-project.org')"

# Set working directory to r
WORKDIR /workspace/r

# Note: Source code and entrypoint script will be mounted via volume at runtime
# Entrypoint is configured in docker-compose.yml to reference the mounted script

# Default command
CMD ["/bin/bash"]

