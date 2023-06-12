#!/bin/bash

# Check if the genpassw executable exists in /usr/local/bin
if [ -f /usr/local/bin/genpassw ]; then
    echo "genpassw found. Uninstalling..."

    # Remove the executable
    sudo rm /usr/local/bin/genpassw

    echo "Uninstallation complete. genpassw has been removed."
else
    echo "genpassw not found. It seems it's already uninstalled."
fi