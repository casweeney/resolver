#!/bin/sh
set -e

# Determining this operating system and architecture
OS=$(uname -s)
ARCH=$(uname -m)


# Set the download URL based on the OS and architecture
if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        URL="https://example.com/resolver-cli-linux-x86_64"
    elif [ "$ARCH" = "arm64" ]; then
        URL="https://example.com/resolver-cli-linux-arm64"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        URL="https://example.com/resolver-cli-macos-x86_64"
    elif [ "$ARCH" = "arm64" ]; then
        URL="https://example.com/resolver-cli-macos-arm64"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
elif [ "$OS" = "Windows" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        URL="https://example.com/resolver-cli-windows-x86_64.exe"
    else
        echo "Unsupported architecture: $ARCH"
        exit 1
    fi
else
    echo "Unsupported OS: $OS"
    exit 1
fi

# Download the binary
curl -L -o resolver-cli $URL

# Make it executable
chmod +x resolver-cli

# Move it to a directory in PATH
mv resolver-cli /usr/local/bin/resolver-cli

echo "resolver-cli installed successfully!"