#!/bin/bash
# Docker entrypoint for R container
# Ensures compiled objects from host are cleaned before any operation

set -e

# Clean any compiled objects that might be from different architecture (host)
if [ "$R_CLEAN_BUILD" = "1" ]; then
    rm -rf /workspace/r/pragmastat/src/*.o \
           /workspace/r/pragmastat/src/*.so \
           /workspace/r/pragmastat/src/*.dll \
           /workspace/r/pragmastat/src/*.dylib 2>/dev/null || true
fi

# Execute the command
exec "$@"

