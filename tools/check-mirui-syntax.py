#!/usr/bin/env python3
"""Validate model/component/callback and UI lowering with actual pinned Mirui macros.

This deliberately does NOT claim to type-check Mirui or render the application.
GitHub source checkouts and Cargo outputs stay in the ignored .cache directory.
"""
import os
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
CACHE = ROOT / ".cache" / "mirui-syntax"


def run(*args, **kwargs):
    subprocess.run(args, check=True, **kwargs)


def checkout(name, repo, tag):
    path = CACHE / name
    if not path.exists():
        run("git", "clone", "--depth", "1", "--branch", tag,
            "https://github.com/" + repo + ".git", str(path))
    else:
        # A stale/tampered cache must not silently validate against another tag.
        head = subprocess.check_output(["git", "-C", str(path), "rev-parse", "HEAD"]).strip()
        expected = subprocess.check_output(["git", "-C", str(path), "rev-parse", tag + "^{commit}"]).strip()
        if head != expected:
            raise SystemExit("Wrong cached revision: " + str(path))
    return path


CACHE.mkdir(parents=True, exist_ok=True)
mirui = checkout("mirui", "W-Mai/mirui", "v0.51.0")
xrune = checkout("xrune", "W-Mai/xrune", "v1.10.1")
patches = {}
for name, tag in [("proc-macro2", "1.0.107"), ("quote", "1.0.47"),
                  ("syn", "2.0.119"), ("unicode-ident", "1.0.27")]:
    patches[name] = checkout(name, "dtolnay/" + name, tag)
for name in ["xrune", "xrune-nexus", "xrune-incant", "xrune-sigil"]:
    patches[name] = xrune / "crates" / name.replace("-", "_")
workspace = CACHE / "check"
workspace.mkdir(exist_ok=True)
macros = workspace / "macros"
# Only this derived copy is replaced, never the checkout or working sources.
if macros.exists():
    shutil.rmtree(macros)
shutil.copytree(mirui / "mirui-macros", macros)
manifest = (macros / "Cargo.toml").read_text().split("[dev-dependencies]")[0]
manifest = manifest.replace('proc-macro2 = "1"',
                            'proc-macro2 = { version = "1", features = ["span-locations"] }')
(macros / "Cargo.toml").write_text(manifest)
with (macros / "src" / "lib.rs").open("a") as output:
    output.write('\n#[cfg(test)]\n#[path = ' + repr(str(ROOT / "tools" / "mirui-syntax-check.rs")).replace("'", '"') + ']\nmod arena_source_check;\n')
manifest = '[workspace]\nmembers = ["macros"]\nresolver = "2"\n\n[patch.crates-io]\n'
for name, path in patches.items():
    manifest += name + ' = { path = "' + str(path) + '" }\n'
(workspace / "Cargo.toml").write_text(manifest)
env = dict(os.environ, MIRUI_KIT_ROOT=str(ROOT))
# Use the caller's Rust toolchain; no rustup download or hidden toolchain change.
run("cargo", "test", "--manifest-path", str(workspace / "Cargo.toml"),
    "--offline", "-p", "mirui-macros", "--lib", "arena_source_check", "--", "--nocapture", env=env)
