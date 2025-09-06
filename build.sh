#!/bin/bash

# Build script for hyprfreeze-rs

echo "Building hyprfreeze-rs..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "Binary location: target/release/hyprfreeze-rs"
    echo ""
    echo "To install system-wide, run:"
    echo "  sudo cp target/release/hyprfreeze-rs /usr/local/bin/"
    echo ""
    echo "To install for current user only, run:"
    echo "  mkdir -p ~/.local/bin"
    echo "  cp target/release/hyprfreeze-rs ~/.local/bin/"
else
    echo "Build failed!"
    exit 1
fi