# Dockerfile for Pragmastat .NET implementation
FROM mcr.microsoft.com/dotnet/sdk:9.0

# Set working directory
WORKDIR /workspace/cs

# Set environment variables
ENV DOTNET_CLI_TELEMETRY_OPTOUT=1 \
    DOTNET_SKIP_FIRST_TIME_EXPERIENCE=1 \
    DOTNET_NOLOGO=1 \
    DOTNET_CLI_HOME=/tmp \
    NUGET_PACKAGES=/tmp/.nuget/packages

# Copy only project files for dependency restoration
COPY cs/*.csproj ./
COPY cs/*/*.csproj ./*/
COPY cs/Directory.Build.props ./

# Restore dependencies (cached in Docker layer)
RUN dotnet restore || true

# Note: Source code will be mounted via volume at runtime
# This ensures dependencies are cached but source is always fresh

# Default command
CMD ["/bin/bash"]

