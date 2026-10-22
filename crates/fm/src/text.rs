//! Typography.
//!
//! The typeface is served through [`crate::font`]. Every size on this scale is
//! baked into the assets as a coverage atlas at the exact pixel size it is
//! drawn at, so the glyphs are blitted one to one and the stems stay sharp.
//! The assets carry the scale at 1x, at 1.5x and at 2x, and a signed distance
//! atlas for the sizes in between, which is smooth but has to be resampled
//! for every pixel.
//!
//! The scale follows what a desktop file manager uses: small text for the
//! icon grid and the status bar, body text for the list, the sidebar and the
//! chrome, and a large size only for a title.

/// Icon grid labels, status bar, inspector details.
pub const SMALL: u16 = 11;

/// List rows, sidebar labels, toolbar title, menus, sheet bodies.
pub const BODY: u16 = 14;

/// A sheet title.
pub const TITLE: u16 = 22;
