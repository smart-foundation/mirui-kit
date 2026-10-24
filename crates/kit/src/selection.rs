//! Automatic, nonmodal editing actions over genuine native TextEditing selection.
//! No synthetic document edits, clipboard cache, focus-stealing backdrop, or per-frame text copy.
use crate::editing_policy::{SelectionSession, place};
use alloc::string::{String, ToString};
use mirui::app::RenderViewport;
use mirui::input::event::{
    PointerCursor,
    focus::{FocusState, PointerFocus},
};
use mirui::prelude::*;
use mirui::ui::{
    ComputedRect, Hidden, Parent,
    clipboard::{ClipboardError, ClipboardFinished, ClipboardOutcome},
    widgets::{DropShadow, Text, TextEditing},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectionStamp {
    pub owner: Entity,
    pub anchor: usize,
    pub cursor: usize,
}

#[model]
#[derive(Default)]
pub struct SelectionTools {
    session: SelectionSession<SelectionStamp>,
    active: Option<Entity>,
    feedback_ticks: u8,
    #[observe]
    pub visible: bool,
    #[observe]
    pub all_selected: bool,
    #[observe]
    pub x: i32,
    #[observe]
    pub y: i32,
    pub feedback: String,
    #[observe]
    pub feedback_revision: u64,
}
#[model]
impl SelectionTools {
    pub fn stamp(&self) -> Option<SelectionStamp> {
        self.session.current
    }
    pub fn sync(
        &mut self,
        active: Option<Entity>,
        stamp: Option<SelectionStamp>,
        down: bool,
        x: i32,
        y: i32,
        all_selected: bool,
    ) {
        if self.active != active || (stamp.is_some() && self.session.current != stamp) {
            self.feedback.clear();
            self.feedback_revision = self.feedback_revision.wrapping_add(1);
            self.feedback_ticks = 0;
        }
        self.active = active;
        self.session.sync(stamp, down);
        self.visible = self.session.visible;
        self.all_selected = all_selected;
        if stamp.is_some() {
            self.x = x;
            self.y = y;
        }
    }
    pub fn pointer_sample(&mut self, down: bool, inside: bool) {
        if self.session.pointer(down, inside) {
            self.visible = false;
            self.feedback.clear();
            self.feedback_revision = self.feedback_revision.wrapping_add(1);
        }
    }
    /// Dismissal is remembered until selection changes; Escape never deselects text.
    pub fn dismiss(&mut self) {
        self.session.dismiss();
        self.visible = false;
        self.feedback.clear();
            self.feedback_revision = self.feedback_revision.wrapping_add(1);
    }
    pub fn finished(&mut self, owner: Entity, result: Result<ClipboardOutcome, ClipboardError>) {
        self.feedback_revision = self.feedback_revision.wrapping_add(1);
        if self.active != Some(owner) {
            return;
        }
        self.feedback = match result {
            Ok(ClipboardOutcome::Copied) => "Copied",
            Ok(ClipboardOutcome::Cut) => "Cut",
            Ok(ClipboardOutcome::Pasted) => "Pasted",
            Ok(ClipboardOutcome::CopiedWithoutCut) => "Copied; text changed before Cut completed",
            Ok(ClipboardOutcome::EmptySelection) => "Select text first",
            Err(ClipboardError::PermissionDenied) => "Clipboard permission denied",
            Err(ClipboardError::Unsupported) => "This host does not support the clipboard",
            Err(ClipboardError::Capacity) => "Clipboard content exceeds the field capacity",
            Err(_) => "Clipboard unavailable; no edit was applied",
        }
        .to_string();
        self.feedback_ticks = 16;
    }
    pub fn tick(&mut self) {
        if self.feedback_ticks > 0 {
            self.feedback_ticks -= 1;
            if self.feedback_ticks == 0 {
                self.feedback.clear();
            self.feedback_revision = self.feedback_revision.wrapping_add(1);
            }
        }
    }
}
pub struct SelectionToolsResource(pub SelectionToolsHandle);

/// Install once, AFTER composing the application's main tree, so this final child is
/// above dialogs too. Do not spawn a second App root: Mirui has one active root. Its empty canvas is not a hit target; only the bar blocks input.
pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>, root: Entity) {
    let tools = app.add_model(SelectionTools::default()).finish();
    app.with_widget(action_hover::view());
    app.add_resource(SelectionToolsResource(tools.clone()));
    app.add_system(synchronize::system(tools.clone()));
    app.add_timer(expire::timer(tools.clone()));
    app.on_event(Key::EVENT, escape::callback(tools.clone()));
    app.compose(root, layer(tools));
}

#[timer(every = 250)]
fn expire(tools: model!(SelectionTools)) {
    tools.tick();
}

#[event]
fn escape(event: event!(Key), tools: model!(SelectionTools)) -> bool {
    if event.pressed && event.code == 27 && (tools.visible() || !tools.feedback().is_empty()) {
        tools.dismiss();
        return true;
    }
    false
}

/// Add this notification listener on the actual editor node, not its chrome.
/// Consumers without the optional service still get normal native clipboard behavior.
#[event]
pub fn clipboard_finished(
    node: entity!(),
    event: event!(ClipboardFinished),
    service: res!(optional SelectionToolsResource),
) {
    if let Some(service) = service {
        service.0.finished(node, event.result);
    }
}

#[system]
fn synchronize(
    tools: model!(SelectionTools),
    focus: res!(FocusState),
    cursor: res!(PointerCursor),
    viewport: res!(RenderViewport),
    editors: query!(TextEditing),
    rects: query!(ComputedRect),
    hidden: query!(Hidden),
    states: query!(crate::availability::InputBlocked),
    parents: query!(Parent),
) {
    let mut active = focus
        .focused
        .filter(|entity| editors.get(*entity).is_some());
    if let Some(owner) = active {
        let mut node = owner;
        // Bound malformed/cyclic parent chains too. Hidden/disabled ancestors
        // exclude gated background editors while a dialog/menu owns input.
        let mut valid = false;
        for _ in 0..64 {
            if hidden.get(node).is_some() || states.get(node).is_some() {
                break;
            }
            if let Some(parent) = parents.get(node) {
                node = parent.0;
            } else {
                valid = true;
                break;
            }
        }
        if !valid {
            active = None;
        }
    }
    let stamp = active.and_then(|owner| {
        let selection = editors.get(owner)?.selection();
        (selection.anchor != selection.cursor).then_some(SelectionStamp {
            owner,
            anchor: selection.anchor,
            cursor: selection.cursor,
        })
    });
    let (width, height) = viewport.0.logical_size();
    let (x, y) = stamp
        .and_then(|s| rects.get(s.owner))
        .map(|rect| {
            place(
                rect.0.x.to_int(),
                rect.0.y.to_int(),
                rect.0.h.to_int(),
                408,
                48,
                i32::from(width),
                i32::from(height),
            )
        })
        .unwrap_or((0, 0));
    let all = stamp.is_some_and(|s| {
        s.anchor.min(s.cursor) == 0
            && editors
                .get(s.owner)
                .is_some_and(|editor| s.anchor.max(s.cursor) == editor.len())
    });
    let in_bar = (tools.visible() || !tools.feedback().is_empty())
        && cursor.x >= Fixed::from_int(tools.x())
        && cursor.x < Fixed::from_int(tools.x() + 408)
        && cursor.y >= Fixed::from_int(tools.y())
        && cursor.y < Fixed::from_int(tools.y() + 48);
    let in_field = active
        .and_then(|entity| rects.get(entity))
        .is_some_and(|rect| {
            cursor.x >= rect.0.x
                && cursor.x < rect.0.x + rect.0.w
                && cursor.y >= rect.0.y
                && cursor.y < rect.0.y + rect.0.h
        });
    tools.pointer_sample(cursor.down, in_bar || in_field);
    tools.sync(active, stamp, cursor.down, x, y, all);
}

#[derive(Clone, Copy)]
enum EditAction {
    Copy,
    Cut,
    Paste,
    All,
}
#[event]
fn operate(
    tools: model!(SelectionTools),
    action: EditAction,
    focus: res!(FocusState),
    editors: query!(TextEditing),
    hidden: query!(Hidden),
    states: query!(crate::availability::InputBlocked),
    parents: query!(Parent),
    commands: commands!(),
) -> bool {
    let Some(stamp) = tools.stamp() else {
        return true;
    };
    // Revalidate at activation, not just when painting. Never send a stale
    // action to a newly focused field or a dialog's now-disabled background.
    let Some(editor) = editors.get(stamp.owner) else {
        tools.dismiss();
        return true;
    };
    let selection = editor.selection();
    if focus.focused != Some(stamp.owner)
        || selection.anchor != stamp.anchor
        || selection.cursor != stamp.cursor
    {
        tools.dismiss();
        return true;
    }
    let mut node = stamp.owner;
    let mut valid = false;
    for _ in 0..64 {
        if hidden.get(node).is_some() || states.get(node).is_some() {
            break;
        }
        if let Some(parent) = parents.get(node) {
            node = parent.0;
        } else {
            valid = true;
            break;
        }
    }
    if !valid {
        tools.dismiss();
        return true;
    }
    tools.dismiss();
    match action {
        EditAction::Copy => commands.clipboard().copy(stamp.owner),
        EditAction::Cut => commands.clipboard().cut(stamp.owner),
        EditAction::Paste => commands.clipboard().paste(stamp.owner),
        EditAction::All => commands.selection().select_all(stamp.owner),
    }
    true
}

#[compose]
fn edit_action(
    tools: model!(SelectionTools),
    label: &'static str,
    shortcut: &'static str,
    action: EditAction,
    width: i32,
) -> Entity {
    let identity = match action {
        EditAction::Copy => "text_selection_copy",
        EditAction::Cut => "text_selection_cut",
        EditAction::Paste => "text_selection_paste",
        EditAction::All => "text_selection_all",
    };
    ui! {
        Column (
            width: width,
            height: 40,
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            border_radius: 6
        ) [
            HitTarget,
            PointerFocus::Preserve,
            InteractionFeedback,
            SelectionAction,
            mirui::ui::NamedId(identity),
        ] {
            Text (
                text: label,
                font: crate::font::MEDIUM,
                font_size: 14,
                text_color: ColorToken::OnSurface
            )
            Text (text: shortcut, font_size: 12, text_color: ColorToken::OnSurfaceVariant)
        } on Tap(operate::callback(tools.clone(), action))
    }
}
#[compose]
fn layer(tools: model!(SelectionTools)) -> Entity {
    ui! {
        View (
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            position: Position::Absolute,
            left: 0,
            top: 0
        ) {
            if ${ tools.visible() || !tools.feedback().is_empty() } {
                View (
                    position: Position::Absolute,
                    left: ${ tools.x() },
                    top: ${ tools.y() },
                    width: 408,
                    height: 48
                ) [
                    DropShadow::new(id("text_selection_actions"))
                        .with_blur_radius(12)
                        .with_offset(0, 4)
                        .with_opacity(52),
                ]
                Row (
                    id: "text_selection_actions",
                    position: Position::Absolute,
                    left: ${ tools.x() },
                    top: ${ tools.y() },
                    width: 408,
                    height: 48,
                    padding: Padding::all(4),
                    column_gap: 2,
                    align: AlignItems::Center,
                    bg_color: ColorToken::SurfaceVariant,
                    border_color: ColorToken::Outline,
                    border_width: 1,
                    border_radius: 10
                ) [
                    HitTarget,
                    PointerFocus::Preserve,
                    crate::field::EditingSurface,
                ] {
                    View (width: 354, height: 40) {
                        if ${ tools.visible() } {
                            action_group (tools)
                        } else {
                            Text (
                                text: ${ tools.feedback() },
                                width: 354,
                                font_size: 14,
                                text_color: ColorToken::OnSurface
                            )
                        }
                    }
                    View (
                        id: "text_selection_dismiss",
                        width: 40,
                        height: 40,
                        align: AlignItems::Center,
                        justify: JustifyContent::Center
                    ) [
                        HitTarget,
                        PointerFocus::Preserve,
                    ] {
                        Text ("Esc", font_size: 12, text_color: ColorToken::OnSurfaceVariant)
                    } on Tap { tools.dismiss(); }
                } on Tap(|_event: event!(Tap)| { true })
            }
        }
    }
}

#[compose]
fn action_group(tools: model!(SelectionTools)) -> Entity {
    ui! {
        Row (height: 40, column_gap: 2, align: AlignItems::Center) {
            edit_action (tools, "Copy", "Ctrl C", (EditAction::Copy), 80)
            edit_action (tools, "Cut", "Ctrl X", (EditAction::Cut), 68)
            edit_action (tools, "Paste", "Ctrl V", (EditAction::Paste), 80)
            View (width: 120, height: 40) {
                if ${ !tools.all_selected() } {
                    edit_action (tools, "Select all", "Ctrl A", (EditAction::All), 120)
                } else {
                    View (
                        width: 120,
                        height: 40,
                        align: AlignItems::Center,
                        justify: JustifyContent::Center
                    ) {
                        Text (
                            "All selected",
                            font_size: 12,
                            text_color: ColorToken::OnSurfaceVariant
                        )
                    }
                }
            }
        }
    }
}

#[component]
pub struct SelectionAction;
#[view(component = SelectionAction, name = "SelectionAction", priority = 70)]
pub fn action_hover(theme: res!(Theme), paint: paint!()) {
    use mirui::ui::theme::WidgetState;
    let color = match paint.state() {
        WidgetState::Pressed => crate::theme::PRESSED,
        WidgetState::Hovered => crate::theme::HOVER,
        _ => return,
    };
    paint.draw(&mirui::render::command::DrawCommand::Fill {
        area: paint.rect(),
        transform: paint.transform(),
        quad: paint.quad(),
        color: theme.resolve(color),
        radius: Fixed::from_int(6),
        opa: 255,
    });
}

impl SelectionToolsHandle {
    pub fn feedback(&self) -> String {
        let _ = self.feedback_revision();
        mirui::core::model::ModelHandle::read(self, |data| data.feedback.clone())
    }
}
