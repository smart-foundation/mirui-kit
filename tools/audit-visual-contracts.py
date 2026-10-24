#!/usr/bin/env python3
"""Read-only source/asset audit. Does NOT render, benchmark, or certify UI quality.

Run from any directory. Prints JSON; redirect to refresh the evidence document.
All measurements are declared-source calculations, except actual file metadata.
"""
import json
import math
from pathlib import Path
import re
import struct

ROOT = Path(__file__).resolve().parents[1]


def read(path):
    return (ROOT / path).read_text()


def constant(source, name):
    return int(re.search(r"pub const " + name + r": \w+ = (\d+);", source)[1])


def ttf_metadata(weight):
    path = ROOT / 'assets/fonts/src' / ('Inter-' + weight + '.ttf')
    data = path.read_bytes()
    count = struct.unpack_from('>H', data, 4)[0]
    tables = {}
    for index in range(count):
        tag, _, offset, size = struct.unpack_from('>4sIII', data, 12 + 16 * index)
        assert offset + size <= len(data)
        tables[tag.decode('latin1')] = (offset, size)
    os2, head = tables['OS/2'][0], tables['head'][0]
    return {
        'file': str(path.relative_to(ROOT)),
        'weight_class': struct.unpack_from('>H', data, os2 + 4)[0],
        'units_per_em': struct.unpack_from('>H', data, head + 18)[0],
        'variable_fvar_table': 'fvar' in tables,
    }


model = read('crates/fm/src/model.rs')
tile = constant(model, 'TILE_WIDTH')
cap = constant(model, 'GRID_COLUMNS')
grid = []
for width, inspector in [(1180, 0), (1180, 236), (1400, 0), (1122, 0)]:
    # Mirror the CURRENT model formula. These are source-derived, not live rects.
    old_available = max(0, width - 216 - 2 * 16)
    columns = max(4, min(cap, old_available // (tile + 6)))
    actual_available = max(0, width - 216 - inspector - 2 * 16)
    row = columns * tile + (columns - 1) * 6
    grid.append({
        'viewport_width': width, 'inspector_width': inspector,
        'current_model_columns': columns,
        'content_width_if_no_other_insets': actual_available,
        'declared_fixed_track_row_width': row,
        'unused_width_positive_overflow_negative': actual_available - row,
        'columns_that_fit_current_fixed_tracks_without_cap':
            max(0, (actual_available + 6) // (tile + 6)),
    })

bake = read('tools/bake-fonts.sh')
coverage = set()
for name in ['SIZES_1X', 'SIZES_15X', 'SIZES_2X']:
    values = re.search(name + r'=\(([^)]*)\)', bake)[1]
    coverage.update(int(value) for value in values.split())
font_probes = []
for logical, scale in [(11, 1), (14, 1), (14, 1.25), (14, 1.5), (14, 2), (19, 1), (15, 1.25)]:
    output = math.ceil(logical * scale)  # Mirui 0.51 output_ppem uses ceil.
    font_probes.append({'logical_size': logical, 'output_scale': scale,
                        'output_ppem': output,
                        'exact_coverage_in_bake_recipe': output in coverage})

images = []
for path in sorted((ROOT / 'artifacts').glob('*.png')):
    data = path.read_bytes()[:24]
    assert data[:8] == b'\x89PNG\r\n\x1a\n'
    width, height = struct.unpack('>II', data[16:24])
    images.append({'file': str(path.relative_to(ROOT)), 'width': width, 'height': height})

print(json.dumps({
    'method': 'Read-only metadata and current source arithmetic; not live render evidence.',
    'font_sources': [ttf_metadata(w) for w in ['Regular', 'Medium', 'SemiBold', 'Thin']],
    'font_probe_scope': 'Bake recipe only, not validation of packed MIRX bytes or selected runtime raster.',
    'coverage_recipe_ppem': sorted(coverage),
    'raster_recipe_probes': font_probes,
    'grid_arithmetic': grid,
    'normal_grid_item_cap': constant(model, 'ICON_GRID_MAX'),
    'search_grid_item_cap': constant(model, 'SEARCH_GRID_MAX'),
    'historical_png_dimensions': images,
}, indent=2))
