#!/usr/bin/env bash
set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."
cargo fmt --all --check
cargo test --workspace --release --locked
cargo build --workspace --release --locked

target_dir="$(cargo metadata --offline --no-deps --format-version 1 | python3 -c 'import json,sys;print(json.load(sys.stdin)["target_directory"])')"
showcase="$target_dir/release/tempo-ui-showcase"
renderers=("$@")
if (( ${#renderers[@]} == 0 )); then
    renderers=(winit-femtovg winit-software)
fi

for renderer in "${renderers[@]}"; do
    export SLINT_BACKEND="$renderer"
    "$showcase" --smoke
    "$showcase" --compact --dark --smoke
    "$showcase" --layout --wide --smoke
    "$showcase" --motion-smoke
    "$showcase" --motion-smoke --compact --dark
done
