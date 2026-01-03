#!/bin/bash

REPO_NAME="footfetch"
INSTALL_DIR="/usr/local/bin"
TEMP_FILE="/tmp/$REPO_NAME"

echo "Starting installation of $REPO_NAME"

if ! command -v curl &> /dev/null; then
    echo "Error: curl is not installed."
    exit 1
fi

echo "Fetching latest release metadata..."
LATEST_RELEASE_URL=$(curl -s https://api.github.com/repos/adenast/footfetch/releases/latest | grep "browser_download_url" | grep "footfetch" | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "Error: Could not find the latest release URL. Check repo name or internet connection."
    exit 1
fi

echo "Downloading package from $LATEST_RELEASE_URL..."
curl -L -o "$TEMP_FILE" "$LATEST_RELEASE_URL"

if [ $? -ne 0 ]; then
    echo "Error: Failed to download the file."
    exit 1
fi

echo "Installing to $INSTALL_DIR..."
sudo mv "$TEMP_FILE" "$INSTALL_DIR/$REPO_NAME"
sudo chmod +x "$INSTALL_DIR/$REPO_NAME"

echo "Installation of $REPO_NAME is complete!"
