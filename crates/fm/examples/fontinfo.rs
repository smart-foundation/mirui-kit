use mirui::render::font::mirx::MirxFontProvider;
use mirui::render::font::{Font, FontProvider};

fn main() {
    let bytes: &[u8] = include_bytes!("../../../assets/misans_ui.mirx");
    let limits = mirx::reader::PayloadLimits::HOST;
    let provider = MirxFontProvider::from_mirx(bytes, &limits).expect("font");
    for size in [10u16, 11, 12, 13, 14, 15, 17, 22] {
        let glyph = provider.map_char('A').expect("A");
        let raster = provider.raster(glyph, size, size);
        match raster {
            Some(r) => println!(
                "size {size}: {}x{} stride {} kind={:?} ppem={:?}",
                r.surface.width(),
                r.surface.height(),
                r.surface.stride(),
                r.representation.kind(),
                r.representation.design_ppem()
            ),
            None => println!("size {size}: no raster"),
        }
    }
    let _ = Font::from_mirx("MiSans UI", 14, bytes, &limits).expect("font");
}
