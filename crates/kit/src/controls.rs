//! Reusable stateful Mirui compositions, backed by tested no_std policies.
use crate::behavior::{
    collection::{ItemId, Modifiers, SelectionMode},
    forms::CheckState,
    group::{GroupItem, GroupKind, GroupState, Move, Orientation},
    tree::{TreeError, TreeKey, TreeNode, TreeRow, TreeState},
};
use crate::{
    Actions, FocusItem, font, icons, style, theme,
    widgets::{FocusRing, text_size},
};
use alloc::{collections::VecDeque, string::String, vec::Vec};
use mirui::prelude::*;
use mirui::ui::widgets::{Button, ButtonSize, Icon, Text};

#[model]
pub struct ChoiceGroup {
    state: GroupState,
    #[observe]
    pub revision: u64,
    pending: VecDeque<ItemId>,
}
impl ChoiceGroup {
    pub fn new(
        kind: GroupKind,
        orientation: Orientation,
        items: Vec<GroupItem>,
    ) -> Result<Self, ItemId> {
        Ok(Self {
            state: GroupState::new(kind, orientation, items)?,
            revision: 0,
            pending: VecDeque::new(),
        })
    }
}
#[model]
impl ChoiceGroup {
    pub fn items(&self) -> Vec<GroupItem> {
        self.state.items().to_vec()
    }
    pub fn selected(&self, id: ItemId) -> bool {
        self.state.selected(id)
    }
    pub fn active(&self) -> Option<ItemId> {
        self.state.active()
    }
    pub fn vertical(&self) -> bool {
        self.state.orientation == Orientation::Vertical
    }
    /// Bounded activation queue. An error means unavailable or backpressured;
    /// callers must drain take_action instead of assuming activation succeeded.
    pub fn activate(&mut self, id: ItemId) -> Result<(), ActivationError> {
        if self.pending.len() >= 32 {
            return Err(ActivationError::Backpressured);
        }
        if self.state.activate(id) {
            self.pending.push_back(id);
            self.revision = self.revision.wrapping_add(1);
            Ok(())
        } else {
            Err(ActivationError::Unavailable)
        }
    }
    pub fn navigate(&mut self, movement: Move, select_on_focus: bool) {
        self.state.navigate(movement, select_on_focus);
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn set_disabled(&mut self, id: ItemId, disabled: bool) {
        self.state.set_disabled(id, disabled);
        self.revision = self.revision.wrapping_add(1);
    }
    #[model(local)]
    pub fn take_action(&mut self) -> Option<ItemId> {
        self.pending.pop_front()
    }
}
/// A single tab stop; arrows operate the active descendant. Radio groups select
/// on arrow movement; toolbar/toggle groups move focus without changing values.
#[compose]
pub fn choice_group(
    actions: model!(Actions),
    group: model!(ChoiceGroup),
    key: u16,
    overlay: bool,
    select_on_focus: bool,
) -> Entity {
    ui! {
        View (width:Dimension::percent(100), padding:Padding::all(4), bg_color:theme::SEGMENTED_BG, border_radius:style::RADIUS, border_width:2, border_color:theme::CLEAR) [
            crate::sizing::ContentFit,Focusable, FocusItem{overlay,order:key}, FocusRing{actions:actions.clone()},
        ] {
            if ${ !group.vertical() } {
                Row (width:Dimension::percent(100),column_gap:2) {
                    walk ${ {let _=group.revision();group.items()} } with item by item.id {
                        choice_item (actions,group,item,overlay)
                    }
                }
            }
            if ${ group.vertical() } {
                Column (width:Dimension::percent(100),row_gap:2) [crate::sizing::ContentFit] {
                    walk ${ {let _=group.revision();group.items()} } with item by item.id {
                        choice_item (actions,group,item,overlay)
                    }
                }
            }
        } on Key(|event:event!(Key)| {
            if !event.pressed || actions.modal()!=overlay {return false;}
            let movement=match event.physical {
                Some(PhysicalKey::ArrowLeft) if !group.vertical()=>Some(Move::Previous),
                Some(PhysicalKey::ArrowRight) if !group.vertical()=>Some(Move::Next),
                Some(PhysicalKey::ArrowUp) if group.vertical()=>Some(Move::Previous),
                Some(PhysicalKey::ArrowDown) if group.vertical()=>Some(Move::Next),
                Some(PhysicalKey::Home)=>Some(Move::First),Some(PhysicalKey::End)=>Some(Move::Last),_=>None,
            };
            if let Some(movement)=movement {group.navigate(movement,select_on_focus);return true;}
            if event.repeat!=Some(true) && (event.code==13 || event.physical==Some(PhysicalKey::Space)) {
                if let Some(id)=group.active() {let _ = group.activate(id);} return true;
            }
            false
        }) on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);})
    }
}
#[compose]
fn choice_item(
    actions: model!(Actions),
    group: model!(ChoiceGroup),
    item: GroupItem,
    overlay: bool,
) -> Entity {
    ui! {
        Button (
            height:${text_size(actions.text_scale(),40)}, min_width:40, grow:1.0,
            padding:Padding::all(8),border_radius:style::RADIUS,border_width:2,
            border_color:${ {let _=group.revision();if group.active()==Some(item.id) {theme::FOCUS} else {theme::CLEAR}} },
            normal_color:${ {let _=group.revision();if group.selected(item.id) {theme::SEGMENTED_SELECTED} else {theme::CLEAR}} },
            pressed_color:theme::PRESSED
        ) {
            Text (text:item.label.clone(),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:if item.disabled {ColorToken::OnSurfaceVariant} else {ColorToken::OnSurface})
        } on Tap {if actions.modal()==overlay {let _ = group.activate(item.id);}}
    }
}

#[model]
pub struct Hierarchy {
    state: TreeState,
    #[observe]
    pub revision: u64,
}
impl Hierarchy {
    pub fn new(nodes: Vec<TreeNode>, mode: SelectionMode) -> Result<Self, TreeError> {
        Ok(Self {
            state: TreeState::new(nodes, mode)?,
            revision: 0,
        })
    }
}
#[model]
impl Hierarchy {
    pub fn rows(&self) -> Vec<TreeRow> {
        self.state.rows()
    }
    pub fn label(&self, id: ItemId) -> String {
        self.state
            .node(id)
            .map(|n| n.label.clone())
            .unwrap_or_default()
    }
    pub fn branch(&self, id: ItemId) -> bool {
        self.state.node(id).is_some_and(|n| n.branch)
    }
    pub fn disabled(&self, id: ItemId) -> bool {
        self.state.node(id).is_none_or(|n| n.disabled)
    }
    pub fn expanded(&self, id: ItemId) -> bool {
        self.state.expanded(id)
    }
    pub fn selected(&self, id: ItemId) -> bool {
        self.state.selection.contains(id)
    }
    pub fn focused(&self, id: ItemId) -> bool {
        self.state.selection.focus() == Some(id)
    }
    pub fn selected_ids(&self) -> Vec<ItemId> {
        self.state.selection.selected().iter().copied().collect()
    }
    pub fn choose(&mut self, id: ItemId, modifiers: Modifiers) {
        self.state.choose(id, modifiers);
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn toggle(&mut self, id: ItemId) {
        self.state.toggle(id);
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn key(&mut self, key: TreeKey, modifiers: Modifiers) {
        self.state.key(key, modifiers);
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn typeahead(&mut self, prefix: &str) {
        self.state.typeahead(prefix);
        self.revision = self.revision.wrapping_add(1);
    }
    pub fn replace(&mut self, nodes: Vec<TreeNode>) -> Result<(), TreeError> {
        self.state.replace(nodes)?;
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }
}
/// Tree viewport is owned by the consumer; this composition supplies rows and
/// navigation. Virtual rendering, drag/drop and host accessibility are separate
/// acceptance work, not implied by an indented list.
#[compose]
pub fn tree_view(
    actions: model!(Actions),
    tree: model!(Hierarchy),
    key: u16,
    overlay: bool,
) -> Entity {
    ui! {
        Column (width:Dimension::percent(100),row_gap:2,border_width:2,border_color:theme::CLEAR,border_radius:style::RADIUS) [crate::sizing::ContentFit,
            Focusable,FocusItem{overlay,order:key},FocusRing{actions:actions.clone()},
        ] {
            walk ${ {let _=tree.revision();tree.rows()} } with row by row.id {
                tree_row (actions,tree,row,overlay)
            }
            Text ("No items",visible:${ {let _=tree.revision();tree.rows().is_empty()} },font:font::MEDIUM,font_size:font::BODY_SIZE,text_color:ColorToken::OnSurfaceVariant)
        } on Key(|event:event!(Key)| {
            if !event.pressed || actions.modal()!=overlay {return false;}
            let m=event.modifiers;
            let modifiers=Modifiers{ctrl:m.is_some_and(|m|m.control),shift:m.is_some_and(|m|m.shift)};
            let key=match event.physical {
                Some(PhysicalKey::ArrowUp)=>Some(TreeKey::Up),Some(PhysicalKey::ArrowDown)=>Some(TreeKey::Down),
                Some(PhysicalKey::ArrowLeft)=>Some(TreeKey::Left),Some(PhysicalKey::ArrowRight)=>Some(TreeKey::Right),
                Some(PhysicalKey::Home)=>Some(TreeKey::Home),Some(PhysicalKey::End)=>Some(TreeKey::End),
                Some(PhysicalKey::Space) if event.repeat!=Some(true)=>Some(TreeKey::Space),
                Some(PhysicalKey::KeyA) if modifiers.ctrl=>Some(TreeKey::SelectAll),_=>None,
            };
            if let Some(key)=key {tree.key(key,modifiers);return true;} false
        }) on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);})
    }
}
#[compose]
fn tree_row(
    actions: model!(Actions),
    tree: model!(Hierarchy),
    row: TreeRow,
    overlay: bool,
) -> Entity {
    ui! {
        Row (width:Dimension::percent(100),height:${text_size(actions.text_scale(),40)},padding:Padding{left:((row.depth.min(32)*16) as i32).into(),top:0.into(),right:4.into(),bottom:0.into()},column_gap:4,bg_color:${ {let _=tree.revision();if tree.selected(row.id) {theme::SELECTION} else {theme::CLEAR}} },border_width:2,border_color:${ {let _=tree.revision();if tree.focused(row.id) {theme::FOCUS} else {theme::CLEAR}} },border_radius:style::RADIUS) {
            View (width:36,height:36) {
                Button (width:36,height:36,normal_color:theme::CLEAR,pressed_color:theme::PRESSED,visible:${ {let _=tree.revision();tree.branch(row.id)} }) {
                    Icon (path:icons::DISCLOSURE_DOWN.clone(),width:18,height:18,color:ColorToken::OnSurface,visible:${ {let _=tree.revision();tree.expanded(row.id)} })
                    Icon (path:icons::DISCLOSURE_RIGHT.clone(),width:18,height:18,color:ColorToken::OnSurface,visible:${ {let _=tree.revision();!tree.expanded(row.id)} })
                } on Tap {if actions.modal()==overlay {tree.toggle(row.id);}}
            }
            Button (size:ButtonSize::Custom,padding:Padding::all(8),align:AlignItems::Center,grow:1.0,height:${text_size(actions.text_scale(),36)},justify:JustifyContent::FlexStart,normal_color:theme::CLEAR,pressed_color:theme::PRESSED) {
                Text (text:${ {let _=tree.revision();tree.label(row.id)} },font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:${ {let _=tree.revision();if tree.disabled(row.id) {ColorToken::OnSurfaceVariant} else {ColorToken::OnSurface}} })
            } on Tap {if actions.modal()==overlay {tree.choose(row.id,Modifiers::default());}}
        }
    }
}

#[model]
#[derive(Default)]
pub struct Check {
    #[observe]
    pub value: CheckState,
    #[observe]
    pub disabled: bool,
    #[observe]
    pub read_only: bool,
}
#[model]
impl Check {
    pub fn toggle(&mut self) {
        if !self.disabled && !self.read_only {
            self.value = self.value.toggle();
        }
    }
}
#[compose]
pub fn checkbox(
    actions: model!(Actions),
    check: model!(Check),
    key: u16,
    label: String,
    overlay: bool,
) -> Entity {
    ui! {
        Button (width:Dimension::percent(100),height:${text_size(actions.text_scale(),40)},padding:Padding::all(8),normal_color:theme::CLEAR,pressed_color:theme::PRESSED,border_width:2,border_color:theme::CLEAR,border_radius:style::RADIUS) [Focusable,FocusItem{overlay,order:key},FocusRing{actions:actions.clone()}] {
            Row (width:Dimension::percent(100),column_gap:12,align:AlignItems::Center) {
                View (width:20,height:20,border_width:2,border_radius:4,border_color:ColorToken::Outline,bg_color:${if check.value()==CheckState::Unchecked {ColorToken::Surface} else {ColorToken::Primary}}) {
                    Icon (path:icons::CHECK.clone(),width:16,height:16,color:ColorToken::OnPrimary,visible:${check.value()==CheckState::Checked})
                    Icon (path:icons::MINUS.clone(),width:16,height:16,color:ColorToken::OnPrimary,visible:${check.value()==CheckState::Mixed})
                }
                Text (text:label.clone(),font:font::MEDIUM,font_size:${text_size(actions.text_scale(),font::BODY_SIZE)},text_color:${if check.disabled() {ColorToken::OnSurfaceVariant} else {ColorToken::OnSurface}})
            }
        } on Tap {if actions.modal()==overlay {check.toggle();}} on Key(|event:event!(Key)| {
            if event.pressed && event.repeat!=Some(true) && event.physical==Some(PhysicalKey::Space) && actions.modal()==overlay {check.toggle();return true;} false
        }) on FocusChanged(|event:event!(FocusChanged)| {actions.focus(key,event.focused);})
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivationError { Unavailable, Backpressured }
impl ChoiceGroupHandle {
    pub fn take_action(&self) -> Option<ItemId> {
        let mut result = None;
        mirui::core::model::ModelHandle::update(self, |data| { result = data.take_action(); });
        result
    }
}
impl ChoiceGroupOwned {
    pub fn take_action(&mut self) -> Option<ItemId> {
        let mut result = None;
        mirui::core::model::OwnedModel::update(self, |data| { result = data.take_action(); });
        result
    }
}

/// Named content panel for a single-selection ChoiceGroup. The same controller
/// drives a tab bar via choice_group(..., automatic_activation). In manual mode
/// arrows move the active tab and Enter/Space activates it. Hidden panels stay
/// mounted, preserving documents and application state.
#[component]
pub struct TabPanel {
    pub group: model!(ChoiceGroup),
    pub item: ItemId,
}
#[view(component=TabPanel, name="KitTabPanel", systems=[sync_panels::system()])]
pub fn tab_panel() {}
#[system]
fn sync_panels(
    panels: query!(Entity, TabPanel, optional mirui::ui::Hidden),
    commands: commands!(),
) {
    for (entity,panel,hidden) in panels {
        let visible = panel.group.selected(panel.item);
        if visible == hidden.is_some() {
            commands.set::<mirui::ui::property::prop::Visible>(entity,visible);
        }
    }
}
