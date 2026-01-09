#!/bin/bash

REPO_NAME="footfetch"
FILE_PATTERN="footfetch-1.41.7-x86_64-unknown-linux-gnu.tar.gz"
INSTALL_DIR="/usr/local/bin"
TEMP_ARCHIVE="/tmp/footfetch.tar.gz" 
EXTRACT_DIR="/tmp/footfetch_extracted"

echo "--- Starting installation of $REPO_NAME ---"

for cmd in curl tar; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is not installed."
        exit 1
    fi
done

echo "Fetching release metadata..."
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/adenast/footfetch/releases/latest | \
    grep -oP "https://github.com/adenast/footfetch/releases/download/[^\"]*$FILE_PATTERN")

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "Error: Could not find the file $FILE_PATTERN in the latest release."
    echo "Check if the version 1.41.7 actually exists in the 'adenast/footfetch' repo."
    exit 1
fi

echo "Downloading archive from: $LATEST_RELEASE_URL"
curl -L -f -o "$TEMP_ARCHIVE" "$LATEST_RELEASE_URL"

if [ $? -ne 0 ]; then
    echo "Error: Failed to download the file."
    exit 1
fi

FILE_TYPE=$(file "$TEMP_ARCHIVE")
if [[ "$FILE_TYPE" != *"gzip compressed data"* ]]; then
    echo "Error: Downloaded file is not a GZIP archive. Detected: $FILE_TYPE"
    exit 1
fi

echo "Extracting archive..."
rm -rf "$EXTRACT_DIR" && mkdir -p "$EXTRACT_DIR"
tar -xzf "$TEMP_ARCHIVE" -C "$EXTRACT_DIR"

BINARY_PATH=$(find "$EXTRACT_DIR" -type f -name "footfetch" | head -n 1)

if [ -z "$BINARY_PATH" ]; then
    echo "Error: Binary 'footfetch' not found inside the archive."
    echo "Contents of the archive were:"
    ls -R "$EXTRACT_DIR"
    exit 1
fi

echo "Installing to $INSTALL_DIR..."
sudo mv "$BINARY_PATH" "$INSTALL_DIR/$REPO_NAME"
sudo chmod +x "$INSTALL_DIR/$REPO_NAME"

rm "$TEMP_ARCHIVE"
rm -rf "$EXTRACT_DIR"

echo "Success! You can now run '$REPO_NAME' in your terminal."
