# mirui File Manager — a `no_std` validation suite

A modern, Apple-inspired file manager built entirely on
[`mirui`](https://crates.io/crates/mirui) 0.51, with the application logic compiled as
**`#![no_std]` + `alloc`**.

The point of this repository is not the file manager itself — it is a *proof* that mirui can carry
a real desktop-class application (custom chrome, vector icons, a virtualised list, reactive state,
theming) before those patterns get frozen into the OS app suite.

![the file manager](artifacts/01-dark-icons.png)

## What is in here

```
crates/fm          the application: #![no_std] + alloc, zero I/O, static fake filesystem
crates/sdl2-link   linker helper for the host targets
targets/desktop    std shell -> SDL2 window (interactive)
targets/snapshot   std shell -> headless render -> PNG (visual regression)
targets/bare       no_std shell -> bare-metal RISC-V image
assets/            the UI typeface (MIRX container) + licences
```

`crates/fm` owns the entire UI:

| Module | What it holds |
|---|---|
| `model` | The `#[model]` state: place, selection, view mode, palette, filter, history |
| `data` | 2048 fake files, built at compile time, sorted and filtered without touching a disk |
| `icons` | Hand-authored `path!()` vector assets on a 24x24 grid |
| `theme` | The light and dark palettes, including custom `ColorToken`s |
| `ui` | The chrome: toolbar, sidebar, content area, status bar |
| `sync` | The system that keeps the application theme in step with the state model |

## Running it

```bash
make build          # host build
make test           # 18 unit + interaction tests
make clippy         # lint gate
make snapshot       # writes artifacts/*.png
make desktop        # interactive window
make bare           # no_std RISC-V image
```

or directly:

```bash
cargo build --workspace
cargo test --workspace
cargo run -p snapshot
cargo run -p desktop
cd targets/bare && cargo build --target riscv64gc-unknown-none-elf
```

The bare target is a **separate workspace** on purpose: the desktop target turns on mirui's `std`
feature, and feature unification would otherwise drag `std` into the `no_std` binary.

## What the tests prove

* `tests/data.rs` — the fake filesystem, the filter, the sorting, and the state transitions.
* `tests/typeface.rs` — the typeface only carries a subset of ASCII; every string the UI can
  display is checked against the font's coverage so nothing renders as a silent gap.
* `tests/interaction.rs` — a scripted surface feeds real pointer events into the application, and
  the tests assert that tapping the theme button, a sidebar row, a file tile and the view switch
  actually change the state. Real hit testing, real handlers, real reactive updates.

## Fonts

`assets/misans_ui.mirx` is the MiSans UI typeface packaged in mirui's MIRX asset container
(MiSans is free for commercial use — see `assets/LICENSES/`). mirui only loads fonts from MIRX
containers; the built-in fallback is an 8x8 bitmap that is unusable for a real UI.

The asset carries a **reduced character set** (` !#&,0-9:;?@A-Za-z`): no dash, slash, percent sign
or brackets. That is why the chrome says `Users demo Cloud` instead of a path, and why folders
have no size. `tests/typeface.rs` keeps that honest.

## Notes for the OS port

* The application crate never touches the platform: it is a pure function of the state model and
  the data table. A target supplies a `Surface`, a font and the input events.
* Vector chrome is `path!()` literals in the program image — no image decoding, no texture atlas.
* Selection lives in the state model keyed by file id, never in the recycled list cell.
* `mirui` is pinned to `=0.51.0`: the DSL surface is still moving.
