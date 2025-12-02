#!/bin/bash

echo "🧪 Running Calendar Flatpak..."
echo

# Run with debug output
RUST_LOG=debug flatpak run dev.xarbit.apps.Calendar "$@"

# Alternative: Run with shell access for debugging
# Uncomment the following to open a shell inside the sandbox:
# echo "Opening shell in Flatpak sandbox..."
# flatpak run --command=bash dev.xarbit.apps.Calendar
