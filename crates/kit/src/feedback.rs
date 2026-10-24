//! Themed feedback compositions. Live announcements require the host semantics
//! bridge; visual presentation alone is not a screen-reader announcement.
use crate::{Actions, font, style, theme, widgets::text_size};
use alloc::string::String;
use mirui::prelude::*;
use mirui::ui::widgets::Text;
#[derive(Clone, Copy, Debug, Default)]
pub enum Severity {
    #[default]
    Information,
    Success,
    Warning,
    Error,
}
impl Severity {
    pub fn accent(self) -> ColorToken {
        match self {
            Self::Error => ColorToken::Error,
            Self::Success => theme::SUCCESS,
            Self::Warning => theme::WARNING,
            Self::Information => ColorToken::Primary,
        }
    }
}
#[compose]
pub fn badge(actions: model!(Actions), label: String) -> Entity {
    ui! {View (padding:Padding::all(4),bg_color:theme::BADGE,border_radius:style::RADIUS) {Text (text:label.clone(),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::CAPTION_SIZE)},text_color:theme::BADGE_TEXT)}}
}
#[compose]
pub fn alert(
    actions: model!(Actions),
    severity: Severity,
    title: String,
    description: String,
) -> Entity {
    ui! {Column (width:Dimension::percent(100),padding:Padding::all(12),row_gap:style::GAP,bg_color:ColorToken::SurfaceVariant,border_color:severity.accent(),border_width:2,border_radius:style::RADIUS) [crate::sizing::ContentFit] {
        Text (text:title.clone(),font:font::SEMIBOLD,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)
        Text (text:description.clone(),paragraph:mirui::ui::widgets::ParagraphStyle::default(),width:Dimension::percent(100),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)
    }}
}
#[compose]
pub fn empty_state(actions: model!(Actions), title: String, description: String) -> Entity {
    ui! {Column (width:Dimension::percent(100),padding:Padding::all(24),row_gap:style::GAP) [crate::sizing::ContentFit] {
        Text (text:title.clone(),font:font::SEMIBOLD,font_size:${text_size(actions.text_scale(),font::SECTION_SIZE)},text_color:ColorToken::OnSurface)
        Text (text:description.clone(),paragraph:mirui::ui::widgets::ParagraphStyle::default(),width:Dimension::percent(100),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurfaceVariant)
    }}
}
#[model]
#[derive(Default)]
pub struct Progress {
    #[observe]
    pub value: u16,
    #[observe]
    pub indeterminate: bool,
}
#[model]
impl Progress {
    pub fn set(&mut self, value: u16) {
        self.value = value.min(10000);
        self.indeterminate = false;
    }
    pub fn pending(&mut self) {
        self.indeterminate = true;
    }
}
/// Basis points avoid float drift. Pending is explicitly labelled rather than
/// simulating determinate progress with an invented percentage.
#[compose]
pub fn progress(actions: model!(Actions), state: model!(Progress), label: String) -> Entity {
    ui! {Column (width:Dimension::percent(100),row_gap:style::GAP) [crate::sizing::ContentFit] {
        Text (text:label.clone(),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)
        View (width:Dimension::percent(100),height:8,bg_color:ColorToken::SurfaceVariant,border_radius:4) {
            View (width:${Dimension::percent((state.value()/100) as i32)},height:8,bg_color:ColorToken::Primary,border_radius:4,visible:${!state.indeterminate()})
        }
        Text ("Working…",visible:${state.indeterminate()},font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurfaceVariant)
    }}
}
#[compose]
pub fn separator() -> Entity {
    ui! {View (width:Dimension::percent(100),height:1,bg_color:theme::SEPARATOR)}
}
