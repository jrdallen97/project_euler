#!/bin/bash

SCRIPTNAME=$0
NUM=$1

# Helper for checking arg is valid uint
case "$NUM" in
  '' | *[!0-9]* )
    cat << EOF
Helper for creating a new project directory and rust binary
usage: $SCRIPTNAME {problem number}
example: $SCRIPTNAME 1
EOF
    exit 1;;
esac

PADDED=$(printf "%03d" "$NUM")
PNUM="p$PADDED"

# Create problem dir
mkdir -p "$PNUM"

# Create problem definition
if [[ ! -f "$PNUM/problem.md" ]]; then
  cat << EOF >"$PNUM/problem.md"
# $NUM - Title
EOF
fi

# Create rust problem
if [[ ! -f "$PNUM/main.rs" ]]; then
  cat << EOF >"$PNUM/main.rs"
fn main() {
    println!("Hello, world");
}
EOF
fi

# Add rust problem as a new binary in Cargo
if ! grep -F "$PNUM" Cargo.toml >/dev/null; then
  cat << EOF >>Cargo.toml
[[bin]]
name = "$PNUM"
path = "$PNUM/main.rs"
EOF
fi
