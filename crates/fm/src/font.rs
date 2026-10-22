//! A MIRX font provider that always serves the signed-distance representation.
//!
//! `mirui`'s own [`MirxFontProvider`] picks a representation with
//! [`FontRepresentationRequest::new`] and the default `Auto` preference: a
//! coverage raster is only accepted when its `design_ppem` equals the
//! requested size, so every size that is not baked into the asset renders a
//! *scaled* coverage bitmap — soft, and slightly misaligned. The MiSans UI
//! asset also carries a signed distance field, which scales smoothly to any
//! size.
//!
//! This module wraps [`MirxFontProvider`] and re-implements the raster lookup
//! so the signed-distance representation is selected explicitly through
//! [`FontRepresentationPreference::SignedDistance`]. Character mapping,
//! advances, kerning, metrics and shaping are delegated to the wrapped
//! provider, so both behave identically apart from the raster source.
//!
//! # Fallbacks
//!
//! The provider never loses a glyph:
//!
//! * an asset without any signed-distance representation falls back to the
//!   default (`Auto`) selection of the wrapped provider;
//! * a glyph that the selected representation does not carry falls back to
//!   the wrapped provider for that single glyph;
//! * encoded surfaces (which the wrapped provider keeps in caller-owned
//!   storage) fall back to the wrapped provider as well, because decoded
//!   encoded surfaces cannot be re-bound to a different representation.
//!
//! # Example
//!
//! ```no_run
//! use fm::font::sdf_font;
//! use mirui::render::font::FontToken;
//!
//! let font = sdf_font(
//!     "MiSans UI",
//!     14,
//!     fm::FONT_BYTES,
//!     &mirx::reader::PayloadLimits::HOST,
//! )
//! .expect("MiSans UI MIRX asset decodes");
//! let key = FontToken::Default.cache_key();
//! # let _ = (font, key);
//! ```

use alloc::rc::Rc;
use core::cell::Cell;

use mirui::render::font::mirx::{MirxFontError, MirxFontProvider, MirxFontStorage};
use mirui::render::font::{
    Font, FontBackend, FontFaceId, FontMetrics, FontProvider, FontSurfaceId, GlyphId, GlyphSurface,
    RasterGlyph,
};
use mirui::text::{ShapeError, ShapeRequest, ShapedGlyph};
use mirui::types::Fixed;
use mirx::font::{
    FontGlyphs, FontRepresentationFallback, FontRepresentationKind, FontRepresentationPreference,
    FontRepresentationRequest, FontRepresentationView, FontView,
};
use mirx::reader::PayloadLimits;
use mirx::types::Fixed as MirxFixed;

/// A [`FontProvider`] that serves the signed-distance representation.
///
/// The provider owns no glyph storage of its own: it borrows the asset bytes
/// through the wrapped [`MirxFontProvider`], and the selected representation
/// is cached in a single cell, so no allocation happens per glyph.
#[derive(Clone, Debug)]
pub struct SdfFontProvider {
    inner: MirxFontProvider,
    view: FontView<'static>,
    selected: Cell<Option<(u16, FontRepresentationView<'static>)>>,
}

impl SdfFontProvider {
    /// Opens exactly one FONT face from a complete MIRX file.
    ///
    /// The bytes must live for the whole program: the provider borrows them.
    pub fn from_mirx(bytes: &'static [u8], limits: &PayloadLimits) -> Result<Self, MirxFontError> {
        Ok(Self::from_inner(MirxFontProvider::from_mirx(
            bytes, limits,
        )?))
    }

    /// Opens one FONT face and reconstructs encoded surfaces into
    /// caller-owned persistent storage.
    ///
    /// Encoded surfaces are served by the wrapped provider, because decoded
    /// encoded storage cannot be re-bound to another representation.
    pub fn from_mirx_with_storage(
        bytes: &'static [u8],
        limits: &PayloadLimits,
        storage: MirxFontStorage<'_>,
    ) -> Result<Self, MirxFontError> {
        Ok(Self::from_inner(MirxFontProvider::from_mirx_with_storage(
            bytes, limits, storage,
        )?))
    }

    fn from_inner(inner: MirxFontProvider) -> Self {
        Self {
            view: inner.view(),
            inner,
            selected: Cell::new(None),
        }
    }

    /// The representation that serves `output_ppem`, if the asset has one.
    pub fn selected_representation(
        &self,
        output_ppem: u16,
    ) -> Option<FontRepresentationView<'static>> {
        self.select(output_ppem)
    }

    /// The kind of representation that serves `output_ppem`.
    ///
    /// With a signed-distance asset this reports
    /// [`FontRepresentationKind::SignedDistance`] for every size.
    pub fn representation_kind(&self, output_ppem: u16) -> Option<FontRepresentationKind> {
        self.select(output_ppem)
            .map(|view| view.record().representation().kind())
    }

    /// Wraps the provider into a mirui [`Font`] resource.
    pub fn into_font(self, family: &'static str, size: u16) -> Font {
        Font {
            family,
            size,
            backend: FontBackend::Custom(Rc::new(self)),
        }
    }

    fn select(&self, output_ppem: u16) -> Option<FontRepresentationView<'static>> {
        if let Some((cached, view)) = self.selected.get()
            && cached == output_ppem
        {
            return Some(view);
        }
        // An exact coverage raster is a blit and stays cheap, so it wins when
        // the asset has one for this size. Otherwise the signed distance
        // field is used: it scales smoothly, at the cost of resampling the
        // field for every pixel.
        let exact = self
            .view
            .select(
                FontRepresentationRequest::new(output_ppem)
                    .with_preference(FontRepresentationPreference::Coverage)
                    .with_fallback(FontRepresentationFallback::Nearest),
            )
            .ok()
            .filter(|view| view.record().representation().design_ppem() == output_ppem);
        let view = match exact {
            Some(view) => view,
            None => self
                .view
                .select(
                    FontRepresentationRequest::new(output_ppem)
                        .with_preference(FontRepresentationPreference::SignedDistance)
                        .with_fallback(FontRepresentationFallback::Nearest),
                )
                .ok()?,
        };
        self.selected.set(Some((output_ppem, view)));
        Some(view)
    }

    /// Resolves one glyph against the signed-distance representation.
    ///
    /// Returns `None` when the asset carries no signed-distance raster for
    /// this glyph, which lets the caller fall back to the default selection.
    fn sdf_raster(
        &self,
        glyph: GlyphId,
        layout_ppem: u16,
        output_ppem: u16,
    ) -> Option<RasterGlyph<'_>> {
        let selected = self.select(output_ppem)?;
        let representation = selected.record().representation();
        if !matches!(
            representation.kind(),
            FontRepresentationKind::SignedDistance { .. }
        ) {
            // An exact coverage raster is a blit and stays cheap; the distance
            // field is resampled per pixel, so it is only worth it when the
            // asset has no coverage for this size.
            return None;
        }
        let mirx_glyph = mirx::font::GlyphId::new(glyph.value());
        let ordinal = self.view.raster_ordinal(mirx_glyph)?;
        let metric = selected.raster_metrics(mirx_glyph)?;
        let (plane, region) = match selected.glyphs() {
            FontGlyphs::Raw(storage) => {
                let raster = storage.get(ordinal)?;
                (raster.storage().plane(0)?, raster.region())
            }
            // Encoded surfaces live in the wrapped provider's storage.
            FontGlyphs::Encoded(_) => return None,
        };
        let geometry = plane.geometry();
        let surface_index = selected.record().surface_index();
        let surface = GlyphSurface::new(
            plane.bytes(),
            geometry.width(),
            geometry.height(),
            plane.memory().stride(),
            selected.surface().sample_layout(),
            plane.memory().required_alignment(),
            surface_id(self.inner.face_id(), surface_index),
        )
        .ok()?;
        Some(RasterGlyph {
            surface,
            region: (region.width() != 0 && region.height() != 0).then_some(region),
            representation,
            offset_x: scale(metric.offset_x(), layout_ppem, representation.design_ppem()),
            offset_y: scale(metric.offset_y(), layout_ppem, representation.design_ppem()),
        })
    }
}

impl FontProvider for SdfFontProvider {
    fn face_id(&self) -> FontFaceId {
        self.inner.face_id()
    }

    fn revision(&self) -> u64 {
        self.inner.revision()
    }

    fn map_char(&self, ch: char) -> Option<GlyphId> {
        self.inner.map_char(ch)
    }

    fn glyph_advance(&self, glyph: GlyphId, ppem: u16) -> Option<Fixed> {
        self.inner.glyph_advance(glyph, ppem)
    }

    fn pair_kerning(&self, left: GlyphId, right: GlyphId, ppem: u16) -> Fixed {
        self.inner.pair_kerning(left, right, ppem)
    }

    /// Raster storage for `glyph`, served from the signed-distance
    /// representation whenever the asset carries one.
    fn raster(
        &self,
        glyph: GlyphId,
        layout_ppem: u16,
        output_ppem: u16,
    ) -> Option<RasterGlyph<'_>> {
        self.sdf_raster(glyph, layout_ppem, output_ppem)
            .or_else(|| self.inner.raster(glyph, layout_ppem, output_ppem))
    }

    fn metrics(&self, ppem: u16) -> FontMetrics {
        self.inner.metrics(ppem)
    }

    fn notdef_glyph(&self) -> Option<GlyphId> {
        self.inner.notdef_glyph()
    }

    fn shaping_data(&self) -> Option<&[u8]> {
        self.inner.shaping_data()
    }

    fn shape_into(
        &self,
        ppem: u16,
        request: &ShapeRequest<'_>,
        output: &mut [ShapedGlyph],
    ) -> Result<usize, ShapeError> {
        self.inner.shape_into(ppem, request, output)
    }
}

/// Builds a mirui [`Font`] that renders through the signed-distance
/// representation.
///
/// The returned font can be registered with a [`FontToken`] cache key:
///
/// ```
/// use fm::font::sdf_font;
/// use mirui::render::font::FontToken;
///
/// let font = sdf_font(
///     "MiSans UI",
///     14,
///     fm::FONT_BYTES,
///     &mirx::reader::PayloadLimits::HOST,
/// )
/// .expect("MiSans UI MIRX asset decodes");
/// let _key = FontToken::Default.cache_key();
/// ```
pub fn sdf_font(
    family: &'static str,
    size: u16,
    bytes: &'static [u8],
    limits: &PayloadLimits,
) -> Result<Font, MirxFontError> {
    Ok(SdfFontProvider::from_mirx(bytes, limits)?.into_font(family, size))
}

/// Mirrors mirui's surface identity so a cached surface keeps its key when
/// the representation changes.
fn surface_id(face: FontFaceId, surface: u16) -> FontSurfaceId {
    FontSurfaceId::new(face.value().rotate_left(17) ^ u64::from(surface))
}

/// Scales a MIRX metric into mirui's Q24.8 fixed-point, in the logical
/// pixels of `layout_ppem`.
///
/// `value` is expressed in the pixels of the representation's
/// `design_ppem`, so the scale converts design pixels into logical pixels.
fn scale(value: MirxFixed, layout_ppem: u16, design_ppem: u16) -> Fixed {
    let numerator = i64::from(layout_ppem);
    let denominator = i64::from(design_ppem.max(1));
    let raw = i64::from(mirx_raw(value)) * numerator / denominator;
    let clamped = raw.clamp(i64::from(i32::MIN), i64::from(i32::MAX));
    Fixed::from_ratio(clamped as i32, 256)
}

/// Recovers the raw Q24.8 integer of a MIRX fixed-point value.
///
/// MIRX keeps its integer storage private, so the fractional byte is
/// recovered by binary search over the 256 steps of the integer part. The
/// search is exact: MIRX values are Q24.8, so `value >= from_ratio(k, 256)`
/// holds exactly for every `k` at or below the raw value.
fn mirx_raw(value: MirxFixed) -> i32 {
    let base = i64::from(value.to_int()) * 256;
    let mut low = 0i64;
    let mut high = 255i64;
    while low < high {
        let mid = (low + high + 1) / 2;
        let candidate = (base + mid).clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32;
        if value >= MirxFixed::from_ratio(candidate, 256) {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    (base + low).clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}
