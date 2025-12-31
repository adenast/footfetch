#!/bin/bash

EXECUTABLE_NAME="footfetch"
INSTALL_DIR="/usr/local/bin"

echo "Uninstalling $EXECUTABLE_NAME..."

if [ -f "$INSTALL_DIR/$EXECUTABLE_NAME" ]; then
    sudo rm "$INSTALL_DIR/$EXECUTABLE_NAME"
    echo "$EXECUTABLE_NAME has been successfully uninstalled."
else
    echo "The file '$EXECUTABLE_NAME' was not found in '$INSTALL_DIR'. It may not be installed or is in a different directory."
fi
