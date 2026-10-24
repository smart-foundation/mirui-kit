#!/bin/sh
# Tests the exact shared policy source; does not simulate Mirui or clipboard I/O.
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT HUP INT TERM
rustc --edition=2024 --test "$ROOT/crates/kit/src/editing_policy.rs" -o "$TMP/editing-policy-tests"
"$TMP/editing-policy-tests"
printf '#![no_std]\n#[path="%s/crates/kit/src/editing_policy.rs"]\npub mod editing_policy;\n' "$ROOT" > "$TMP/no_std.rs"
rustc --edition=2024 --crate-type=lib "$TMP/no_std.rs" --out-dir "$TMP"
