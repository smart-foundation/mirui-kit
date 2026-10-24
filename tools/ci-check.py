#!/usr/bin/env python3
"""Stream a CI command, preserve its exit code, expose failure diagnostics in checks.

Annotations make compiler output available through the Checks API even when a
runner's separate log-download host cannot be reached. Never mark failures green.
"""
import collections
import os
import re
import subprocess
import sys

if len(sys.argv) < 2:
    raise SystemExit("usage: ci-check.py COMMAND [ARG ...]")
lines = collections.deque(maxlen=5000)
with subprocess.Popen(sys.argv[1:], stdout=subprocess.PIPE,
                      stderr=subprocess.STDOUT, text=True, env=dict(os.environ, CARGO_TERM_COLOR="never")) as child:
    for line in child.stdout:
        print(line, end="", flush=True)
        lines.append(line)
    status = child.wait()
if status:
    full = "".join(lines)
    diagnostics = re.split(r"(?=^error(?:\[|:))", full, flags=re.M)
    text = "\n".join("\n".join(block.splitlines()[:18]) for block in diagnostics if block.startswith("error"))
    failed_tests = full.find("\nfailures:\n")
    if failed_tests >= 0:
        text = full[failed_tests:failed_tests + 17000] + "\n" + text
    if not text:
        text = full[-18000:]
    if len(text) > 18000:
        text = text[:18000]
    if len(text) > 80000:
        text = text[:65000] + "\n... middle diagnostics omitted ...\n" + text[-15000:]
    for i in range(0, len(text), 2000):
        escaped = text[i:i + 2000].replace("%", "%25").replace("\r", "%0D").replace("\n", "%0A")
        print(f"::error title=Command diagnostics {i // 2000 + 1}::{escaped}")
raise SystemExit(status)
