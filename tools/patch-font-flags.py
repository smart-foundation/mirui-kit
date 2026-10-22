#!/usr/bin/env python3
"""Clear the OpenType lookup flags that mirui's shaper rejects.

`mirui` 0.51 shapes with its own OpenType engine. A positioning lookup may
only set the "ignore marks" flag, and a substitution lookup may not set any
flag at all; anything else is reported as
`UnsupportedFeature { tag: "kern" }` and the whole interface then fails to
lay out. Inter sets the "use mark filtering set" flag on its kerning lookups,
so the font has to be patched before it is baked.

The patch clears the offending bits in place. The tables keep their size, so
no offset moves, and the mark filtering sets that the cleared flag pointed at
are simply left unread.

Usage: tools/patch-font-flags.py <input.ttf> <output.ttf>
"""

import struct
import sys

# The lookup flags mirui accepts.
POSITIONING_FLAGS = 0x0008  # ignore marks
SUBSTITUTION_FLAGS = 0x0000  # none


def tables(data):
    """The sfnt table directory as a list of (tag, offset, length)."""
    count = struct.unpack(">H", data[4:6])[0]
    found = []
    for index in range(count):
        entry = 12 + index * 16
        tag = data[entry : entry + 4].decode("latin1")
        offset, length = struct.unpack(">II", data[entry + 8 : entry + 16])
        found.append((tag, offset, length))
    return found


def patch_lookup_flags(data, base, allowed):
    """Clear the disallowed flag bits of every lookup of a layout table."""
    lookup_list = base + struct.unpack(">H", data[base + 8 : base + 10])[0]
    count = struct.unpack(">H", data[lookup_list : lookup_list + 2])[0]
    changed = 0
    for index in range(count):
        entry = lookup_list + 2 + index * 2
        # The lookup offsets are relative to the lookup list.
        lookup = lookup_list + struct.unpack(">H", data[entry : entry + 2])[0]
        flags = struct.unpack(">H", data[lookup + 2 : lookup + 4])[0]
        if flags & ~allowed:
            struct.pack_into(">H", data, lookup + 2, flags & allowed)
            changed += 1
    return changed


def main():
    if len(sys.argv) != 3:
        print(__doc__, file=sys.stderr)
        return 2
    source, target = sys.argv[1], sys.argv[2]
    with open(source, "rb") as handle:
        data = bytearray(handle.read())
    changed = 0
    for tag, offset, _ in tables(data):
        if tag == "GPOS":
            changed += patch_lookup_flags(data, offset, POSITIONING_FLAGS)
        elif tag == "GSUB":
            changed += patch_lookup_flags(data, offset, SUBSTITUTION_FLAGS)
    with open(target, "wb") as handle:
        handle.write(data)
    print(f"{source} -> {target}: {changed} lookups patched")
    return 0


if __name__ == "__main__":
    sys.exit(main())
