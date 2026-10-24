//! Keyboard focus stays inside the active input layer and restores on dismissal.
use crate::{Actions, ActionsHandle, FocusItem};
use alloc::vec::Vec;
use mirui::input::event::focus::FocusState;
use mirui::prelude::*;
use mirui::ui::{Hidden, Parent, dirty::Dirty};

pub fn install<B: Surface, F: RendererFactory<B>>(app: &mut App<B, F>, actions: ActionsHandle) {
    app.on_event(Key::EVENT, tab::callback(actions.clone()));
    app.add_system(focus_layer::system(actions));
}

#[event]
fn tab(
    event: event!(Key),
    actions: model!(Actions),
    focus: res!(mut FocusState),
    targets: query!(Entity, FocusItem),
    hidden: query!(Hidden),
    blocked: query!(crate::availability::InputBlocked),
    parents: query!(Parent),
    commands: commands!(),
) -> bool {
    if !event.pressed || event.physical != Some(PhysicalKey::Tab) {
        return false;
    }
    let mut candidates: Vec<(u16, Entity)> = Vec::new();
    for (entity, item) in targets {
        if item.overlay != actions.modal() {
            continue;
        }
        let mut node = entity;
        let mut visible = false;
        for _ in 0..64 {
            if hidden.get(node).is_some() || blocked.get(node).is_some() {
                visible = false;
                break;
            }
            if let Some(parent) = parents.get(node) {
                node = parent.0;
            } else {
                visible = true;
                break;
            }
        }
        if visible {
            candidates.push((item.order, entity));
        }
    }
    candidates.sort_by_key(|(order, entity)| (*order, entity.id));
    if candidates.is_empty() {
        return true;
    }
    let previous = focus.focused;
    let index = candidates.iter().position(|(_, e)| Some(*e) == previous);
    let reverse = event.modifiers.is_some_and(|m| m.shift);
    let next = match (index, reverse) {
        (Some(i), true) => (i + candidates.len() - 1) % candidates.len(),
        (Some(i), false) => (i + 1) % candidates.len(),
        (None, true) => candidates.len() - 1,
        (None, false) => 0,
    };
    focus.focused = Some(candidates[next].1);
    if let Some(previous) = previous {
        commands.insert(previous, Dirty);
    }
    commands.insert(candidates[next].1, Dirty);
    true
}
#[system]
fn focus_layer(
    actions: model!(Actions),
    focus: res!(mut FocusState),
    targets: query!(Entity, FocusItem),
    hidden: query!(Hidden),
    blocked: query!(crate::availability::InputBlocked),
    parents: query!(Parent),
    commands: commands!(),
) {
    if actions.applied_layer() == Some(actions.modal()) {
        return;
    }
    let previous = focus.focused;
    let restore = actions.saved_focus();
    actions.accept_layer(previous);
    let mut candidates: Vec<(u16, Entity)> = Vec::new();
    for (entity, item) in targets {
        if item.overlay != actions.modal() {
            continue;
        }
        let mut node = entity;
        let mut visible = false;
        for _ in 0..64 {
            if hidden.get(node).is_some() || blocked.get(node).is_some() {
                visible = false;
                break;
            }
            if let Some(parent) = parents.get(node) {
                node = parent.0;
            } else {
                visible = true;
                break;
            }
        }
        if visible {
            candidates.push((item.order, entity));
        }
    }
    candidates.sort_by_key(|(order, entity)| (*order, entity.id));
    let restored = if !actions.modal() {
        restore.filter(|e| candidates.iter().any(|(_, candidate)| candidate == e))
    } else {
        None
    };
    focus.focused = restored.or_else(|| candidates.first().map(|(_, e)| *e));
    if let Some(entity) = previous {
        commands.insert(entity, Dirty);
    }
    if let Some(entity) = focus.focused {
        commands.insert(entity, Dirty);
    }
}
