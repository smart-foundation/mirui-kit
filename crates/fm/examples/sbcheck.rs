//! Does the scroll bar find its container?
use mirui::app::App;
use mirui::render::texture::ColorFormat;
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::PhysicalRect;

fn main() {
    let backend = FramebufSurface::with_format(1180, 720, ColorFormat::RGBA8888, |_: &[u8], _: PhysicalRect| {});
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();
    let state = fm::install(&mut app);
    state.set_view(fm::model::ViewMode::List);
    app.render().expect("frame");
    app.tick();

    let ids = app.world.resource::<mirui::ui::IdMap>().expect("ids");
    println!("grid_scroller -> {:?}", ids.get("grid_scroller"));
    println!("list_scroller -> {:?}", ids.get("list_scroller"));

    let bars = app.world.query::<fm::ui::scrollbar::ScrollBar>();
    for (entity, bar) in bars.iter() {
        println!("bar {entity:?} target={:?} vertical={}", bar.target, bar.vertical);
        let off = app.world.get::<mirui::input::event::scroll::ScrollOffset>(bar.target);
        let bnd = app.world.get::<mirui::input::event::scroll::ScrollBounds>(bar.target);
        let rct = app.world.get::<mirui::ui::ComputedRect>(bar.target);
        println!("   offset={off:?}");
        println!("   bounds={bnd:?}");
        println!("   rect={:?}", rct.map(|r| (r.0.x.to_int(), r.0.y.to_int(), r.0.w.to_int(), r.0.h.to_int())));
        let own = app.world.get::<mirui::ui::ComputedRect>(entity);
        println!("   bar rect={:?}", own.map(|r| (r.0.x.to_int(), r.0.y.to_int(), r.0.w.to_int(), r.0.h.to_int())));
    }
    let _ = state;
}
