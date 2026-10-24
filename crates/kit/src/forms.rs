//! Form compositions with persistent labels and explicit validation feedback.
use crate::behavior::forms::{NumberError, NumberState};
use crate::{
    Actions, Editor, FocusItem, font, icons, style, theme,
    widgets::{FocusRing, field, text_size},
};
use alloc::string::{String, ToString};
use mirui::prelude::*;
use mirui::ui::widgets::{Button, Icon, Text, TextDocument, TextEditing};
#[model]
#[derive(Default)]
pub struct Validation {
    pub message: String,
    pub description: String,
    #[observe]
    pub revision: u64,
    #[observe]
    pub required: bool,
}
#[model]
impl Validation {
    pub fn set_error(&mut self, message: String) {
        self.message = message;
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn describe(&mut self, description: String) {
        self.description = description;
        self.revision = self.revision.wrapping_add(1);
    }
}
#[compose]
pub fn form_field(
    actions: model!(Actions),
    document: model!(TextDocument),
    validation: model!(Validation),
    key: u16,
    label: String,
    placeholder: &'static str,
    overlay: bool,
    submit: u16,
) -> Entity {
    ui! {Column (width:Dimension::percent(100),row_gap:style::GAP) [crate::sizing::ContentFit] {
        Text (text:label.clone(),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)
        Text ("Required",visible:${validation.required()},font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::CAPTION_SIZE)},text_color:ColorToken::OnSurfaceVariant)
        field (actions,document,key,placeholder,overlay,submit)
        Text (text:${validation.description()},visible:${!validation.description().is_empty()},width:Dimension::percent(100),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurfaceVariant)
        Text (text:${validation.message()},visible:${!validation.message().is_empty()},width:Dimension::percent(100),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::Error)
    }}
}
#[model]
pub struct NumericEntry {
    pub document: model!(owned TextDocument),
    state: NumberState,
    #[observe]
    pub value: i64,
    pub error: String,
    #[observe]
    pub error_revision: u64,
}
impl NumericEntry {
    pub fn new(value: i64, min: i64, max: i64, step: i64) -> Result<Self, NumberError> {
        let state = NumberState::new(value, min, max, step)?;
        let mut document = TextDocument::new();
        document.set_capacity(32).expect("numeric input capacity");
        document
            .set_text(&value.to_string())
            .expect("i64 fits input");
        Ok(Self {
            document: document.into(),
            state,
            value,
            error: String::new(),
            error_revision: 0,
        })
    }
}
#[model]
impl NumericEntry {
    pub fn commit(&mut self) {
        self.error_revision = self.error_revision.wrapping_add(1);
        let text = self.document.read(str::to_string);
        match self.state.commit(&text) {
            Ok(_) => {
                self.value = self.state.value();
                self.error.clear();
            }
            Err(error) => {
                self.error = match error {
                    NumberError::Empty => "Enter a number.",
                    NumberError::Invalid => "Enter a whole number.",
                    NumberError::OutOfRange => "The number is outside the allowed range.",
                    NumberError::InvalidBounds => "Invalid number bounds.",
                }
                .into()
            }
        }
    }
    pub fn step(&mut self, direction: i32, page: bool) {
        self.error_revision = self.error_revision.wrapping_add(1);
        self.state.step(direction, page);
        self.value = self.state.value();
        self.error.clear();
        let text = self.value.to_string();
        self.document.set_text(&text).expect("i64 fits input");
    }
}
/// Native editable numeric text, separate draft/committed value, inline errors,
/// Up/Down stepping and PageUp/PageDown coarse steps. Not the slider helper.
#[compose]
pub fn number_field(
    actions: model!(Actions),
    entry: model!(NumericEntry),
    key: u16,
    label: String,
    overlay: bool,
) -> Entity {
    assert!(
        key <= u16::MAX - 2,
        "number_field reserves three consecutive focus IDs"
    );
    ui! {Column (width:Dimension::percent(100),row_gap:style::GAP) [crate::sizing::ContentFit] {
        Text (text:label.clone(),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)
        Row (width:Dimension::percent(100),column_gap:style::GAP) {
            View (grow:1.0,height:${text_size(actions.text_scale(),44)},padding:Padding::all(10),bg_color:ColorToken::SurfaceVariant,border_width:1,border_color:${if entry.error().is_empty() {ColorToken::Outline} else {ColorToken::Error}},border_radius:style::RADIUS) [crate::field::FieldFrame::default(),crate::field::EditingSurface,HitTarget,mirui::input::event::focus::PointerFocus::Preserve] {
                Editor (document:Some(entry.document()),width:Dimension::percent(100),height:24,font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)}) [crate::field::FieldContent,FocusItem{overlay,order:key}] on TextEditing::Submitted {if actions.modal()==overlay {entry.commit();}} on Key(|event:event!(Key)| {
                    if !event.pressed || actions.modal()!=overlay {return false;}
                    let direction=match event.physical {Some(PhysicalKey::ArrowUp)|Some(PhysicalKey::PageUp)=>1,Some(PhysicalKey::ArrowDown)|Some(PhysicalKey::PageDown)=>-1,_=>0};
                    if direction!=0 {entry.step(direction,matches!(event.physical,Some(PhysicalKey::PageUp)|Some(PhysicalKey::PageDown)));return true;} false
                }) on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);if !event.focused {entry.commit();}})
            } on Tap(crate::field::focus::callback())
            number_step (actions,entry,key+1,overlay,-1)
            number_step (actions,entry,key+2,overlay,1)
        }
        Text (text:${entry.error()},visible:${!entry.error().is_empty()},width:Dimension::percent(100),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:ColorToken::Error)
    }}
}
#[compose]
fn number_step(
    actions: model!(Actions),
    entry: model!(NumericEntry),
    key: u16,
    overlay: bool,
    direction: i32,
) -> Entity {
    ui! {Button (width:${text_size(actions.text_scale(),44)},height:${text_size(actions.text_scale(),44)},normal_color:ColorToken::SurfaceVariant,pressed_color:theme::PRESSED,border_width:2,border_color:ColorToken::Outline,border_radius:style::RADIUS) [Focusable,FocusItem{overlay,order:key},FocusRing{actions:actions.clone()}] {
        Icon (path:if direction<0 {icons::MINUS.clone()} else {icons::PLUS.clone()},width:${text_size(actions.text_scale(),18)},height:${text_size(actions.text_scale(),18)},color:ColorToken::OnSurface)
    } on Tap {if actions.modal()==overlay {entry.step(direction,false);}} on Key(|event:event!(Key)| {if event.pressed && event.repeat!=Some(true) && (event.code==13||event.physical==Some(PhysicalKey::Space)) && actions.modal()==overlay {entry.step(direction,false);return true;} false}) on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);})}
}

impl ValidationHandle {
    pub fn message(&self) -> String {
        let _ = self.revision();
        mirui::core::model::ModelHandle::read(self, |data| data.message.clone())
    }
}
impl ValidationOwned {
    pub fn message(&self) -> String {
        mirui::core::model::OwnedModel::read(self, |data| data.message.clone())
    }
}

impl ValidationHandle {
    pub fn description(&self) -> String {
        let _ = self.revision();
        mirui::core::model::ModelHandle::read(self, |data| data.description.clone())
    }
}
impl ValidationOwned {
    pub fn description(&self) -> String {
        mirui::core::model::OwnedModel::read(self, |data| data.description.clone())
    }
}

impl NumericEntryHandle {
    pub fn error(&self) -> String {
        let _ = self.error_revision();
        mirui::core::model::ModelHandle::read(self, |data| data.error.clone())
    }
}
impl NumericEntryOwned {
    pub fn error(&self) -> String {
        mirui::core::model::OwnedModel::read(self, |data| data.error.clone())
    }
}
