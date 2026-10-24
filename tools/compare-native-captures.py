#!/usr/bin/env python3
"""Compare native RGBA PNGs to explicitly reviewed baselines. No dependencies.

Missing baselines require review, not approval. This tool NEVER updates them.
Exact comparisons are default; tolerances are explicit command-line policy.
"""
import argparse
import json
from pathlib import Path
import struct
import zlib


def rgba(path):
    data = Path(path).read_bytes()
    if data[:8] != b'\x89PNG\r\n\x1a\n':
        raise ValueError(f'not PNG: {path}')
    offset = 8
    payload = bytearray()
    width = height = None
    ended = False
    while offset < len(data):
        size, = struct.unpack_from('>I', data, offset)
        kind = data[offset+4:offset+8]
        chunk = data[offset+8:offset+8+size]
        crc, = struct.unpack_from('>I', data, offset+8+size)
        if zlib.crc32(kind + chunk) & 0xffffffff != crc:
            raise ValueError(f'PNG CRC failure: {path}')
        if kind == b'IHDR':
            width, height, depth, color, compression, filtering, interlace = struct.unpack('>IIBBBBB', chunk)
            if (depth, color, compression, filtering, interlace) != (8, 6, 0, 0, 0):
                raise ValueError('expected non-interlaced RGBA8 native capture')
            if not width or not height or width * height > 16_000_000:
                raise ValueError('capture dimensions outside decoder budget')
        elif kind == b'IDAT':
            payload.extend(chunk)
        elif kind == b'IEND':
            ended = True
            offset += size + 12
            break
        offset += size + 12
    if not ended or offset != len(data) or width is None:
        raise ValueError(f'incomplete PNG: {path}')
    expected = (width * 4 + 1) * height
    decoder = zlib.decompressobj()
    raw = decoder.decompress(payload, expected + 1)
    if len(raw) != expected or not decoder.eof or decoder.unused_data:
        raise ValueError('invalid PNG scanline payload')
    stride = width * 4
    pixels = bytearray()
    previous = bytearray(stride)
    for y in range(height):
        start = y * (stride + 1)
        mode = raw[start]
        row = bytearray(raw[start+1:start+1+stride])
        if mode > 4:
            raise ValueError('unknown PNG filter')
        for x in range(stride):
            left = row[x-4] if x >= 4 else 0
            up = previous[x]
            upper_left = previous[x-4] if x >= 4 else 0
            if mode == 1:
                row[x] = (row[x] + left) & 255
            elif mode == 2:
                row[x] = (row[x] + up) & 255
            elif mode == 3:
                row[x] = (row[x] + (left + up)//2) & 255
            elif mode == 4:
                p = left + up - upper_left
                distances = (abs(p-left), abs(p-up), abs(p-upper_left))
                nearest = (left, up, upper_left)[distances.index(min(distances))]
                row[x] = (row[x] + nearest) & 255
        pixels.extend(row)
        previous = row
    return width, height, pixels


def write_rgba(path, width, height, pixels):
    def chunk(kind, body):
        return struct.pack('>I', len(body)) + kind + body + struct.pack('>I', zlib.crc32(kind+body) & 0xffffffff)
    stride = width * 4
    raw = b''.join(b'\0' + pixels[y*stride:(y+1)*stride] for y in range(height))
    data = b'\x89PNG\r\n\x1a\n' + chunk(b'IHDR', struct.pack('>IIBBBBB', width, height, 8, 6, 0, 0, 0)) + chunk(b'IDAT', zlib.compress(raw)) + chunk(b'IEND', b'')
    Path(path).write_bytes(data)


def compare(candidate, baseline, heatmap, threshold=0):
    w, h, actual = rgba(candidate)
    bw, bh, expected = rgba(baseline)
    if (w, h) != (bw, bh):
        return {'status': 'changed-size', 'actual': [w, h], 'baseline': [bw, bh]}
    changed = 0
    box = [w, h, -1, -1]
    image = bytearray()
    for i in range(w*h):
        delta = max(abs(actual[4*i+c]-expected[4*i+c]) for c in range(4))
        if delta > threshold:
            changed += 1
            x, y = i % w, i // w
            box = [min(box[0], x), min(box[1], y), max(box[2], x), max(box[3], y)]
            image.extend((255, 0, 80, 255))
        else:
            image.extend((*[actual[4*i+c]//4 for c in range(3)], 255))
    if changed:
        write_rgba(heatmap, w, h, image)
    return {'status': 'changed' if changed else 'identical', 'changed_pixels': changed, 'ratio': changed/(w*h), 'difference_bounds': box if changed else None}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('captures', type=Path)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--channel-threshold', type=int, default=0)
    parser.add_argument('--max-changed-ratio', type=float, default=0)
    parser.add_argument('--allow-missing', action='store_true', help='emit needs-review evidence without failing the capture-only bootstrap job')
    args = parser.parse_args()
    if not 0 <= args.channel_threshold <= 255 or not 0 <= args.max_changed_ratio <= 1:
        parser.error('tolerance outside valid range')
    args.out.mkdir(parents=True, exist_ok=True)
    index = json.loads((args.captures/'index.json').read_text())
    results = []
    failed = False
    for case in index['captures']:
        name = case['file']
        if Path(name).name != name or not name.endswith('.png'):
            raise ValueError('unsafe capture filename')
        actual = args.captures/name
        rgba(actual)  # validate even when no baseline exists
        baseline = args.baseline/name
        if not baseline.exists():
            result = {'status': 'needs-review', 'reason': 'no explicitly approved baseline'}
            failed |= not args.allow_missing
        else:
            result = compare(actual, baseline, args.out/name, args.channel_threshold)
            failed |= result['status'] == 'changed-size' or result.get('ratio', 0) > args.max_changed_ratio
        results.append(dict(case, comparison=result))
    report = {'schema': 1, 'capture_configuration': {k:v for k,v in index.items() if k != 'captures'}, 'tolerance': {'channel': args.channel_threshold, 'ratio': args.max_changed_ratio}, 'results': results, 'approved': False}
    (args.out/'report.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps({'cases': len(results), 'failed': failed, 'needs_review': sum(r['comparison']['status']=='needs-review' for r in results)}))
    return 1 if failed else 0

if __name__ == '__main__':
    raise SystemExit(main())
