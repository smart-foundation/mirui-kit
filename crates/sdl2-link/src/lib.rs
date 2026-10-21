//! Link-time helper for the host targets.
//!
//! Some distributions ship `libSDL2-2.0.so.0` without the unversioned
//! `libSDL2.so` development symlink that `-lSDL2` looks for. The build script
//! of this crate creates that symlink inside `OUT_DIR` and points the linker
//! at it, so the SDL backend links on a plain runtime-only install.
//!
//! The crate has no code on purpose: it exists only to carry linker
//! directives from its build script into the final binary.

/// Name of the library the linker has to resolve.
pub const LIBRARY: &str = "SDL2";
