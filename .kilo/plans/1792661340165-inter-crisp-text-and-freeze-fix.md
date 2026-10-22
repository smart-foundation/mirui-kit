# Inter typeface with crisp coverage glyphs + kill the desktop meltdown

Repo: `/home/lufuno/mirui` (mirui 0.51.0 / mirx 0.51.0, pinned in `Cargo.lock:486,530`).
mirui sources: `/home/lufuno/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/mirui-0.51.0/`.

## Goal

1. Replace the MiSans asset with **Inter** (all 9 weights baked) and make every glyph **crisp**
   (exact-size coverage rasters, not a scaled SDF), on the baseline, at 1x / 1.5x / 2x scale.
2. Stop the desktop binary from melting down: it currently spins at 100% CPU and swallows clicks.
3. Give the desktop executable a real screenshot path (`--shot`) so before/after can be compared.

## Current state (verified, not assumed)

**Text is soft because the requested sizes are not in the asset.**
`assets/misans_ui.mirx` carries coverage at design ppem **11, 14, 22, 28** plus SDF at 24 and 64
(`mirui-0.51.0/tests/sdf_atlas_fixture.rs:87-117`). The app renders at **12/13/15**
(`crates/fm/src/text.rs:12,15,18`), so mirui's built-in `Auto` selection finds no exact coverage and
picks the SDF (`mirx-0.51.0/src/font/representation.rs:437-442`), which is then resampled
(`mirui-0.51.0/src/render/backends/sw/label.rs:1072-1084`) => soft, slightly misaligned stems.
The file is byte-identical to `mirui-0.51.0/src/gallery/demos/assets/misans_ui.mirx` (verified with
`diff`), and that asset was baked with `icu 0.11.1 bake-font`
(`mirui-0.51.0/src/gallery/demos/assets/nearby-font-source.json`) — so the upstream tool's output
already loads under mirx 0.51.

**`crates/fm/src/font.rs` is a workaround, and its docs are wrong.** It prefers exact coverage then
falls back to SDF (`font.rs:134-166`); the module docs claim "always serves the signed-distance
representation" (`font.rs:1-15`, `crates/fm/src/text.rs:1-5`, `crates/fm/src/lib.rs:45-47`). mirui
already has the built-in provider we need: `mirui::render::font::Font::from_mirx`
(`mirui-0.51.0/src/render/font/mod.rs:448-463`).

**Meltdown — two independent defects (agent-verified):**
- **No frame pacing.** `App::run()` is `loop { self.tick() }` with no sleep
  (`mirui-0.51.0/src/app/mod.rs:1420-1430`); a tick only sleeps if a `FramePacing` resource exists
  (`mirui-0.51.0/src/app/plugins/frame_rate.rs:49-56`), which only `FrameRateCapPlugin::build`
  inserts (`frame_rate.rs:84-90`). SDL `poll()` is non-blocking
  (`mirui-0.51.0/src/surface/sdl_events.rs:172-183`). The desktop target adds only
  `StdInstantClockPlugin` (`targets/desktop/src/main.rs:87`) => **permanent 100% CPU spin**.
- **The toast is a full-window `HitTarget`.** `crates/fm/src/ui/status.rs:55-68`: the outer View is
  `position: Absolute; width/height 100%` and owns `[HitTarget]`; the visible bubble is only 28px
  tall. The `[HitTarget]` enchant is inserted into the *enclosing* widget
  (`mirui-macros-0.51.0/src/lib.rs:644-653`), and the last matching hit candidate wins
  (`mirui-0.51.0/src/input/event/hit_test.rs:344-368`). `say()` sets `toast_visible = true`
  (`model.rs:708-713`) and it is only ever cleared by the toast's own `on Tap`
  (`status.rs:66-68` -> `model.rs:716-720`) => **the first click after any of the 7 `say()` actions
  is eaten**, and the toast also shadows the menu backdrop (toast is composed after the menu,
  `crates/fm/src/ui/mod.rs:47,50`).
- Per-frame model writes: `drag_tracking` calls `end_drag()` + `clear_press()` every frame
  (`crates/fm/src/sync.rs:73-77`); `end_drag()` unconditionally bumps `revision`
  (`model.rs:702`) and scans every `ComputedRect` entity before the early return
  (`sync.rs:36,48-71`). Refuted: the `revision` bump does **not** dirty the UI (nothing reads
  `fm.revision()`; `mirui-macros-0.51.0/src/model_attr.rs:916-931` only notifies on change and the
  source has no subscribers) and mirui has no reactive write-during-read spin (it panics instead,
  `mirui-0.51.0/src/core/model.rs:596-600`; flushes are capped at 32 passes,
  `mirui-0.51.0/src/core/reactive/mod.rs:799`). It is pure per-frame waste, not the meltdown.
- Latent meltdown: `theme_sync` compares *theme id strings* (`crates/fm/src/sync.rs:16-21`); a
  mismatch would call `set_theme` every frame, and `set_theme` does
  `world.mark_subtree_dirty(root)` (`mirui-0.51.0/src/ui/theme.rs:421-441`) = full re-layout every
  frame. Currently dormant (ids match), fix it anyway.
- `history` grows without bound (`model.rs:284`).

## Decisions (already agreed with the user)

- Bake **all 9 Inter weights** (Thin 100 … Black 900). A MIRX face is static and mirui 0.51 rejects
  multi-face files (`mirui-0.51.0/src/render/font/mirx.rs:106-112`; no bundle API exists in 0.51),
  and mirx 0.51 has no variable-axis support at all => **one `.mirx` per weight**.
- **Embed 3 by default** (Regular -> `FontToken::Default`, Medium, SemiBold -> `FontToken::Heading`),
  the other 6 behind a cargo feature. All 9 assets live in `assets/fonts/` for the OS suite.
- Bake a **proper charset** (ASCII + Latin-1 + typographic) and drop the "reduced charset" contract;
  restore the UI strings that the old asset forbade.
- Inter source: try to download the official release; **if the download is blocked, stop and ask the
  user** — never substitute a different typeface.
- Screenshot: use mirui's built-in `App::snapshot()` readback in the desktop target.

## Work plan

### Phase A — kill the meltdown

- **A1** `targets/desktop/src/main.rs:87` — add
  `app.add_plugin(mirui::app::plugins::FrameRateCapPlugin::new(60));`
  (API: `mirui-0.51.0/src/app/plugins/frame_rate.rs:70-76`; `std` feature is on for this target).
  Also add `mirui::app::plugins::PerfReportPlugin` temporarily to measure tick cost.
- **A2** `crates/fm/src/ui/status.rs:54-83` — move `[HitTarget]` from the full-window overlay onto the
  28px bubble, and put `IgnoreHitTest` on the overlay
  (`mirui::ui::{HitTarget, IgnoreHitTest}` are exported at `mirui-0.51.0/src/ui/mod.rs:60`;
  `IgnoreHitTest` excludes only that entity, children stay hit-testable,
  `mirui-0.51.0/src/input/event/hit_test.rs:208-211`). Add a one-shot auto-dismiss timer
  (`App::add_timer`, `mirui-0.51.0/src/app/mod.rs:741`, `core::timer::TimerSchedule::after`) that
  calls `clear_toast()` after ~2.5s, so the toast can never linger.
- **A3** `crates/fm/src/sync.rs:33-77` + `crates/fm/src/model.rs:677-703` —
  (a) move the `!cursor.down` early return **above** the zone scan;
  (b) call `end_drag()` only when `dragging`/`drop_target`/`pressed` is set, and make `end_drag()`
  return before the `revision` bump when there is nothing to end;
  (c) call `clear_press()` only when `pressed` is set;
  (d) query only `DropZone` entities and look up their rects instead of iterating every
  `ComputedRect`.
- **A4** `crates/fm/src/ui/menu.rs:29-47` + `crates/fm/src/ui/mod.rs:47,50` — same overlay treatment
  for the menu backdrop (no full-window `HitTarget`; close on a pointer press outside the panel),
  compose the **toast before the menu** so the menu dismissal wins, and clamp the menu panel
  (`menu.rs:48-61`, fixed 214x214 at the raw pointer position) so it stays on screen.
- **A5** `crates/fm/src/sync.rs:16-21` — drive the theme from model state (an observed
  `theme_applied` flag), never from a theme id string; only call `set_theme` on an actual change.
- **A6** `crates/fm/src/model.rs:284` — cap `history` (keep the last N places).

### Phase B — the font pipeline

- **B1** Vendor the sources: `https://github.com/rsms/inter/releases/download/v4.1/Inter-4.1.zip`
  -> extract `extras/ttf/Inter-{Thin,ExtraLight,Light,Regular,Medium,SemiBold,Bold,ExtraBold,Black}.ttf`
  and `LICENSE.txt` into `assets/fonts/src/`. Record the URL + SHA-256 of the zip in
  `assets/fonts/README.md`. If the download is blocked -> **stop and ask the user**.
  (Variable fonts are not an option: mirx 0.51 cannot express gvar/HVAR, and `icu bake-font`
  never exposes `--variation`.)
- **B2** `assets/fonts/charset.txt` — one codepoint per line:
  ASCII `0x20..0x7E`, Latin-1 `0xA0..0xFF`, plus `U+2013 U+2014 U+2018 U+2019 U+201C U+201D
  U+2026 U+2022 U+2039 U+203A U+00AB U+00BB U+2122 U+20AC U+2190 U+2191 U+2192 U+2193
  U+2248 U+2260 U+2264 U+2265`. The baker fails loudly on a missing codepoint
  (`hb-subset` `MissingCodepoint`), so iterate until it bakes clean.
- **B3** `tools/bake-fonts.sh` — per weight, per size, then merge into one face:
  ```
  icu bake-font assets/fonts/src/Inter-Regular.ttf --charset-file assets/fonts/charset.txt \
      --size 12 --bit-depth 8 --format gray -O build/fonts -r
  icu merge-fonts build/fonts/Inter-Regular_coverage_{11..16}.mirx \
      build/fonts/Inter-Regular_coverage_{18,20,23}.mirx \
      build/fonts/Inter-Regular_coverage_{24,26,28,30,32}.mirx \
      build/fonts/Inter-Regular_sdf_32.mirx -O assets/fonts/inter-regular.mirx
  ```
  SDF bake: `--size 32 --bit-depth 8 --spread 8 --min-ppem 17 --max-ppem 64 --format sdf`.
  Tool: `cargo install icu_tool --version 0.11.1`.
- **B4** **Size ladder per weight** (this is what makes glyphs crisp):
  - coverage A8 at design ppem **11,12,13,14,15,16** (1x band; the app uses 12/13/15)
  - coverage A8 at **18,20,23** (1.5x band: ceil(12x1.5)=18, ceil(13x1.5)=20, ceil(15x1.5)=23)
  - coverage A8 at **24,26,28,30,32** (2x band: ceil(12x2)=24, ceil(13x2)=26, ceil(15x2)=30)
  - one SDF A8 at design 32, spread 8, range 17..64 as the graceful fallback
  Selection is driven by **output ppem = ceil(logical x viewport_scale)**
  (`mirui-0.51.0/src/render/font/mod.rs:345-353`, `render/font/mirx.rs:350-356`), so a coverage baked
  only at 12 is never used at 2x. An exact-size coverage blits **1:1 with no resampling**
  (`render/backends/sw/label.rs:1072-1084` + `render/font/scalar.rs:139-158`).
- **B5** Outputs: `assets/fonts/inter-{thin,extralight,light,regular,medium,semibold,bold,extrabold,black}.mirx`
  (one face, 15 representations each). Delete `assets/misans_ui.mirx`; replace
  `assets/LICENSES/misans-ui.txt` with the Inter OFL 1.1 text.

### Phase C — app integration

- **C1** Delete `crates/fm/src/font.rs` (the custom provider) and fix the stale docs in
  `crates/fm/src/text.rs:1-5` and `crates/fm/src/lib.rs:45-47`.
- **C2** `crates/fm/src/lib.rs` — build fonts with the built-in provider:
  `mirui::render::font::Font::from_mirx("Inter", 12, bytes, &mirx::reader::PayloadLimits::HOST)`
  (`mirui-0.51.0/src/render/font/mod.rs:448-463`; `PayloadLimits::HOST` allows 1024 representations
  and 64 MiB decoded, `mirx-0.51.0/src/reader/limits.rs:51-70`). Register
  `FontToken::Default` -> Regular, `FontToken::Heading` -> SemiBold,
  `FontToken::Custom("inter-medium")` -> Medium, and the remaining six weights behind a feature.
- **C3** `crates/fm/Cargo.toml` — features `weight-thin|weight-extralight|weight-light|weight-regular|
  weight-medium|weight-semibold|weight-bold|weight-extrabold|weight-black` plus a convenience
  `weights-all`; default = regular + medium + semibold. `include_bytes!` the assets behind `cfg`.
- **C4** UI weight assignment (`Text` has a `font:` property,
  `mirui-0.51.0/src/ui/widgets/text.rs:639-642`): sheet titles -> `FontToken::Heading`, sidebar
  section header + toolbar title -> Medium, body stays Regular. This is what finally makes the
  registered `FontToken::Heading` meaningful.
- **C5** Restore the strings the old reduced charset forbade (e.g. `crates/fm/src/data.rs`
  "Users demo Cloud" -> "/Users/demo/Cloud", folder sizes, dashes/percent/parentheses). Check every
  widened string against the row widths in `crates/fm/src/ui/*.rs` in the visual pass.

### Phase D — screenshot

- **D1** `targets/desktop/src/main.rs` — accept `--shot <path>` (and/or `MIRUI_SHOT`): after
  `fm::install(&mut app)`, call `app.render()` then `app.snapshot()`
  (`mirui-0.51.0/src/app/mod.rs:1114-1138`) and write the RGBA bytes to a PNG with the `png` crate
  (already used by `targets/snapshot`); the SDL surface implements `FramebufferAccess`, so the SW
  renderer readback works. Continue into `app.run()` afterwards.
- **D2** `Makefile` — add a `shot` target next to `snapshot`/`desktop`.
- **D3** Keep `make snapshot` (headless PNG matrix in `artifacts/`) as the deterministic visual diff.

### Phase E — tests and docs

- **E1** `crates/fm/tests/typeface.rs` — rewrite:
  1. the new assets open under mirx 0.51 via `MirxFontProvider::from_mirx` (this is the
     mirx-0.46-written-by-`icu` vs mirx-0.51 compatibility gate);
  2. the ladder contains coverage at 12/13/15/24/26/30 and the SDF fallback;
  3. `provider.raster(glyph, 12, 12)` returns a **Coverage** raster (not SDF) for 12/13/15 and for
     24/26/30 (regression test for "crisp");
  4. the punctuation that used to be missing (`- / % ( ) _`) is now present (flip the old
     "known gaps" assertions, `typeface.rs:93-100`);
  5. all 9 weight files parse and share the same cmap.
- **E2** New test: every character appearing in the UI's string literals exists in the typeface cmap
  (scan `crates/fm/src/**/*.rs` at test time) — kills the "missing glyph" class of bug.
- **E3** `crates/fm/tests/interaction.rs` — regression test: after `trash()` raises a toast, the next
  pointer tap on a tile reaches the tile, not the toast.
- **E4** `README.md` — rewrite the Fonts section: MiSans -> Inter, "SDF for every size" -> the
  coverage ladder, drop the reduced-charset note, state the ROM cost of the 3 embedded weights and
  the `weights-all` feature. Update `assets/LICENSES/`.

## Validation

- `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`.
- `make bare` (riscv64gc, no_std) still builds; check the image size delta.
- `make snapshot` -> compare `artifacts/*.png` before/after (glyph sharpness at 12/13/15).
- `make shot` -> live SDL window capture; compare with the snapshot.
- Idle CPU ~0% (`top`, and `PerfReportPlugin` tick cost).
- Click-through after a toast; right-click menu still dismisses and stays on screen.
- The typeface test's coverage assertions are the objective "crisp glyphs" gate.

## Risks / fallbacks

- **Download blocked** -> stop and ask the user for the TTFs (their explicit instruction); do not
  substitute a typeface.
- **`icu` tool output vs mirx 0.51** -> already proven compatible (mirui 0.51 ships an asset baked
  by `icu 0.11.1`, and ours is byte-identical to it); the new typeface test gates it anyway.
- **`merge-fonts` reports `IncompatibleFaces`** -> `advance_source` compares hb-subset bytes
  byte-for-byte; if hb-subset is not deterministic, bake all sizes from one subset pass by porting
  upstream `font_bake.rs` into `tools/bake_font/` pinned to mirx 0.51.
- **ROM** -> 3 embedded weights ~2.5 MB (9 assets ~7 MB). Levers if needed: A4 coverage for the 2x
  band (halves it, `mirx-0.51.0/src/font/representation.rs:131-138` allows 1/2/4/8), or a
  regular-only feature for the bare target.
- **Inter's line box** -> line height = ascender - descender + line_gap scaled by the logical ppem
  (`mirui-0.51.0/src/render/font/mirx.rs:400-409`); Inter is taller than MiSans, so fixed row
  heights may need a nudge. Verify in the visual pass; the baked ladder covers 11-16 so a size
  tweak needs no re-bake.

## Out of scope

- Hinted/vector glyph rendering (mirx has no outline representation) and subpixel (LCD) AA.
- A runtime font service (mirui's provider wants `&'static [u8]`).
- Latin Extended-A / Greek / Cyrillic (fallback fonts, not Inter's job here).
