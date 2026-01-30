#!/usr/bin/env bash

set -e

# Ensure cargo is available
export PATH="$HOME/.cargo/bin:/usr/bin:/bin"

# Move to project directory
cd /home/nikola-bozin/Documents/projects2.0/telegram_clear_all_chats

# Run the project
cargo run --release


# NOTE:
# This script is intended to be run from a GNOME keyboard shortcut / launcher,
# NOT from an interactive terminal.
#
# Keyboard shortcuts run with a minimal environment:
# - no working directory (cwd defaults to /)
# - no Cargo environment loaded
# - PATH does not include ~/.cargo/bin
#
# Therefore we must:
# 1) explicitly add Cargo to PATH
# 2) cd into the directory containing Cargo.toml
# 3) use absolute paths only
#
# When run manually from a terminal, this setup may look redundant — it is not.