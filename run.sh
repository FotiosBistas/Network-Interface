#!/bin/bash
cargo b --release 

# Check if compilation succeeded
ext=$?
if [[ $ext -ne 0 ]]; then
    echo "compilation failed with exit status $ext"
    exit $ext
fi

# Set capabilities to the binary
sudo setcap CAP_NET_ADMIN=eip target/release/tcp

# Check if setcap succeeded
ext=$?
if [[ $ext -ne 0 ]]; then
    echo "setcap failed with exit status $ext"
    exit $ext
fi

# Run the binary in the background
target/release/tcp &
# Capture the PID of the background process
pid=$!  


# Set up a trap to handle interrupt and terminate signals
trap "echo 'Signal caught, killing process $pid'; kill $pid" INT TERM

# Wait for the background process to finish
wait $pid
