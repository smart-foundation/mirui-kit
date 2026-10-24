//! The context menu and the drag badge.
//!
//! A popup anchored to the file it belongs to. The menu is a plain layer in the
//! window: a backdrop, and a panel placed at the recorded position. The
//! backdrop consumes outside taps and dismisses the menu. Dismissing a menu
//! never also activates the file or toolbar control underneath it.
//!
//! There are two menus. The file menu belongs to the file under the pointer,
//! the background menu to the empty space around the files. The model decides
//! which one is shown: [`MenuTarget::Item`] or [`MenuTarget::Background`].

use mirui::prelude::*;
use mirui::ui::OffscreenRender;
use mirui::ui::widgets::{Button, DropShadow, Icon, Text};

use crate::icons;
use mirui_kit::style;
use crate::model::{DialogKind, FmState, FmStateHandle, MenuTarget, Place, SortMode};
use crate::theme;
use crate::ui::hover::{HoverSurface, KindIcon};
use crate::ui::kit::{pad, pad_h, pad_v};

/// The items the file menu can offer, as stable ids.
pub const ITEM_OPEN: u8 = 0;
pub const ITEM_INFO: u8 = 1;
pub const ITEM_RENAME: u8 = 2;
pub const ITEM_DUPLICATE: u8 = 3;
pub const ITEM_PROJECTS: u8 = 4;
pub const ITEM_DOWNLOADS: u8 = 5;
pub const ITEM_TRASH: u8 = 6;
pub const ITEM_RESTORE: u8 = 7;

/// The items the background menu can offer, as stable ids.
pub const ITEM_NEW_FOLDER: u8 = 20;
pub const ITEM_SORT_NAME: u8 = 21;
pub const ITEM_SORT_DATE: u8 = 22;
pub const ITEM_SORT_SIZE: u8 = 23;
pub const ITEM_VIEW_OPTIONS: u8 = 24;

/// The menu layer: a backdrop plus the anchored panel.
#[compose]
pub fn menu_layer(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100)
        ) {
            View (
                position: Position::Absolute,
                left: 0,
                top: 0,
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                visible: ${ fm.menu_target().is_some() }
            ) [
                HitTarget
            ] on Tap {
                fm.close_menu();
            }
            Column (
                id: "menu_panel",
                position: Position::Absolute,
                left: ${ Dimension::px(fm.menu_x()) },
                top: ${ Dimension::px(fm.menu_y()) },
                width: 214,
                height: ${ fm.menu_height() },
                border_radius: 8,
                bg_color: ColorToken::Surface,
                border_color: theme::SEPARATOR,
                border_width: 1,
                padding: Padding::all(style::MENU_INSET),
                row_gap: style::MENU_GAP,
                clip_children: true,
                visible: ${ fm.menu_target().is_some() }
            ) [
                HitTarget
                OffscreenRender::new()
            ] {
                // The model decides which menu is shown: the file menu belongs
                // to the file under the pointer, the background menu to the
                // empty space around the files.
                if ${ matches!(fm.menu_target(), Some(MenuTarget::Item(_))) } {
                    menu_row (fm, ITEM_OPEN, "Open", "")
                    menu_row (fm, ITEM_INFO, "Get Info", "")
                    menu_row (fm, ITEM_RENAME, "Rename", "")
                    menu_row (fm, ITEM_DUPLICATE, "Duplicate", "")
                    separator ()
                    menu_row (fm, ITEM_PROJECTS, "Move to Projects", "")
                    menu_row (fm, ITEM_DOWNLOADS, "Move to Downloads", "")
                    separator ()
                    menu_row (fm, ITEM_TRASH, "Move to Trash", "")
                    menu_row (fm, ITEM_RESTORE, "Put Back", "")
                } else {
                    menu_row (fm, ITEM_NEW_FOLDER, "New Folder", "")
                    separator ()
                    menu_row (fm, ITEM_SORT_NAME, "Sort by Name", "")
                    menu_row (fm, ITEM_SORT_DATE, "Sort by Date", "")
                    menu_row (fm, ITEM_SORT_SIZE, "Sort by Size", "")
                    separator ()
                    menu_row (fm, ITEM_VIEW_OPTIONS, "View Options", "")
                }
            }
            View (
                id: "menu_shadow",
                position: Position::Absolute,
                left: ${ Dimension::px(fm.menu_x()) },
                top: ${ Dimension::px(fm.menu_y()) },
                width: 214,
                height: ${ fm.menu_height() }
            ) [
                DropShadow::new(id("menu_panel"))
                    .with_blur_radius(Fixed::from_int(10))
                    .with_offset(Fixed::ZERO, Fixed::from_int(3))
                    .with_opacity(80)
            ]
        }
    }
}

/// The badge that follows the pointer while a file is being dragged.
///
/// Shows the actual icon and label of the dragged item as a physical card
/// under the pointer, instead of an abstract text pill.
#[compose]
pub fn drag_badge(fm: model!(FmState)) -> Entity {
    let dragged = fm.dragging();
    ui! {
        View (
            position: Position::Absolute,
            left: ${ Dimension::px(fm.drag_x() + 12) },
            top: ${ Dimension::px(fm.drag_y() + 12) },
            visible: ${ fm.dragging().is_some() }
        ) {
            View (
                bg_color: ColorToken::Surface,
                border_radius: 6,
                border_width: 1,
                border_color: theme::SEPARATOR,
                padding: pad(8, 10, 8, 10),
            ) {
                Row (
                    align: AlignItems::Center,
                    column_gap: 8,
                ) {
                    View (width: 20, height: 20) [
                        KindIcon {
                            model: fm.clone(),
                            index: 0,
                            id: dragged,
                            size: 20,
                        }
                    ]
                    Text (
                        text: ${ { let _ = fm.dragging(); fm.drag_label() } },
                        font: crate::font::MEDIUM,
                    font_size: crate::font::BODY_SIZE,
                        text_color: ColorToken::OnSurface,
                    )
                }
            }
        }
    }
}

#[compose]
fn separator() -> Entity {
    ui! {
        View (width: Dimension::percent(100), height: 1, bg_color: theme::SEPARATOR)
    }
}

/// The icon shown beside a menu item label.
fn menu_icon(label: &'static str) -> &'static Path {
    match label {
        "Open" => &icons::DOCUMENT,
        "Get Info" => &icons::INFO,
        "Rename" => &icons::RENAME,
        "Duplicate" => &icons::DUPLICATE,
        "Move to Projects" => &icons::FOLDER,
        "Move to Downloads" => &icons::FOLDER,
        "Move to Trash" => &icons::TRASH,
        "Put Back" => &icons::FOLDER,
        "New Folder" => &icons::NEW_FOLDER,
        "Sort by Name" | "Sort by Date" | "Sort by Size" => &icons::SORT,
        "View Options" => &icons::GEAR,
        _ => &icons::MORE,
    }
}

#[component]
pub struct MenuRow { pub item: u8 }

/// One row of a menu.
///
/// The row rests in the colour of the panel, so it is invisible until the
/// pointer is on it: the surface paints the hover colour over it, and
/// mirui lifts the background of a hovered widget, so the row lights up even
/// without the surface.
#[compose]
fn menu_row(fm: model!(FmState), item: u8, label: &'static str, shortcut: &'static str) -> Entity {
    ui! {
        Button (
            width: Dimension::percent(100),
            height: style::MENU_ROW_HEIGHT,
            border_radius: 5,
            padding: pad_v(0),
            normal_color: ColorToken::Surface,
            pressed_color: theme::SELECTION
        ) [
            MenuRow {item},
            HoverSurface {
                hover: theme::MENU_HOVER.into(),
                pressed: theme::MENU_HOVER.into(),
                radius: Fixed::from_int(5)
            }
        ] {
            Row (
                width: Dimension::percent(100),
                height: style::MENU_ROW_HEIGHT,
                align: AlignItems::Center,
                column_gap: 8,
                padding: pad_h(8)
            ) {
                Icon (
                    path: menu_icon(label).clone(),
                    color: ColorToken::OnSurfaceVariant,
                    width: 16,
                    height: 16
                )
                Text (
                    text: label,
                    grow: 1.0,
                    font: crate::font::MEDIUM,
                    font_size: crate::font::BODY_SIZE,
                    text_color: ColorToken::OnSurface
                )
                Text (
                    text: shortcut,
                    font: crate::font::MEDIUM,
                    font_size: crate::font::BODY_SIZE,
                    text_color: ColorToken::OnSurfaceVariant
                )
            }
        } on Tap {
            // The target is read when the row is used: a composition argument
            // would be frozen when the tree is built.
            match fm.menu_target() {
                Some(MenuTarget::Item(id)) => run_file(&fm, id, item),
                Some(MenuTarget::Background) => run_background(&fm, item),
                None => {}
            }
        }
    }
}

/// What the file menu does.
fn run_file(fm: &FmStateHandle, id: u32, item: u8) {
    match item {
        ITEM_OPEN => {
            fm.select(id);
            fm.close_menu();
        }
        ITEM_INFO => {
            fm.select(id);
            fm.open_dialog(DialogKind::GetInfo);
        }
        ITEM_RENAME => {
            fm.select(id);
            let title = fm.entry_title(id);
            fm.set_rename_text(&title);
            fm.open_dialog(DialogKind::Rename);
        }
        ITEM_DUPLICATE => fm.duplicate(id),
        ITEM_TRASH => fm.trash(id),
        ITEM_RESTORE => fm.restore(id),
        ITEM_PROJECTS => fm.move_to(id, Place::Projects),
        ITEM_DOWNLOADS => fm.move_to(id, Place::Downloads),
        _ => {}
    }
}

/// What the background menu does.
fn run_background(fm: &FmStateHandle, item: u8) {
    match item {
        ITEM_NEW_FOLDER => {
            fm.set_folder_text("");
            fm.open_dialog(DialogKind::NewFolder);
        }
        ITEM_SORT_NAME => fm.set_sort(SortMode::Name),
        ITEM_SORT_DATE => fm.set_sort(SortMode::Date),
        ITEM_SORT_SIZE => fm.set_sort(SortMode::Size),
        ITEM_VIEW_OPTIONS => fm.toggle_options(),
        _ => {}
    }
}
