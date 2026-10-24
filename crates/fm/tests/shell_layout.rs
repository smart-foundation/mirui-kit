//! Files itself must be tested, not only Studio's shared specimens.
use mirui::prelude::*;
use fm::model::{FmState,Place};
#[test]
fn grid_uses_the_measured_pane_without_an_eight_column_cap() {
    let mut state=FmState::new();
    state.set_content_width(1184);
    assert_eq!(state.grid_columns(),10);
    state.set_content_width(236);
    assert_eq!(state.grid_columns(),1);
    state.set_content_width(472);
    assert_eq!(state.grid_columns(),4);
}
#[test]
fn native_shell_counts_and_controls_fit_at_real_window_sizes() {
    for (width,height) in [(640,600),(1180,720),(1400,900)] {
        let backend=mirui::surface::framebuf::FramebufSurface::with_format(width,height,mirui::render::texture::ColorFormat::RGBA8888,|_,_|{});
        let mut app=App::new(backend);app.with_default_widgets().with_default_systems();
        let state=fm::install(&mut app);
        for _ in 0..6 {app.tick();app.render().unwrap();}
        let rect=|id:&str| {let node=app.world.find_by_id(id).unwrap_or_else(||panic!("missing {id}"));app.world.get::<mirui::ui::ComputedRect>(node).unwrap().0};
        let cap=rect("sidebar_caption");let sidebar=rect("sidebar");let content=rect("files_content");
        assert_eq!((cap.x,cap.y),(Fixed::ZERO,Fixed::ZERO));
        assert_eq!(cap.w,sidebar.w);
        assert_eq!(sidebar.y,cap.h);
        assert_eq!(sidebar.y+sidebar.h,Fixed::from_int(height.into()));
        for place in [Place::Favourites,Place::Documents,Place::Downloads,Place::Projects,Place::Trash] {
            let entity=app.world.query::<fm::ui::sidebar::PlaceRow>().iter().find(|(_,row)|row.place==place).expect("place row").0;
            let row=app.world.get::<mirui::ui::ComputedRect>(entity).unwrap().0;
            assert!(row.x>=sidebar.x+Fixed::from_int(8));
            assert!(row.x+row.w<=sidebar.x+sidebar.w-Fixed::from_int(8));
        }
        for (badge,_) in app.world.query::<fm::ui::kit::CountBadge>().iter() {
            let badge=app.world.get::<mirui::ui::ComputedRect>(badge).unwrap().0;
            assert!(badge.w>Fixed::from_int(12),"count collapsed instead of reserving text width");
            assert!(badge.x+badge.w<=sidebar.x+sidebar.w-Fixed::from_int(14),"count clipped at divider");
        }
        let toolbar=rect("files_toolbar");let search=rect("toolbar_search_frame");
        assert!(search.w>Fixed::from_int(80));
        assert!(search.x>=toolbar.x && search.x+search.w<=toolbar.x+toolbar.w,"search outside toolbar at {width}");
        assert!(search.y+search.h<=toolbar.y+toolbar.h);
        let available=state.content_width()-32;
        assert!(state.grid_columns() as i32*fm::model::TILE_WIDTH+(state.grid_columns() as i32-1)*6<=available);
        assert!(content.w>Fixed::ZERO);
        if width==1400 {assert!(state.grid_columns()>8);}
        state.toggle_sidebar();
        for _ in 0..4 {app.tick();app.render().unwrap();}
        assert!(!state.sidebar_visible());
        let node=app.world.find_by_id("files_content").unwrap();
        let after=app.world.get::<mirui::ui::ComputedRect>(node).unwrap().0;
        assert!(after.w>content.w,"content failed to reclaim collapsed sidebar");
    }
}
