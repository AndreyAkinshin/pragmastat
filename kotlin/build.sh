#!/bin/bash

echo "Building Pragmastat Kotlin implementation..."

# Clean and build
./gradlew clean build

# Run tests
echo -e "\nRunning tests..."
./gradlew test

# Generate test report
echo -e "\nTest results available at: build/reports/tests/test/index.html"

# Package JAR
echo -e "\nPackaging JAR..."
./gradlew jar

echo -e "\nBuild complete!"