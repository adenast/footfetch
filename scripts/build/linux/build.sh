#!/bin/bash

echo "What would you like to do?"
echo "1) Install (cargo install)"
echo "2) Build only (cargo build --release)"
read -p "Enter 1 or 2: " choice

echo "Cloning footfetch..."
git clone https://github.com/adenast/footfetch
cd footfetch

if [ "$choice" == "1" ]; then
    echo "Installing..."
    cargo install --path .
elif [ "$choice" == "2" ]; then
    echo "Building..."
    cargo build --release
    echo "Done. Binary is in footfetch/target/release/"
else
    echo "Invalid choice. Exiting."
    exit 1
fi