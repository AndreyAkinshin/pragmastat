# Dockerfile for Pragmastat PDF generation
# Note: pandoc/latex doesn't support ARM64. On Apple Silicon, this runs via x86_64 emulation (Rosetta 2)
# The platform is specified in docker-compose.yml as linux/amd64
FROM pandoc/latex:3.7

# Install bash (needed for build.sh scripts)
RUN apk add --no-cache bash

# Update TeX Live and install required packages
RUN tlmgr update --self && \
    tlmgr install \
    pgf \
    caption \
    textpos \
    mdframed \
    zref \
    needspace \
    csquotes \
    multirow \
    wrapfig \
    colortbl \
    pdflscape \
    tabu \
    varwidth \
    threeparttable \
    threeparttablex \
    environ \
    trimspaces \
    ulem \
    makecell \
    biblatex \
    logreq \
    biber \
    fvextra \
    upquote \
    lineno

# Set working directory to pdf
WORKDIR /workspace/pdf

# Note: No dependencies to install, source files will be mounted via volume at runtime

# Default command
CMD ["/bin/sh"]

