use super::collection::ItemId;
use alloc::{collections::BTreeSet, string::String, vec::Vec};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroupKind {
    Actions,
    Single { required: bool },
    Multiple,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
#[derive(Clone, Copy, Debug)]
pub enum Move {
    Previous,
    Next,
    First,
    Last,
}
#[derive(Clone, Debug)]
pub struct GroupItem {
    pub id: ItemId,
    pub label: String,
    pub disabled: bool,
}
#[derive(Clone, Debug)]
pub struct GroupState {
    pub kind: GroupKind,
    pub orientation: Orientation,
    items: Vec<GroupItem>,
    selected: BTreeSet<ItemId>,
    active: Option<ItemId>,
}
impl GroupState {
    pub fn new(
        kind: GroupKind,
        orientation: Orientation,
        items: Vec<GroupItem>,
    ) -> Result<Self, ItemId> {
        let mut seen = BTreeSet::new();
        for item in &items {
            if !seen.insert(item.id) {
                return Err(item.id);
            }
        }
        let active = items.iter().find(|i| !i.disabled).map(|i| i.id);
        let mut selected = BTreeSet::new();
        if kind == (GroupKind::Single { required: true }) {
            selected.extend(active);
        }
        Ok(Self {
            kind,
            orientation,
            items,
            selected,
            active,
        })
    }
    pub fn items(&self) -> &[GroupItem] {
        &self.items
    }
    pub fn active(&self) -> Option<ItemId> {
        self.active
    }
    pub fn selected(&self, id: ItemId) -> bool {
        self.selected.contains(&id)
    }
    pub fn activate(&mut self, id: ItemId) -> bool {
        if !self.items.iter().any(|i| i.id == id && !i.disabled) {
            return false;
        }
        self.active = Some(id);
        match self.kind {
            GroupKind::Actions => {}
            GroupKind::Single { required } => {
                let remove = !required && self.selected.contains(&id);
                self.selected.clear();
                if !remove {
                    self.selected.insert(id);
                }
            }
            GroupKind::Multiple => {
                if !self.selected.remove(&id) {
                    self.selected.insert(id);
                }
            }
        }
        true
    }
    pub fn navigate(&mut self, movement: Move, select_on_focus: bool) {
        let ids: Vec<_> = self
            .items
            .iter()
            .filter(|i| !i.disabled)
            .map(|i| i.id)
            .collect();
        if ids.is_empty() {
            self.active = None;
            return;
        }
        let i = self.active.and_then(|id| ids.iter().position(|x| *x == id));
        let next = match movement {
            Move::First => 0,
            Move::Last => ids.len() - 1,
            Move::Next => i.map(|i| (i + 1) % ids.len()).unwrap_or(0),
            Move::Previous => i
                .map(|i| (i + ids.len() - 1) % ids.len())
                .unwrap_or(ids.len() - 1),
        };
        self.active = Some(ids[next]);
        if select_on_focus && matches!(self.kind, GroupKind::Single { .. }) {
            self.selected.clear();
            self.selected.insert(ids[next]);
        }
    }
    pub fn set_disabled(&mut self, id: ItemId, disabled: bool) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.disabled = disabled;
        }
        if disabled {
            self.selected.remove(&id);
        }
        if !self
            .items
            .iter()
            .any(|i| Some(i.id) == self.active && !i.disabled)
        {
            self.active = self.items.iter().find(|i| !i.disabled).map(|i| i.id);
        }
        if self.kind == (GroupKind::Single { required: true }) && self.selected.is_empty() {
            self.selected.extend(self.active);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    fn items() -> Vec<GroupItem> {
        (0..3)
            .map(|id| GroupItem {
                id,
                label: String::new(),
                disabled: id == 1,
            })
            .collect()
    }
    #[test]
    fn wraps_and_skips_disabled() {
        let mut s = GroupState::new(GroupKind::Actions, Orientation::Horizontal, items()).unwrap();
        s.navigate(Move::Previous, false);
        assert_eq!(s.active(), Some(2));
        s.navigate(Move::Next, false);
        assert_eq!(s.active(), Some(0));
        assert!(!s.activate(1));
    }
    #[test]
    fn required_selection_survives_disable() {
        let mut s = GroupState::new(
            GroupKind::Single { required: true },
            Orientation::Vertical,
            items(),
        )
        .unwrap();
        s.set_disabled(0, true);
        assert!(s.selected(2));
        s.activate(2);
        assert!(s.selected(2));
    }
    #[test]
    fn optional_single_and_multiple_have_different_contracts() {
        let mut s = GroupState::new(
            GroupKind::Single { required: false },
            Orientation::Horizontal,
            items(),
        )
        .unwrap();
        s.activate(0);
        s.activate(0);
        assert!(!s.selected(0));
        let mut m = GroupState::new(GroupKind::Multiple, Orientation::Horizontal, items()).unwrap();
        m.activate(0);
        m.activate(2);
        assert!(m.selected(0) && m.selected(2));
    }
    #[test]
    fn toolbar_navigation_does_not_select() {
        let mut s = GroupState::new(
            GroupKind::Single { required: true },
            Orientation::Horizontal,
            items(),
        )
        .unwrap();
        s.navigate(Move::Next, false);
        assert!(s.selected(0));
        s.navigate(Move::Next, true);
        assert!(s.selected(0));
    }
    #[test]
    fn duplicate_ids_rejected_and_empty_safe() {
        let i = GroupItem {
            id: 1,
            label: String::new(),
            disabled: false,
        };
        assert!(
            GroupState::new(
                GroupKind::Actions,
                Orientation::Horizontal,
                vec![i.clone(), i]
            )
            .is_err()
        );
        let mut s = GroupState::new(GroupKind::Actions, Orientation::Horizontal, vec![]).unwrap();
        s.navigate(Move::Last, false);
        assert_eq!(s.active(), None);
    }
}
