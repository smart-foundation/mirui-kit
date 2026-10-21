//! The application shell: toolbar, sidebar, content, status bar.

use mirui::prelude::*;

use crate::model::FmState;
use crate::ui::chrome::toolbar;
use crate::ui::content::area;
use crate::ui::sidebar::panel;
use crate::ui::status::bar;

pub mod chrome;
pub mod content;
pub mod kit;
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
            toolbar (fm)
            Row (width: Dimension::percent(100), grow: 1.0) {
                panel (fm)
                area (fm)
            }
            bar (fm)
        }
    }
}
