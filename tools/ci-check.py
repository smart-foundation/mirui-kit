#!/usr/bin/env python3
"""Stream a CI command, preserve its exit code, expose failure diagnostics in checks.

Annotations make compiler output available through the Checks API even when a
runner's separate log-download host cannot be reached. Never mark failures green.
"""
import collections
import subprocess
import sys

if len(sys.argv) < 2:
    raise SystemExit("usage: ci-check.py COMMAND [ARG ...]")
lines = collections.deque(maxlen=600)
with subprocess.Popen(sys.argv[1:], stdout=subprocess.PIPE,
                      stderr=subprocess.STDOUT, text=True) as child:
    for line in child.stdout:
        print(line, end="", flush=True)
        lines.append(line)
    status = child.wait()
if status:
    text = "".join(lines)
    for i in range(0, len(text), 10000):
        escaped = text[i:i + 10000].replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
        print(f"::error title=Command diagnostics {i // 10000 + 1}::{escaped}")
raise SystemExit(status)
