//! Locate the SDL2 runtime library.
//!
//! Some distributions ship `libSDL2-2.0.so.0` without the unversioned
//! `libSDL2.so` development symlink, which is what `-lSDL2` looks for. When the
//! symlink is missing we create one inside `OUT_DIR` and point the linker at
//! it, so the desktop target builds on a plain runtime-only install.

use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let candidates = [
        "/usr/lib/x86_64-linux-gnu/libSDL2-2.0.so.0",
        "/usr/lib64/libSDL2-2.0.so.0",
        "/usr/lib/libSDL2-2.0.so.0",
        "/usr/local/lib/libSDL2-2.0.so.0",
    ];

    for candidate in candidates {
        let source = Path::new(candidate);
        if !source.exists() {
            continue;
        }
        let link = out_dir.join("libSDL2.so");
        let _ = std::fs::remove_file(&link);
        if std::os::unix::fs::symlink(source, &link).is_ok() {
            println!("cargo:rustc-link-search=native={}", out_dir.display());
            println!("cargo:rustc-link-lib=dylib=SDL2");
            return;
        }
    }

    // No runtime-only install found: fall back to the normal linker search.
    println!("cargo:rustc-link-lib=dylib=SDL2");
}
