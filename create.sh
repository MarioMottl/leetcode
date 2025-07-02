#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <problem_name>"
  exit 1
fi

PROBLEM_NAME="$1"
RUST_FILE="src/bin/${PROBLEM_NAME}.rs"
CARGO_TOML="Cargo.toml"

if [ -f "$RUST_FILE" ]; then
  echo "Error: $RUST_FILE already exists."
  exit 1
fi

mkdir -p src/bin

cat >"$RUST_FILE" <<EOF


fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
}
EOF

echo -e "\n[[bin]]\nname = \"$PROBLEM_NAME\"\npath = \"$RUST_FILE\"" >>"$CARGO_TOML"

echo "Created $RUST_FILE and updated Cargo.toml"
