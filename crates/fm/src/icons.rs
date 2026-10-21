//! Hand-authored vector chrome.
//!
//! Every icon is a `path!` literal on a 24x24 grid, embedded in the program image —
//! no image decoding, no texture atlas, no filesystem access. mirui fills paths with
//! the even-odd rule, so a ring is a ring and two disjoint shapes are two shapes.

use mirui::prelude::*;

/// A folder with a tab.
pub static FOLDER: Path = path!(M 2 6 L 10 6 L 12 9 L 22 9 L 22 20 L 2 20 Z);

/// A document page with a folded corner.
pub static DOCUMENT: Path = path!(M 6 3 L 14 3 L 19 8 L 19 21 L 6 21 Z);

/// A photo frame: an outer plate with the picture cut out of it.
pub static IMAGE: Path = path!(M 3 5 L 21 5 L 21 19 L 3 19 Z M 6 8 L 18 8 L 18 16 L 6 16 Z);

/// Angle brackets for source files.
pub static CODE: Path = path!(M 9 7 L 3 12 L 9 17 L 11 15 L 7 12 L 11 9 Z M 15 7 L 21 12 L 15 17 L 13 15 L 17 12 L 13 9 Z);

/// A zip archive box.
pub static ARCHIVE: Path = path!(M 3 4 L 21 4 L 21 8 L 3 8 Z M 5 10 L 19 10 L 19 20 L 5 20 Z);

/// A music note.
pub static AUDIO: Path =
    path!(M 9 3 L 20 3 L 20 13 L 17 13 L 17 6 L 12 6 L 12 17 A 3 3 0 1 1 9 17 Z);

/// A star, for favourites.
pub static STAR: Path =
    path!(M 12 4 L 13 9 L 18 9 L 14 12 L 16 17 L 12 14 L 8 17 L 10 12 L 6 9 L 11 9 Z);

/// A download arrow.
pub static DOWNLOAD: Path = path!(M 12 3 L 20 11 L 15 11 L 15 17 L 9 17 L 9 11 L 4 11 Z);

/// A folder pair, for projects.
pub static PROJECTS: Path = path!(M 2 5 L 9 5 L 11 8 L 20 8 L 20 12 L 2 12 Z M 2 14 L 9 14 L 11 17 L 20 17 L 20 21 L 2 21 Z);

/// A trash can.
pub static TRASH: Path =
    path!(M 9 3 L 15 3 L 15 5 L 20 5 L 20 7 L 4 7 L 4 5 L 9 5 Z M 6 9 L 18 9 L 17 21 L 7 21 Z);

/// A magnifying glass: outer ring, inner ring, handle.
pub static SEARCH: Path = path!(M 10 3 A 7 7 0 1 0 10 17 A 7 7 0 1 0 10 3 Z M 10 6 A 4 4 0 1 1 10 14 A 4 4 0 1 1 10 6 Z M 15 15 L 21 21 L 19 23 L 13 17 Z);

/// A four-square grid, for the icon view.
pub static GRID: Path = path!(M 4 4 L 11 4 L 11 11 L 4 11 Z M 13 4 L 20 4 L 20 11 L 13 11 Z M 4 13 L 11 13 L 11 20 L 4 20 Z M 13 13 L 20 13 L 20 20 L 13 20 Z);

/// Three lines, for the list view.
pub static LIST: Path = path!(M 9 5 L 21 5 L 21 7 L 9 7 Z M 9 11 L 21 11 L 21 13 L 9 13 Z M 9 17 L 21 17 L 21 19 L 9 19 Z M 3 5 L 6 5 L 6 7 L 3 7 Z M 3 11 L 6 11 L 6 13 L 3 13 Z M 3 17 L 6 17 L 6 19 L 3 19 Z);

/// A sun, for the light theme.
pub static SUN: Path = path!(M 11 6 A 5 5 0 1 0 13 6 A 5 5 0 1 0 11 6 Z M 11 1 L 13 1 L 13 4 L 11 4 Z M 11 20 L 13 20 L 13 23 L 11 23 Z M 1 11 L 4 11 L 4 13 L 1 13 Z M 20 11 L 23 11 L 23 13 L 20 13 Z M 4 5 L 6 7 L 4 9 L 2 7 Z M 18 5 L 20 7 L 18 9 L 16 7 Z M 6 18 L 4 20 L 2 18 L 4 16 Z M 18 18 L 20 16 L 22 18 L 20 20 Z);

/// A crescent moon, for the dark theme.
pub static MOON: Path = path!(M 19 3 A 9 9 0 1 0 21 17 A 7 7 0 1 1 19 3 Z);

/// A chevron pointing left.
pub static CHEVRON_LEFT: Path = path!(M 15 4 L 7 12 L 15 20 L 17 18 L 11 12 L 17 6 Z);

/// A chevron pointing right.
pub static CHEVRON_RIGHT: Path = path!(M 9 4 L 17 12 L 9 20 L 7 18 L 13 12 L 7 6 Z);
