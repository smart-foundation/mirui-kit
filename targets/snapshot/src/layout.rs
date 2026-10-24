//! Capture the actual Files app at OS-like sizes, not a Studio surrogate.
#[path="../../studio/src/capture_png.rs"]
mod capture_png;
use mirui::{prelude::*,surface::framebuf::FramebufSurface,render::texture::ColorFormat};
fn main() {
    let output=std::path::Path::new("artifacts/studio/files-layout");
    std::fs::create_dir_all(output).unwrap();
    for theme in ["dark","light"] {
        for (width,height,inspector) in [(640u16,600u16,false),(1180,720,false),(1400,900,false),(1400,900,true)] {
            let surface=FramebufSurface::with_format(width,height,ColorFormat::RGBA8888,|_,_|{});
            let mut app=App::new(surface);app.with_default_widgets().with_default_systems();
            let state=fm::install(&mut app);
            if theme=="light" {state.toggle_theme();}
            if inspector {state.toggle_inspector();}
            for _ in 0..6 {app.tick();app.render().unwrap();}
            let name=format!("files-{theme}-{width}x{height}{}.png",if inspector {"-inspector"} else {""});
            let image=app.snapshot().unwrap().unwrap();
            capture_png::write_png(&output.join(name),&image.read_pixels().unwrap());
        }
    }
}
