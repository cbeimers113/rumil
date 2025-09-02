#!/bin/bash

# Check if running from project root
ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
CURRENT_DIR="$PWD"

if [ "$PWD" != "$ROOT_DIR" ]; then
    echo "Please run build script from project root"
    exit 1
fi

mkdir -p build
cd build
cmake ..
cmake --build .
cd ..
