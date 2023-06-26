#!/usr/bin/env bash

# Helper for checking arg is a positive int
is_uint() {
  case $1 in '' | *[!0-9]* ) return 1;; esac
}

if ! is_uint "$1"; then
  echo 'Helper for creating a new project directory and rust binary'
  echo 'usage: ./new.sh {problem number}'
  echo 'example: ./new.sh 1'
  exit 1
fi

NUM="$1"
PADDED=$(printf "%03d" "$1")
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
