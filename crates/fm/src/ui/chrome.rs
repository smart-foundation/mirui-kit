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
use crate::ui::controls::{self, FieldFocus};
use crate::ui::kit::{pad, pad_h, pad_v, segmented_control};

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
            border_radius: 12,
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
            border_radius: 12,
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
            HitTarget
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
            HitTarget
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
                Icon (path: icons::SEARCH.clone(), color: ColorToken::OnSurfaceVariant, width: 13, height: 13)
                    Text (
                        text: "Ctrl P",
                    grow: 1.0,
                    font_size: text::SMALL,
                    text_color: ColorToken::OnSurfaceVariant
                )
            }
        } on Tap {
            fm.open_palette();
        }
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
                OffscreenRender::new()
            ] {
                Row (
                    width: Dimension::percent(100),
                    height: 26,
                    align: AlignItems::Center,
                    column_gap: 6,
                    padding: pad_h(8)
                ) {
                    Icon (path: icons::FOLDER.clone(), color: ColorToken::OnSurfaceVariant, width: 13, height: 13)
                    Text (
                        text: "New Folder",
                        grow: 1.0,
                        font_size: text::SMALL,
                        text_color: ColorToken::OnSurfaceVariant
                    )
                }
            } on Tap {
                fm.new_folder_inline();
            }
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
                    .with_opacity(80)
            ]
        }
    }
}

/// The search field.
///
/// The field is as tall as the line of type it holds: mirui lays the text of
/// a field out from a fixed two pixel inset and never centres it, so the
/// height follows the metrics of the typeface.
#[compose]
fn search_field(fm: model!(FmState)) -> Entity {
    let text_height = res!(FontManager).with(|fonts| controls::field_height(&fonts));
    ui! {
        Row (
            width: 190,
            height: 26,
            border_radius: 7,
            bg_color: ColorToken::Surface,
            column_gap: 4,
            align: AlignItems::Center,
            padding: pad_h(6)
        ) {
            Icon (path: icons::SEARCH.clone(), color: ColorToken::OnSurfaceVariant, width: 13, height: 13)
            TextInput (
                id: "toolbar_search",
                width: 156,
                height: text_height,
                font_size: text::SMALL,
                bg_color: ColorToken::Surface,
                border_radius: 4,
                // mirui draws a square ring of its own when the field has the
                // focus. The ring is given the colour of the field, so it
                // cannot be seen: the application draws its own, rounded.
                focus_border_color: ColorToken::Surface
            ) [
                TextEditing::new(fm.document()),
                Placeholder("Search"),
                FieldFocus {
                    color: theme::FOCUS.into(),
                    radius: Fixed::from_int(8),
                    width: Fixed::from_int(3),
                }
            ] on TextEditing::Changed {
                let text = fm.document_text();
                fm.set_query(&text);
            }
        }
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
            HitTarget
        ] {
            Icon (path: icons::GEAR.clone(), color: ColorToken::OnSurface, width: 15, height: 15)
        } on Tap {
            fm.toggle_options();
        }
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
            HitTarget
        ] {
            Icon (path: icons::PANEL.clone(), color: ColorToken::OnSurface, width: 15, height: 15)
        } on Tap {
            fm.toggle_inspector();
        }
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
            HitTarget
        ] {
            Icon (path: icons::MOON.clone(), color: ColorToken::OnSurface, width: 15, height: 15)
        } on Tap {
            fm.toggle_theme();
        }
    }
}
