#!/bin/bash

# Installation script for hyprfreeze-rs

echo "Installing hyprfreeze-rs..."

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust first: https://www.rust-lang.org/"
    exit 1
fi

# Build the project
echo "Building hyprfreeze-rs..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful!"
    
    # Install system-wide (requires sudo)
    echo "Installing system-wide (requires sudo)..."
    sudo cp target/release/hyprfreeze-rs /usr/local/bin/
    
    if [ $? -eq 0 ]; then
        echo "Installation successful!"
        echo "You can now use 'hyprfreeze-rs' from anywhere in your system."
    else
        echo "Installation failed! You may need to run this script with sudo."
        echo "Alternatively, you can manually copy the binary from target/release/hyprfreeze-rs"
    fi
else
    echo "Build failed!"
    exit 1
fi