//! Controls used by both Studio chrome and live specimens.
use crate::{font, theme};
use alloc::{string::String, vec::Vec};
use mirui::input::event::focus::PointerFocus;
use mirui::prelude::*;
use mirui::render::path::Path;
use mirui::ui::clipboard::ClipboardFinished;
use mirui::ui::{
    widgets::{
        Button, Icon, Placeholder, Slider, Switch, Text, TextDocument, TextEditing, TextInput,
    },
};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ActionError {WrongLayer,Backpressured}

/// Bounded per-instance action channel, with explicit overlay ownership.
#[model]
#[derive(Default)]
pub struct Actions {
    pending: Vec<(u16, bool)>,
    applied_layer: Option<bool>,
    saved_focus: Option<Entity>,
    #[observe]
    pub modal: bool,
    #[observe]
    pub focused: Option<u16>,
    #[observe]
    pub focus_revision: u64,
    #[observe]
    pub text_scale: u16,
}
#[model]
impl Actions {
    pub fn activate(&mut self, id: u16, overlay: bool) -> Result<(),ActionError> {
        if overlay!=self.modal {return Err(ActionError::WrongLayer);}
        if self.pending.len()>=32 {return Err(ActionError::Backpressured);}
        self.pending.push((id,overlay));Ok(())
    }
    pub fn applied_layer(&self) -> Option<bool> {
        self.applied_layer
    }
    pub fn saved_focus(&self) -> Option<Entity> {
        self.saved_focus
    }
    pub fn accept_layer(&mut self, focus: Option<Entity>) {
        if self.modal {
            self.saved_focus = focus;
        }
        self.applied_layer = Some(self.modal);
    }
    #[model(local)]
    pub fn take(&mut self) -> Vec<(u16, bool)> {
        core::mem::take(&mut self.pending)
    }
    pub fn set_modal(&mut self, modal: bool) {
        self.modal = modal;
    }
    pub fn focus(&mut self, id: u16, focused: bool) {
        self.focus_revision = self.focus_revision.wrapping_add(1);
        if focused {
            self.focused = Some(id);
        } else if self.focused == Some(id) {
            self.focused = None;
        }
    }
    pub fn set_scale(&mut self, scale: u16) {
        self.text_scale = scale.clamp(100, 150);
    }
}
#[model]
pub struct Number {
    #[observe]
    pub value: Fixed,
}
#[model]
impl Number {
    pub fn set(&mut self, value: Fixed) {
        self.value = value.clamp(Fixed::ZERO, Fixed::from_int(100));
    }
}
#[model]
#[derive(Default)]
pub struct Toggle {
    #[observe]
    pub checked: bool,
}
#[model]
impl Toggle {
    pub fn set(&mut self, checked: bool) {
        self.checked = checked;
    }
    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}

/// A single mount's input boundary; Disabled propagates to every descendant.
#[component]
pub struct InputGate {
    pub actions: model!(Actions),
    pub overlay: bool,
}
#[view(component = InputGate, name = "KitInputGate")]
pub fn gate_view() {}
#[system]
pub fn gate_sync(
    gates: query!(Entity, InputGate),
    states: query!(crate::availability::InputBlocked),
    commands: commands!(),
) {
    for (entity, gate) in gates {
        let disabled = gate.actions.modal() != gate.overlay;
        if disabled && states.get(entity).is_none() {
            commands.set::<crate::availability::InputAvailability>(entity, false);
        } else if !disabled && states.get(entity).is_some() {
            commands.set::<crate::availability::InputAvailability>(entity, true);
        }
    }
}
/// Explicit keyboard order, scoped to the active input layer.
#[component]
pub struct FocusItem {
    pub overlay: bool,
    pub order: u16,
}

/// Mirui's native document/editing pair, attached using the upstream editor pattern.
#[component]
#[derive(Default)]
pub struct Editor {
    pub document: model!(optional TextDocument),
}
#[event]
fn attach_editor(
    node: entity!(),
    rows: query!(Editor),
    editing: query!(TextEditing),
    appearance: query!(TextInput),
    commands: commands!(),
) {
    let Some(editor) = rows.get(node) else {
        return;
    };
    if editing.get(node).is_none() {
        commands.insert(
            node,
            editor
                .document
                .clone()
                .map(TextEditing::new)
                .unwrap_or_default(),
        );
    }
    if appearance.get(node).is_none() {
        commands.insert(
            node,
            TextInput::new()
                .with_cursor_color(theme::FOCUS)
                .with_focus_border_color(theme::FOCUS),
        );
    }
}
#[view(component = Editor, attach = attach_editor::callback())]
pub fn editor_view() {}

#[compose]
pub fn action(
    actions: model!(Actions),
    key: u16,
    label: String,
    icon: &'static Path,
    overlay: bool,
) -> Entity {
    ui! {
        Button (
            height: ${ text_size(actions.text_scale(), 40) },
            min_width: 40,
            width: Dimension::percent(100),
            padding: Padding::all(8),
            normal_color: ColorToken::SurfaceVariant,
            pressed_color: theme::PRESSED,
            border_radius: 8,
            border_width: 2,
            border_color: ColorToken::Outline
        ) [
            FocusRing {
                actions: actions.clone(),
            },
            Focusable,
            FocusItem { overlay, order: key },
        ] {
            Row (
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::Center,
                column_gap: 10
            ) {
                Icon (path: icon.clone(), width: 18, height: 18, color: ColorToken::OnSurface)
                Text (
                    text: label.clone(),
                    font: font::MEDIUM,
                    font_size: ${ text_size(actions.text_scale(), font::BODY_SIZE) },
                    text_color: ColorToken::OnSurface
                )
            }
        } on Tap { let _=actions.activate(key, overlay); } on Key(|event: event!(Key)| {
            if event.pressed && event.repeat != Some(true)
                && (event.code == 13 || event.physical == Some(PhysicalKey::Space))
            {
                let _=actions.activate(key, overlay);
                return true;
            }
            false
        }) on FocusChanged(|event: event!(FocusChanged)| {
            actions.focus(key, event.focused);
        })
    }
}
#[compose]
pub fn field(
    actions: model!(Actions),
    document: model!(TextDocument),
    key: u16,
    placeholder: &'static str,
    overlay: bool,
    submit: u16,
) -> Entity {
    ui! {
        View (
            width: Dimension::percent(100),
            height: ${ text_size(actions.text_scale(), 44) },
            padding: Padding {
                top: 0.into(),
                right: 10.into(),
                bottom: 0.into(),
                left: 10.into(),
            },
            align: AlignItems::Center,
            bg_color: ColorToken::SurfaceVariant,
            border_width: 1,
            border_color: ColorToken::Outline,
            border_radius: 8
        ) [
            crate::field::FieldFrame::default(),
            crate::field::EditingSurface,
            HitTarget,
            PointerFocus::Preserve,
        ] {
            Editor (
                document: Some(document),
                width: Dimension::percent(100),
                height: 24,
                font: font::MEDIUM,
                font_size: ${ text_size(actions.text_scale(), font::BODY_SIZE) }
            ) [
                crate::field::FieldContent,
                TextInput::new()
                    .with_cursor_color(theme::FOCUS)
                    .with_focus_border_color(theme::CLEAR),
                Placeholder(placeholder),
                FocusItem { overlay, order: key },
            ] on TextEditing::Submitted {
                if submit != 0 {
                    let _=actions.activate(submit, overlay);
                }
            } on FocusChanged(|event: event!(FocusChanged)| {
                actions.focus(key, event.focused);
            }) on ClipboardFinished(crate::selection::clipboard_finished::callback())
        } on Tap(crate::field::focus::callback())
    }
}

#[compose]
pub fn number(actions: model!(Actions), number: model!(Number), key: u16) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 4) {
            Slider (
                min: 0,
                max: 100,
                value: ${ number.value() },
                width: Dimension::percent(100),
                height: 36
            ) on Slider::ValueChanged(|event: event!(mirui::ui::widgets::slider::ValueChanged)| {
                number.set(event.new);
            })
            Row (width: Dimension::percent(100), height: 40, column_gap: 8) {
                Button (
                    "- 5",
                    width: 80,
                    height: 40,
                    font: font::MEDIUM,
                    font_size: ${ text_size(actions.text_scale(), font::BODY_SIZE) },
                    border_width: 2,
                    border_color: ColorToken::Outline
                ) [
                    FocusRing {
                        actions: actions.clone(),
                    },
                    Focusable,
                    FocusItem {
                        overlay: false,
                        order: key,
                    },
                ] on Tap { number.set(number.value() - Fixed::from_int(5)); } on Key(|event: event!(Key)| {
                    if event.pressed && event.repeat != Some(true)
                        && (event.code == 13 || event.physical == Some(PhysicalKey::Space))
                    {
                        number.set(number.value() - Fixed::from_int(5));
                        return true;
                    }
                    false
                }) on FocusChanged(|event: event!(FocusChanged)| {
                    actions.focus(key, event.focused);
                })
                Button (
                    "+ 5",
                    width: 80,
                    height: 40,
                    font: font::MEDIUM,
                    font_size: ${ text_size(actions.text_scale(), font::BODY_SIZE) },
                    border_width: 2,
                    border_color: ColorToken::Outline
                ) [
                    FocusRing {
                        actions: actions.clone(),
                    },
                    Focusable,
                    FocusItem {
                        overlay: false,
                        order: key + 1,
                    },
                ] on Tap { number.set(number.value() + Fixed::from_int(5)); } on Key(|event: event!(Key)| {
                    if event.pressed && event.repeat != Some(true)
                        && (event.code == 13 || event.physical == Some(PhysicalKey::Space))
                    {
                        number.set(number.value() + Fixed::from_int(5));
                        return true;
                    }
                    false
                }) on FocusChanged(|event: event!(FocusChanged)| {
                    actions.focus(key + 1, event.focused);
                })
                Text (
                    text: ${ alloc::format!("{}%", number.value().to_int()) },
                    font: font::MEDIUM,
                    font_size: ${ text_size(actions.text_scale(), font::BODY_SIZE) }
                )
            }
        }
    }
}
#[compose]
pub fn toggle(
    actions: model!(Actions),
    toggle: model!(Toggle),
    key: u16,
    label: &'static str,
) -> Entity {
    ui! {
        Row (
            width: Dimension::percent(100),
            height: 40,
            align: AlignItems::Center,
            column_gap: 12
        ) {
            Switch (
                on: ${ toggle.checked() },
                width: 44,
                height: 26,
                border_radius: 13,
                border_width: 2,
                border_color: ColorToken::Outline
            ) [
                FocusRing {
                    actions: actions.clone(),
                },
                Focusable,
                FocusItem {
                    overlay: false,
                    order: key,
                },
            ] on Switch::Toggled(|event: event!(mirui::ui::widgets::switch::Toggled)| {
                toggle.set(event.now);
            }) on Key(|event: event!(Key)| {
                if event.pressed && event.repeat != Some(true)
                    && (event.code == 13 || event.physical == Some(PhysicalKey::Space))
                {
                    toggle.toggle();
                    return true;
                }
                false
            }) on FocusChanged(|event: event!(FocusChanged)| {
                actions.focus(key, event.focused);
            })
            Text (
                label,
                font: font::MEDIUM,
                font_size: ${ text_size(actions.text_scale(), font::BODY_SIZE) },
                text_color: ColorToken::OnSurface
            )
        }
    }
}

/// Focus feedback is tied to the actual entity, not a command ID shared by specimens.
#[component]
pub struct FocusRing {
    pub actions: model!(Actions),
}
#[view(component=FocusRing,name="KitFocusRing",priority=90)]
pub fn focus_ring(
    _ring: component!(),
    node: entity!(),
    _actions: model!(component.actions, watch(focus_revision)),
    focus: res!(mirui::input::event::focus::FocusState),
    theme: res!(Theme),
    paint: paint!(),
) {
    if focus.focused != Some(node) {
        return;
    }
    paint.draw(&mirui::render::DrawCommand::Border {
        area: paint.rect(),
        transform: paint.transform(),
        quad: paint.quad(),
        color: theme.resolve(crate::theme::FOCUS),
        width: Fixed::from_int(3),
        radius: paint.style().border_radius,
        opa: 255,
    });
}

/// Text scale is distinct from viewport DPI. Callers read the observed scale
/// explicitly so Mirui subscribes to changes, rather than hiding that read.
pub fn text_size(scale: u16, base: u16) -> u16 {
    (u32::from(base) * u32::from(scale.clamp(100, 150)) / 100).min(u32::from(u16::MAX)) as u16
}

// Model commands publish unit changes. Queue extraction returns through the
// synchronous update closure instead of violating that transaction protocol.
impl ActionsHandle {
    pub fn take(&self) -> Vec<(u16, bool)> {
        let mut result = Vec::new();
        mirui::core::model::ModelHandle::update(self, |data| { result = data.take(); });
        result
    }
}
impl ActionsOwned {
    pub fn take(&mut self) -> Vec<(u16, bool)> {
        let mut result = Vec::new();
        mirui::core::model::OwnedModel::update(self, |data| { result = data.take(); });
        result
    }
}
