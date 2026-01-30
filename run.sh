#!/usr/bin/env bash

set -e

# Ensure cargo is available
export PATH="$HOME/.cargo/bin:/usr/bin:/bin"

# Move to project directory
cd /home/nikola-bozin/Documents/projects2.0/telegram_clear_all_chats

# Run the project
cargo run --release
