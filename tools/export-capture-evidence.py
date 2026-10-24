#!/usr/bin/env python3
"""Opt-in Checks-API transport for native captures when artifact hosts are blocked.

Normal runs still use upload-artifact. A manual export publishes the exact PNG
bytes as notice chunks, never synthesized images or a substitute screenshot.
"""
import base64
from pathlib import Path
import sys

if len(sys.argv) != 2:
    raise SystemExit("usage: export-capture-evidence.py CAPTURE.png")
path = Path(sys.argv[1])
data = path.read_bytes()
if not data.startswith(b"\x89PNG\r\n\x1a\n"):
    raise SystemExit("evidence must be a native PNG capture")
encoded = base64.b64encode(data).decode("ascii")
chunks = [encoded[i:i + 60000] for i in range(0, len(encoded), 60000)]
if len(chunks) > 10:
    raise SystemExit("capture exceeds a single step's notice capacity; use the artifact")
for index, chunk in enumerate(chunks, 1):
    print(f"::notice title=Native PNG {path.name} {index}/{len(chunks)}::{chunk}")
