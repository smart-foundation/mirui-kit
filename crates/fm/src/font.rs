//! The typefaces.
//!
//! Inter is baked into one MIRX asset per weight. A MIRX face is static — it
//! carries one advance source and one raster set — and `mirui` 0.51 opens
//! exactly one face per file, so every weight is its own asset.
//!
//! A face in turn carries one *representation* per size. `mirui` picks the
//! representation by the output ppem, that is the logical size multiplied by
//! the viewport scale, and under the default `Auto` preference a coverage
//! atlas is only accepted when its design ppem matches that number exactly.
//! An exact match is blitted one to one, so the stems stay sharp; every other
//! size is resampled from the signed distance atlas, which is smooth but
//! soft.
//!
//! The assets therefore carry a coverage atlas for every size the interface
//! can ask for — at 1x, at 1.5x and at 2x — and one signed distance atlas for
//! the sizes in between. `tools/bake-fonts.sh` builds them from the sources in
//! `assets/fonts/src`.
//!
//! # Weights
//!
//! The regular weight is always compiled in. The other eight are behind cargo
//! features, because every weight is around two megabytes of atlas:
//!
//! | feature | weight | token |
//! |---|---|---|
//! | — | regular | [`FontToken::Default`] |
//! | `weight-medium` | medium | [`MEDIUM`] |
//! | `weight-semibold` | semibold | [`FontToken::Heading`] |
//! | `weight-thin` | thin | [`THIN`] |
//! | `weight-extralight` | extra light | [`EXTRA_LIGHT`] |
//! | `weight-light` | light | [`LIGHT`] |
//! | `weight-bold` | bold | [`BOLD`] |
//! | `weight-extrabold` | extra bold | [`EXTRA_BOLD`] |
//! | `weight-black` | black | [`BLACK`] |
//!
//! `weights-all` turns on every one of them.

use mirui::app::{App, RendererFactory};
use mirui::render::font::{Font, FontManager, FontToken};
use mirui::surface::Surface;
use mirx::reader::PayloadLimits;

/// The family name of the typefaces.
pub const FAMILY: &str = "Inter";

/// The layout size a typeface is registered with.
///
/// A widget that asks for a size of its own overrides this, so this is only
/// the size used by the widgets that do not.
pub const BASE_SIZE: u16 = 14;

/// The token of the medium weight.
pub const MEDIUM: FontToken = FontToken::Custom("inter-medium");

/// The token of the semibold weight.
pub const SEMIBOLD: FontToken = FontToken::Heading;

/// The token of the thin weight.
#[cfg(feature = "weight-thin")]
pub const THIN: FontToken = FontToken::Custom("inter-thin");

/// The token of the extra light weight.
#[cfg(feature = "weight-extralight")]
pub const EXTRA_LIGHT: FontToken = FontToken::Custom("inter-extralight");

/// The token of the light weight.
#[cfg(feature = "weight-light")]
pub const LIGHT: FontToken = FontToken::Custom("inter-light");

/// The token of the bold weight.
#[cfg(feature = "weight-bold")]
pub const BOLD: FontToken = FontToken::Custom("inter-bold");

/// The token of the extra bold weight.
#[cfg(feature = "weight-extrabold")]
pub const EXTRA_BOLD: FontToken = FontToken::Custom("inter-extrabold");

/// The token of the black weight.
#[cfg(feature = "weight-black")]
pub const BLACK: FontToken = FontToken::Custom("inter-black");

/// Inter Regular, the default weight.
pub const REGULAR: &[u8] = include_bytes!("../../../assets/fonts/inter-regular.mirx");

/// Inter Medium.
#[cfg(feature = "weight-medium")]
pub const MEDIUM_BYTES: &[u8] = include_bytes!("../../../assets/fonts/inter-medium.mirx");

/// Inter SemiBold.
#[cfg(feature = "weight-semibold")]
pub const SEMIBOLD_BYTES: &[u8] = include_bytes!("../../../assets/fonts/inter-semibold.mirx");

/// Inter Thin.
#[cfg(feature = "weight-thin")]
pub const THIN_BYTES: &[u8] = include_bytes!("../../../assets/fonts/inter-thin.mirx");

/// Inter ExtraLight.
#[cfg(feature = "weight-extralight")]
pub const EXTRA_LIGHT_BYTES: &[u8] = include_bytes!("../../../assets/fonts/inter-extralight.mirx");

/// Inter Light.
#[cfg(feature = "weight-light")]
pub const LIGHT_BYTES: &[u8] = include_bytes!("../../../assets/fonts/inter-light.mirx");

/// Inter Bold.
#[cfg(feature = "weight-bold")]
pub const BOLD_BYTES: &[u8] = include_bytes!("../../../assets/fonts/inter-bold.mirx");

/// Inter ExtraBold.
#[cfg(feature = "weight-extrabold")]
pub const EXTRA_BOLD_BYTES: &[u8] = include_bytes!("../../../assets/fonts/inter-extrabold.mirx");

/// Inter Black.
#[cfg(feature = "weight-black")]
pub const BLACK_BYTES: &[u8] = include_bytes!("../../../assets/fonts/inter-black.mirx");

/// Register the typefaces with the font manager.
///
/// Every weight is registered under its own token, so a widget picks a weight
/// by naming the token.
pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>) {
    let Some(manager) = app.world.resource::<FontManager>() else {
        return;
    };
    let limits = PayloadLimits::HOST;
    register(manager, FontToken::Default, REGULAR, &limits);
    #[cfg(feature = "weight-medium")]
    register(manager, MEDIUM, MEDIUM_BYTES, &limits);
    #[cfg(feature = "weight-semibold")]
    register(manager, SEMIBOLD, SEMIBOLD_BYTES, &limits);
    #[cfg(feature = "weight-thin")]
    register(manager, THIN, THIN_BYTES, &limits);
    #[cfg(feature = "weight-extralight")]
    register(manager, EXTRA_LIGHT, EXTRA_LIGHT_BYTES, &limits);
    #[cfg(feature = "weight-light")]
    register(manager, LIGHT, LIGHT_BYTES, &limits);
    #[cfg(feature = "weight-bold")]
    register(manager, BOLD, BOLD_BYTES, &limits);
    #[cfg(feature = "weight-extrabold")]
    register(manager, EXTRA_BOLD, EXTRA_BOLD_BYTES, &limits);
    #[cfg(feature = "weight-black")]
    register(manager, BLACK, BLACK_BYTES, &limits);
}

/// Open one asset and put it under its token.
fn register(manager: &FontManager, token: FontToken, bytes: &'static [u8], limits: &PayloadLimits) {
    let font = Font::from_mirx(FAMILY, BASE_SIZE, bytes, limits)
        .expect("Inter MIRX asset decodes");
    manager.add_static(token.cache_key(), font);
}
