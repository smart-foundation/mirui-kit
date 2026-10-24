use mirui::render::texture::{ColorFormat,TexturePixels};
use std::{path::Path,fs::File,io::BufWriter};
pub fn write_png(path: &Path, pixels: &TexturePixels<'_>) {
    assert_eq!(pixels.format(), ColorFormat::RGBA8888);
    let row_bytes = usize::from(pixels.width()) * 4;
    let mut packed = Vec::with_capacity(row_bytes * usize::from(pixels.height()));
    for row in 0..usize::from(pixels.height()) {
        let start = row * pixels.stride();
        packed.extend_from_slice(&pixels.bytes()[start..start + row_bytes]);
    }
    let mut encoder = png::Encoder::new(
        BufWriter::new(File::create(path).expect("snapshot file")),
        u32::from(pixels.width()),
        u32::from(pixels.height()),
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .expect("PNG header")
        .write_image_data(&packed)
        .expect("PNG pixels");
}
