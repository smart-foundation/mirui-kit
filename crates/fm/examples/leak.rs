//! Does the app grow while it is used?
use mirui::app::App;
use mirui::render::texture::ColorFormat;
use mirui::surface::framebuf::FramebufSurface;
use mirui::types::PhysicalRect;
use std::time::Instant;

fn main() {
    let backend = FramebufSurface::with_format(
        1180,
        720,
        ColorFormat::RGBA8888,
        |_: &[u8], _: PhysicalRect| {},
    );
    let mut app = App::new(backend);
    app.with_default_widgets().with_default_systems();
    app.add_plugin(mirui::app::plugins::StdInstantClockPlugin);
    let state = fm::install(&mut app);
    app.render().expect("frame");

    let count = |app: &App<_, _>| {
        let query = app.world.query::<mirui::ui::ComputedRect>();
        query.iter().count()
    };
    println!("entities after the first frame: {}", count(&app));

    let t = Instant::now();
    for i in 0..600u32 {
        state.select(i % 40);
        app.tick();
        if i % 100 == 99 {
            println!(
                "after {} clicks: {} entities, {:?}",
                i + 1,
                count(&app),
                t.elapsed()
            );
        }
    }
    let click = t.elapsed().as_micros() as f64 / 600.0;
    println!("per click: {click:.1} us");

    // And the listing changes.
    let t = Instant::now();
    for i in 0..100u32 {
        state.set_query(if i % 2 == 0 { "report" } else { "" });
        app.tick();
    }
    println!(
        "per filter change: {:.1} us",
        t.elapsed().as_micros() as f64 / 100.0
    );
    println!("entities at the end: {}", count(&app));
}
