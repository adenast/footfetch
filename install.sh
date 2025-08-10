#!/bin/bash

REPO_NAME="footfetch"
INSTALL_DIR="/usr/local/bin"
REPO_URL="https://github.com/aidendev0/$REPO_NAME.git"

# --- MAIN LOGIC ---

echo "Starting installation of $REPO_NAME"

if ! command -v git &> /dev/null
then
    echo "Error: git is not installed. Please install it."
    exit 1
fi

if ! command -v gcc &> /dev/null
then
    echo "Error: gcc is not installed. Please install it."
    exit 1
fi

echo "Cloning the repository..."
git clone "$REPO_URL" "$REPO_NAME"

if [ $? -ne 0 ]; then
    echo "Error: Failed to clone the repository."
    exit 1
fi

echo "Building the project..."
cd "$REPO_NAME"
gcc -o footfetch footfetch.c

if [ $? -ne 0 ]; then
    echo "Error: Failed to build the project."
    exit 1
fi

echo "Installing the executable to $INSTALL_DIR..."
sudo mv "$REPO_NAME" "$INSTALL_DIR/"

if [ $? -ne 0 ]; then
    echo "Error: Failed to move the executable."
    exit 1
fi

sudo chmod +x "$INSTALL_DIR/$REPO_NAME"

echo "Cleaning up temporary files..."
cd ..
rm -rf "$REPO_NAME"

echo "Installation of $REPO_NAME is complete! You can now run the program by typing '$REPO_NAME' in your terminal."