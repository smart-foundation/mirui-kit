//! Typography.
//!
//! The typeface is served through [`crate::font`], which hands out the exact
//! coverage raster whenever the asset has one for the requested size: a
//! coverage raster is a blit, so it stays cheap, and at its own size it is as
//! sharp as it can be. The sizes the asset does not have exactly fall back to
//! its signed distance field, which scales smoothly but has to be resampled
//! for every pixel, so the type scale sticks to the sizes that are baked.
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
