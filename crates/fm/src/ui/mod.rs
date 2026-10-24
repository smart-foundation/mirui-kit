//! The application shell: toolbar, sidebar, content, inspector, status bar,
//! and the layers that float above them — context menu, modal sheets, the
//! command palette and the drag badge.

use mirui::prelude::*;

use crate::model::FmState;
use crate::theme;
use crate::ui::chrome::{sidebar_caption, toolbar};
use crate::ui::content::area;
use crate::ui::dialog::dialog_layer;
use crate::ui::inspector::panel;
use crate::ui::menu::{drag_badge, menu_layer};
use crate::ui::options::options_layer;
use crate::ui::palette::palette_layer;
use crate::ui::sidebar::sidebar;
use crate::ui::status::{bar, toast};

pub mod chrome;
pub mod content;
pub mod controls;
pub mod dialog;
pub mod hover;
pub mod inspector;
pub mod kindicon;
pub mod kit;
pub mod menu;
pub mod options;
pub mod palette;
pub mod scrollbar;
pub mod sidebar;
pub mod status;

/// The root of the file manager.
#[compose]
pub fn shell(fm: model!(FmState)) -> Entity {
    ui! {
        Column (
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            bg_color: ColorToken::Surface
        ) {
            Row (id:"chrome_frame",width:Dimension::percent(100),height:${{let _=fm.viewport();fm.header_height()}},bg_color:theme::TOOLBAR) {
                sidebar_caption (fm)
                toolbar (fm)
            }
            Row (id:"content_frame",width:Dimension::percent(100),grow:1.0) {
                sidebar (fm)
                Column (grow:1.0,shrink:1.0,height:Dimension::percent(100),min_width:0) {
                    Row (width:Dimension::percent(100),grow:1.0) {
                        area (fm)
                        panel (fm)
                    }
                    bar (fm)
                }
            }
            options_layer (fm)
            // The message sits below the menu: when both are on screen the
            // menu wins the hit test, so its dismissal is not swallowed.
            toast (fm)
            menu_layer (fm)
            dialog_layer (fm)
            palette_layer (fm)
            drag_badge (fm)
        }
    }
}
