# Dockerfile for Pragmastat Kotlin implementation
FROM eclipse-temurin:11-jdk

# Install bash
RUN apt-get update && \
    apt-get install -y --no-install-recommends bash && \
    rm -rf /var/lib/apt/lists/*

# Set working directory to kt
WORKDIR /workspace/kt

# Set Gradle home to a writable location for non-root users
ENV GRADLE_USER_HOME=/tmp/.gradle

# Copy only build files and gradle wrapper
COPY kt/build.gradle.kts kt/settings.gradle.kts kt/gradle.properties ./
COPY kt/gradle gradle/
COPY kt/gradlew ./

# Make gradlew executable
RUN chmod +x gradlew

# Download Gradle wrapper and dependencies (cached in Docker layer)
RUN ./gradlew --version --no-daemon

# Download project dependencies
RUN ./gradlew dependencies --no-daemon || true

# Note: Source code will be mounted via volume at runtime
# This ensures dependencies are cached but source is always fresh

# Default command
CMD ["/bin/bash"]

