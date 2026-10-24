//! The window: title bar, toolbar, search and view switcher.

use mirui::prelude::*;
use mirui::render::font::FontManager;
use mirui::ui::OffscreenRender;
use mirui::ui::widgets::{Button, DropShadow, Icon, Placeholder, Text, TextEditing, TextInput};

use crate::data;
use crate::font::MEDIUM;
use crate::icons;
use crate::model::FmState;
use crate::text;
use crate::theme;
use crate::ui::controls;
use crate::ui::kit::{pad, pad_h, pad_v, segmented_control};
use mirui::input::event::focus::PointerFocus;
use mirui::ui::clipboard::ClipboardFinished;
use mirui_kit::field::{FieldContent, FieldFrame, focus as field_focus};
use mirui_kit::selection::clipboard_finished;

/// The unified toolbar.
#[compose]
pub fn toolbar(fm: model!(FmState)) -> Entity {
    ui! {
        Row (
            width: Dimension::percent(100),
            height: 52,
            bg_color: theme::TOOLBAR,
            align: AlignItems::Center,
            column_gap: 8,
            padding: pad(0, 12, 0, 12),
            border_radius: 12
        ) {
            back_button (fm)
            forward_button (fm)
            Column (width: 240, height: 34, row_gap: 0, justify: JustifyContent::Center) {
                Text (
                    text: ${ data::place_name(fm.place()) },
                    font: MEDIUM,
                    font_size: text::BODY,
                    height: 17,
                    text_color: ColorToken::OnSurface
                )
                Text (
                    text: "/Users/demo/Cloud",
                    font_size: text::SMALL,
                    height: 15,
                    text_color: ColorToken::OnSurfaceVariant
                )
            }
            View (grow: 1.0)
            palette_button (fm)
            new_folder_button (fm)
            segmented_control (fm)
            search_field (fm)
            options_button (fm)
            inspector_button (fm)
            theme_button (fm)
        }
    }
}

/// The window titlebar: the traffic lights live here, outside the content
/// toolbar, so the toolbar keeps a clean nav/actions split (place column left
/// of the grow spacer, actions to its right).
#[compose]
pub fn titlebar() -> Entity {
    ui! {
        Row (
            width: Dimension::percent(100),
            height: 22,
            bg_color: theme::TOOLBAR,
            align: AlignItems::Center,
            column_gap: 8,
            padding: pad(0, 12, 0, 12),
            border_radius: 12
        ) {
            traffic_lights ()
            View (grow: 1.0)
            Text (
                text: "Files",
                font: MEDIUM,
                font_size: text::BODY,
                text_color: ColorToken::OnSurface
            )
        }
    }
}

/// The three window buttons.
#[compose]
fn traffic_lights() -> Entity {
    ui! {
        Row (width: 66, height: 20, column_gap: 8, align: AlignItems::Center) {
            View (width: 12, height: 12, border_radius: 6, bg_color: theme::TRAFFIC_RED)
            View (width: 12, height: 12, border_radius: 6, bg_color: theme::TRAFFIC_YELLOW)
            View (width: 12, height: 12, border_radius: 6, bg_color: theme::TRAFFIC_GREEN)
        }
    }
}

/// The back chevron, live only when there is history.
#[compose]
fn back_button(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            id: "toolbar_back",
            width: 26,
            height: 26,
            border_radius: 6,
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            bg_color: ${ if fm.can_go_back() { theme::SELECTION } else { theme::CLEAR } }
        ) [
            HitTarget,
        ] {
            Icon (
                path: icons::CHEVRON_LEFT.clone(),
                color: ColorToken::OnSurfaceVariant,
                width: 15,
                height: 15
            )
        } on Tap {
            if fm.can_go_back() {
                fm.go_back();
            }
        }
    }
}

/// The forward chevron, shown for symmetry.
#[compose]
fn forward_button(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            width: 26,
            height: 26,
            border_radius: 6,
            align: AlignItems::Center,
            justify: JustifyContent::Center
        ) [
            HitTarget,
        ] {
            Icon (
                path: icons::CHEVRON_RIGHT.clone(),
                color: ColorToken::OnSurfaceVariant,
                width: 15,
                height: 15
            )
        }
    }
}

/// Opens the command palette.
#[compose]
fn palette_button(fm: model!(FmState)) -> Entity {
    ui! {
        Button (
            id: "toolbar_palette",
            width: 104,
            height: 26,
            border_radius: 6,
            padding: pad_v(0),
            normal_color: theme::CLEAR,
            pressed_color: theme::SELECTION
        ) {
            Row (
                width: Dimension::percent(100),
                height: 26,
                align: AlignItems::Center,
                column_gap: 6,
                padding: pad_h(8)
            ) {
                Icon (
                    path: icons::SEARCH.clone(),
                    color: ColorToken::OnSurfaceVariant,
                    width: 13,
                    height: 13
                )
                Text (
                    text: "Ctrl P",
                    grow: 1.0,
                    font_size: text::SMALL,
                    text_color: ColorToken::OnSurfaceVariant
                )
            }
        } on Tap { fm.open_palette(); }
    }
}

/// Creates a new folder.
#[compose]
fn new_folder_button(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            width: 118,
            height: 26
        ) {
            Button (
                id: "toolbar_new_folder",
                position: Position::Absolute,
                left: 0,
                top: 0,
                width: 118,
                height: 26,
                border_radius: 6,
                padding: pad_v(0),
                normal_color: theme::CLEAR,
                pressed_color: theme::SELECTION
            ) [
                OffscreenRender::new(),
            ] {
                Row (
                    width: Dimension::percent(100),
                    height: 26,
                    align: AlignItems::Center,
                    column_gap: 6,
                    padding: pad_h(8)
                ) {
                    Icon (
                        path: icons::FOLDER.clone(),
                        color: ColorToken::OnSurfaceVariant,
                        width: 13,
                        height: 13
                    )
                    Text (
                        text: "New Folder",
                        grow: 1.0,
                        font_size: text::SMALL,
                        text_color: ColorToken::OnSurfaceVariant
                    )
                }
            } on Tap { fm.new_folder_inline(); }
            View (
                id: "new_folder_shadow",
                position: Position::Absolute,
                left: 0,
                top: 0,
                width: 118,
                height: 26
            ) [
                DropShadow::new(id("toolbar_new_folder"))
                    .with_blur_radius(Fixed::from_int(10))
                    .with_offset(Fixed::ZERO, Fixed::from_int(3))
                    .with_opacity(80),
            ]
        }
    }
}

/// The search field.
///
/// The search field owns its whole 40px hit area, not just the text line.
#[compose]
fn search_field(fm: model!(FmState)) -> Entity {
    let text_height = res!(FontManager).with(|fonts| controls::field_height(&fonts));
    ui! {
        Row (
            id: "toolbar_search_frame",
            width: 224,
            height: 40,
            padding: pad_h(10),
            column_gap: 8,
            align: AlignItems::Center,
            bg_color: ColorToken::Surface,
            border_radius: 8,
            border_width: 1,
            border_color: ColorToken::Outline
        ) [
            FieldFrame::default(),
            mirui_kit::field::EditingSurface,
            HitTarget,
            PointerFocus::Preserve,
        ] {
            Icon (
                path: icons::SEARCH.clone(),
                color: ColorToken::OnSurfaceVariant,
                width: 16,
                height: 16
            )
            TextInput (
                id: "toolbar_search",
                width: 144,
                height: text_height,
                font: crate::font::MEDIUM,
                font_size: text::BODY,
                bg_color: theme::CLEAR,
                focus_border_color: theme::CLEAR
            ) [
                TextEditing::new(fm.document()),
                Placeholder("Search"),
                FieldContent,
                crate::sync::FieldScope(0),
            ] on TextEditing::Changed {
                let text = fm.document_text();
                fm.set_query(&text);
            } on ClipboardFinished(clipboard_finished::callback())
            View (
                width: 28,
                height: 32,
                align: AlignItems::Center,
                justify: JustifyContent::Center
            ) {
                if ${
                    {
                        let _ = fm.revision();
                        !fm.query_text().is_empty()
                    }
                } {
                    View (
                        id: "toolbar_search_clear",
                        width: 28,
                        height: 32,
                        align: AlignItems::Center,
                        justify: JustifyContent::Center
                    ) [
                        HitTarget,
                        PointerFocus::Preserve,
                    ] {
                        Icon (
                            path: icons::CLOSE.clone(),
                            width: 16,
                            height: 16,
                            color: ColorToken::OnSurfaceVariant
                        )
                    } on Tap { fm.clear_query(); }
                }
            }
        } on Tap(field_focus::callback())
    }
}

/// Opens the view options.
#[compose]
fn options_button(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            id: "toolbar_options",
            width: 28,
            height: 26,
            border_radius: 6,
            align: AlignItems::Center,
            justify: JustifyContent::Center
        ) [
            HitTarget,
        ] {
            Icon (path: icons::GEAR.clone(), color: ColorToken::OnSurface, width: 15, height: 15)
        } on Tap { fm.toggle_options(); }
    }
}

/// Shows or hides the inspector.
#[compose]
fn inspector_button(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            id: "toolbar_inspector",
            width: 28,
            height: 26,
            border_radius: 6,
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            bg_color: ${ if fm.inspector() { theme::SELECTION } else { theme::CLEAR } }
        ) [
            HitTarget,
        ] {
            Icon (path: icons::PANEL.clone(), color: ColorToken::OnSurface, width: 15, height: 15)
        } on Tap { fm.toggle_inspector(); }
    }
}

/// Light / dark toggle.
#[compose]
fn theme_button(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            id: "toolbar_theme",
            width: 28,
            height: 26,
            border_radius: 6,
            align: AlignItems::Center,
            justify: JustifyContent::Center
        ) [
            HitTarget,
        ] {
            Icon (path: icons::MOON.clone(), color: ColorToken::OnSurface, width: 15, height: 15)
        } on Tap { fm.toggle_theme(); }
    }
}
