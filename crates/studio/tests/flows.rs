//! Full Mirui tests. Unlike the dependency-free core suite, these require crates.io.
use mirui::{prelude::*, render::texture::ColorFormat, surface::framebuf::FramebufSurface};
use studio::{commands::Command, model::StudioHandle};
use studio_core::{Route, Surface};
fn flush(_: &[u8], _: mirui::types::PhysicalRect) {}
fn app() -> (
    App<FramebufSurface<fn(&[u8], mirui::types::PhysicalRect)>, mirui::render::SwRendererFactory>,
    StudioHandle,
) {
    let mut app = App::new(FramebufSurface::with_format(
        1400,
        900,
        ColorFormat::RGBA8888,
        flush as fn(&[u8], mirui::types::PhysicalRect),
    ));
    app.with_default_widgets().with_default_systems();
    let state = studio::install(&mut app);
    (app, state)
}
fn press(state: &StudioHandle, key: Command) {
    state.execute(key as u16);
}
#[test]
fn every_route_can_render_with_real_widgets() {
    let (mut app, state) = app();
    for route in Route::ALL {
        state.execute(route as u16 + 1);
        app.tick();
        app.render().expect("route frame");
        assert_eq!(state.route(), route);
    }
}
#[test]
fn palette_close_is_not_filtered_as_an_unavailable_command() {
    let (_app, state) = app();
    press(&state, Command::Palette);
    press(&state, Command::Close);
    assert!(state.surface().is_none());
}
#[test]
fn sampled_background_actions_cannot_pass_an_opening_palette() {
    let (_app, state) = app();
    press(&state, Command::Collections);
    state.actions().activate(Command::Palette as u16, false);
    state.actions().activate(Command::NewFolder as u16, false);
    state.sync();
    assert_eq!(state.surface(), Some(Surface::Palette));
    assert!(state.editing().is_none());
}
#[test]
fn input_gate_disables_the_entire_background_branch() {
    let (mut app, state) = app();
    app.tick();
    app.render().expect("initial frame");
    press(&state, Command::Palette);
    app.tick();
    app.render().expect("modal frame");
    for (entity, gate) in app.world.query::<mirui_kit::InputGate>().iter() {
        assert_eq!(
            matches!(
                app.world.get::<mirui::ui::UserState>(entity),
                Some(mirui::ui::UserState::Disabled)
            ),
            !gate.overlay
        );
    }
    press(&state, Command::Close);
    app.tick();
    app.render().expect("dismissed frame");
    for (entity, gate) in app.world.query::<mirui_kit::InputGate>().iter() {
        assert_eq!(
            matches!(
                app.world.get::<mirui::ui::UserState>(entity),
                Some(mirui::ui::UserState::Disabled)
            ),
            gate.overlay
        );
    }
}
#[test]
fn an_unsaved_note_can_be_cancelled_then_saved_on_departure() {
    let (_app, state) = app();
    press(&state, Command::Workbench);
    state.note().set_text("A new session note").unwrap();
    press(&state, Command::Collections);
    assert_eq!(state.surface(), Some(Surface::Unsaved));
    press(&state, Command::Close);
    assert_eq!(state.route(), Route::Workbench);
    press(&state, Command::Collections);
    press(&state, Command::SaveLeave);
    assert_eq!(state.route(), Route::Collections);
    assert_eq!(state.note().read(str::to_owned), "A new session note");
}
#[test]
fn separate_apps_do_not_share_action_queues_or_documents() {
    let (_a, first) = app();
    let (_b, second) = app();
    first.note().set_text("First app only").unwrap();
    first.actions().activate(Command::Help as u16, false);
    first.sync();
    assert!(second.surface().is_none());
    assert_ne!(second.note().read(str::to_owned), "First app only");
}
#[test]
fn an_empty_palette_cannot_execute_a_hidden_command() {
    let (_app, state) = app();
    press(&state, Command::Palette);
    state.palette().set_text("no matching action xyz").unwrap();
    state.execute(90);
    assert_eq!(state.surface(), Some(Surface::Palette));
    assert!(!state.error().is_empty());
}
#[test]
fn a_filtered_folder_still_allows_visible_inline_creation() {
    let (_app, state) = app();
    press(&state, Command::Collections);
    state.query().set_text("no matches").unwrap();
    press(&state, Command::NewFolder);
    let id = state.editing().expect("inline draft");
    assert!(state.rows().contains(&id));
    state.name().set_text("New sample").unwrap();
    press(&state, Command::CommitName);
    assert_eq!(state.asset_label(id), "New sample");
}
#[test]
fn registry_entries_are_unique_and_have_real_icon_paths() {
    for (i, entry) in studio::commands::COMMANDS.iter().enumerate() {
        assert!(!entry.label.is_empty());
        let _typed_real_path: &mirui::render::path::Path = entry.icon;
        for other in &studio::commands::COMMANDS[i + 1..] {
            assert_ne!(entry.command as u16, other.command as u16);
        }
    }
}
