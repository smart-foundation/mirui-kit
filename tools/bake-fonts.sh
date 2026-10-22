#!/usr/bin/env bash
# Bake the Inter weights into MIRX font assets.
#
# mirui 0.51 picks a font representation by the *output* ppem, that is the
# logical size multiplied by the viewport scale. A coverage representation is
# only accepted when its design ppem matches that number exactly, and an exact
# match is blitted one to one, without resampling. A signed distance
# representation is resampled from its design size and therefore soft.
#
# The ladder below therefore carries a coverage atlas for every size the
# application can ask for, at 1x, 1.5x and 2x, and one signed distance atlas
# for everything in between.
#
# Usage: tools/bake-fonts.sh [weight ...]
# Requires: cargo install icu_tool --version 0.11.1

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$ROOT/assets/fonts/src"
OUT="$ROOT/assets/fonts"
CHARSET="$OUT/charset.txt"
BUILD="$ROOT/build/fonts"
PATCHED="$BUILD/src"

# The 1x band: the sizes the interface uses (11, 14, 22) and their neighbours.
SIZES_1X=(11 12 13 14 15 16 22)
# The 1.5x band: the same sizes on a 150% display.
SIZES_15X=(17 18 20 21 23 33)
# The 2x band: the same sizes on a 200% display.
SIZES_2X=(24 26 28 30 32 44)
# The signed distance fallback: everything from 17ppem up to 64ppem.
SDF_SIZE=24
SDF_SPREAD=4
SDF_MIN=17
SDF_MAX=64

# The small type is baked at eight bits per pixel; the large type is baked at
# four, because it is blitted one to one and the ramps are one or two pixels
# wide, so the sixteen levels are not visible.
DEPTH_SMALL=8
DEPTH_LARGE=4

ALL_WEIGHTS=(Thin ExtraLight Light Regular Medium SemiBold Bold ExtraBold Black)

WEIGHTS=("$@")
if [ ${#WEIGHTS[@]} -eq 0 ]; then
    WEIGHTS=("${ALL_WEIGHTS[@]}")
fi

command -v icu >/dev/null || {
    echo "icu not found: cargo install icu_tool --version 0.11.1" >&2
    exit 1
}

mkdir -p "$BUILD" "$PATCHED"

# `mirui` shapes with its own OpenType engine and rejects the lookup flags
# Inter uses, so the sources are patched before they are baked.
for weight in "${WEIGHTS[@]}"; do
    python3 "$ROOT/tools/patch-font-flags.py" \
        "$SRC/Inter-$weight.ttf" "$PATCHED/Inter-$weight.ttf" >/dev/null
done

bake() {
    local ttf="$1" size="$2" depth="$3" format="$4"
    shift 4
    icu bake-font "$ttf" \
        --charset-file "$CHARSET" \
        --size "$size" \
        --bit-depth "$depth" \
        --format "$format" \
        "$@" \
        -O "$BUILD" \
        -r >/dev/null
}

for weight in "${WEIGHTS[@]}"; do
    ttf="$PATCHED/Inter-$weight.ttf"
    if [ ! -f "$ttf" ]; then
        echo "missing source font: $SRC/Inter-$weight.ttf" >&2
        exit 1
    fi

    echo "== Inter $weight"
    pieces=()
    for size in "${SIZES_1X[@]}"; do
        bake "$ttf" "$size" "$DEPTH_SMALL" gray
        pieces+=("$BUILD/Inter-$weight"_coverage_"$size".mirx)
    done
    for size in "${SIZES_15X[@]}" "${SIZES_2X[@]}"; do
        bake "$ttf" "$size" "$DEPTH_LARGE" gray
        pieces+=("$BUILD/Inter-$weight"_coverage_"$size".mirx)
    done
    bake "$ttf" "$SDF_SIZE" 8 sdf \
        --spread "$SDF_SPREAD" --min-ppem "$SDF_MIN" --max-ppem "$SDF_MAX"
    pieces+=("$BUILD/Inter-$weight"_sdf_"$SDF_SIZE".mirx)

    name="$(echo "$weight" | tr '[:upper:]' '[:lower:]')"
    icu merge-fonts "${pieces[@]}" -O "$OUT/inter-$name.mirx"
    rm -f "${pieces[@]}"
done

echo "baked ${#WEIGHTS[@]} weights into $OUT"
