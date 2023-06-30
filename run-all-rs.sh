#!/bin/bash

# A stupid helper to run all my rust solutions in one go
# Run `cargo build -r` first
for f in target/release/p*; do
  if [[ "$f" =~ p[0-9]+$ ]]; then
    echo "$f"
    eval "$f"
    echo
  fi
done
