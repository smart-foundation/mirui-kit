#!/bin/sh
# Dependency-free tests for the exact no_std source used by Studio.
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT HUP INT TERM
rustc --edition=2024 --test "$ROOT/crates/studio-core/src/lib.rs" -o "$TMP/studio-core-tests"
"$TMP/studio-core-tests"
rustc --edition=2024 --crate-type=lib "$ROOT/crates/studio-core/src/lib.rs" --out-dir "$TMP"
