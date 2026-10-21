//! The window: title bar, toolbar, search and view switcher.

use mirui::prelude::*;
use mirui::ui::widgets::{Icon, Placeholder, Text, TextEditing, TextInput};

use crate::data;
use crate::icons;
use crate::model::{FmState, ViewMode};
use crate::theme;
use crate::ui::kit::{pad, pad_h, pad_v};

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
            padding: pad(0, 12, 0, 12)
        ) {
            traffic_lights ()
            View (width: 1, height: 22, bg_color: theme::SEPARATOR)
            back_button (fm)
            forward_button (fm)
            Column (width: 240, height: 34, row_gap: 0, justify: JustifyContent::Center) {
                Text (
                    text: ${ data::place_name(fm.place()) },
                    font_size: 13,
                    height: 17,
                    text_color: ColorToken::OnSurface
                )
                Text (
                    text: "Users demo Cloud",
                    font_size: 11,
                    height: 15,
                    text_color: ColorToken::OnSurfaceVariant
                )
            }
            View (grow: 1.0)
            view_switch (fm)
            search_field (fm)
            theme_button (fm)
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

/// Icon grid / list segmented control.
#[compose]
fn view_switch(fm: model!(FmState)) -> Entity {
    ui! {
        Row (
            width: 62,
            height: 26,
            border_radius: 7,
            bg_color: ColorToken::SurfaceVariant,
            column_gap: 2,
            padding: pad_v(2)
        ) {
            View (
                id: "toolbar_view_icons",
                width: 28,
                height: 22,
                border_radius: 5,
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                bg_color: ${ if fm.view() == ViewMode::Icons { ColorToken::Surface } else { theme::CLEAR } }
            ) [
                HitTarget
            ] {
                Icon (path: icons::GRID.clone(), color: ColorToken::OnSurface, width: 14, height: 14)
            } on Tap {
                fm.set_view(ViewMode::Icons);
            }
            View (
                id: "toolbar_view_list",
                width: 28,
                height: 22,
                border_radius: 5,
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                bg_color: ${ if fm.view() == ViewMode::List { ColorToken::Surface } else { theme::CLEAR } }
            ) [
                HitTarget
            ] {
                Icon (path: icons::LIST.clone(), color: ColorToken::OnSurface, width: 14, height: 14)
            } on Tap {
                fm.set_view(ViewMode::List);
            }
        }
    }
}

/// The search field.
#[compose]
fn search_field(fm: model!(FmState)) -> Entity {
    ui! {
        Row (
            width: 210,
            height: 26,
            border_radius: 7,
            bg_color: ColorToken::Surface,
            column_gap: 4,
            align: AlignItems::Center,
            padding: pad_h(6)
        ) {
            Icon (path: icons::SEARCH.clone(), color: ColorToken::OnSurfaceVariant, width: 13, height: 13)
            TextInput (width: 176, height: 20) [
                TextEditing::new(fm.document()),
                Placeholder("Search"),
            ] on TextEditing::Changed {
                let text = fm.document_text();
                fm.set_query(&text);
            }
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
