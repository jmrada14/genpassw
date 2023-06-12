#!/bin/bash

# build the project in release mode
cargo build --release

# check if the build was successful
if [ $? -eq 0 ]; then
    echo "Build successful. Installing..."

    # move the executable to /usr/local/bin
    sudo mv target/release/genpassw /usr/local/bin/

    echo "Installation complete. You can now use genpassw from anywhere."
else
    echo "Build failed. Please check your code and try again."
fi