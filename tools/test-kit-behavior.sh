#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT HUP INT TERM
printf '#![no_std]\nextern crate alloc;\n#[path="%s/crates/kit/src/behavior/mod.rs"]\npub mod behavior;\n' "$ROOT" > "$TMP/lib.rs"
rustc --edition=2024 --test "$TMP/lib.rs" -o "$TMP/tests"
"$TMP/tests"
rustc --edition=2024 --crate-type=lib "$TMP/lib.rs" --out-dir "$TMP"
