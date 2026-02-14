#!/bin/bash
# Wrapper script to run Gjallarhorn with root privileges for full hardware details.
# This script grants the root user access to the X11 display to avoid "Authorization required" errors.

# Allow root to connect to the X server
if command -v xhost &> /dev/null; then
    xhost +si:localuser:root
    echo "Granted X11 access to root."
else
    echo "Warning: xhost not found. GUI might fail if running as root."
fi

# Run the application with sudo, preserving environment variables (DISPLAY, XAUTHORITY)
echo "Starting Gjallarhorn as root..."
sudo -E ./target/release/gjallarhorn

# Note: Permissions persist until reboot or manual revocation.
