#!/bin/bash

REPO_NAME="footfetch"
FILE_PATTERN="footfetch-1.41.7-x86_64-unknown-linux-gnu.tar.gz"
INSTALL_DIR="/usr/local/bin"
TEMP_ARCHIVE="/tmp/$FILE_PATTERN"
EXTRACT_DIR="/tmp/footfetch_extracted"

echo "Starting installation of $REPO_NAME"

for cmd in curl tar; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is not installed."
        exit 1
    fi
done

echo "Fetching release metadata for version 1.41.7..."
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/adenast/footfetch/releases/latest | \
    grep "browser_download_url" | \
    grep "$FILE_PATTERN" | \
    cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "Error: Could not find the file $FILE_PATTERN in the latest release."
    exit 1
fi

echo "Downloading $FILE_PATTERN..."
curl -L -o "$TEMP_ARCHIVE" "$LATEST_RELEASE_URL"

if [ $? -ne 0 ]; then
    echo "Error: Failed to download the file."
    exit 1
fi

echo "Extracting archive..."
mkdir -p "$EXTRACT_DIR"
tar -xzf "$TEMP_ARCHIVE" -C "$EXTRACT_DIR"

BINARY_PATH=$(find "$EXTRACT_DIR" -type f -name "footfetch" | head -n 1)

if [ -z "$BINARY_PATH" ]; then
    echo "Error: Binary 'footfetch' not found inside the archive."
    exit 1
fi

echo "Installing to $INSTALL_DIR..."
sudo mv "$BINARY_PATH" "$INSTALL_DIR/$REPO_NAME"
sudo chmod +x "$INSTALL_DIR/$REPO_NAME"

rm "$TEMP_ARCHIVE"
rm -rf "$EXTRACT_DIR"

echo "Installation of $REPO_NAME is complete!"