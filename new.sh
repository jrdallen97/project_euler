#!/usr/bin/env bash

# Helper for checking arg is a positive int
is_uint() {
  case $1 in '' | *[!0-9]* ) return 1;; esac
}

if ! is_uint "$1"; then
  echo 'integer problem name is required, e.g. 1'
  exit 1
fi

NUM="$1"
PADDED=$(printf "%03d" "$1")
PNUM="p$PADDED"

mkdir "$PNUM"

cat << EOF >"$PNUM/main.rs"
fn main() {
    println!("Hello, world");
}
EOF

cat << EOF >"$PNUM/problem.md"
# $NUM - Title
EOF

cat << EOF >>Cargo.toml
[[bin]]
name = "$PNUM"
path = "$PNUM/main.rs"
EOF

