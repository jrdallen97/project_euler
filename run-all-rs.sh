#!/bin/bash

while getopts "t" arg; do
  case $arg in
    t) time_scripts=true;;
  esac
done

# A stupid helper to run all my rust solutions in one go
# Run `cargo build -r` first
for f in target/release/p*; do
  if [[ "$f" =~ p[0-9]+$ ]]; then
    echo "$f"
    if [[ $time_scripts = true ]]; then
      eval time "$f"
    else
      eval "$f"
    fi
    echo
  fi
done
