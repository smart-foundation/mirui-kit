#!/usr/bin/env python3
"""Opt-in Checks-API transport for native captures when artifact hosts are blocked.

Normal runs still use upload-artifact. A manual export publishes the exact PNG
bytes as notice chunks, never synthesized images or a substitute screenshot.
"""
import base64
from pathlib import Path
import sys

if len(sys.argv) != 3:
    raise SystemExit("usage: export-capture-evidence.py CAPTURE.png BATCH_1_TO_4")
path = Path(sys.argv[1])
data = path.read_bytes()
if not data.startswith(b"\x89PNG\r\n\x1a\n"):
    raise SystemExit("evidence must be a native PNG capture")
encoded = base64.b64encode(data).decode("ascii")
# Checks truncates each annotation's message to 4096 bytes. Actions permits
# ten notices per step; batches keep both limits explicit and lossless.
chunks = [encoded[i:i + 4000] for i in range(0, len(encoded), 4000)]
if len(chunks) > 40:
    raise SystemExit("capture exceeds review transport capacity; use the artifact")
batch = int(sys.argv[2])
if not 1 <= batch <= 4:
    raise SystemExit("batch must be between 1 and 4")
for index in range((batch - 1) * 10, min(batch * 10, len(chunks))):
    print(f"::notice title=Native PNG {path.name} {index + 1}/{len(chunks)}::{chunks[index]}")
