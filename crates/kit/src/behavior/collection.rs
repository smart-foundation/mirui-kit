use alloc::{collections::BTreeSet, vec::Vec};
pub type ItemId = u64;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionMode {
    None,
    Single,
    Multiple,
}
#[derive(Clone, Copy, Debug, Default)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
}
/// Focus and selection are deliberately independent. Ranges follow visible order,
/// never numeric IDs. Call reconcile after filtering, removal, or disabling rows.
#[derive(Clone, Debug)]
pub struct Selection {
    pub mode: SelectionMode,
    selected: BTreeSet<ItemId>,
    focus: Option<ItemId>,
    anchor: Option<ItemId>,
}
impl Selection {
    pub fn new(mode: SelectionMode) -> Self {
        Self {
            mode,
            selected: BTreeSet::new(),
            focus: None,
            anchor: None,
        }
    }
    pub fn selected(&self) -> &BTreeSet<ItemId> {
        &self.selected
    }
    pub fn focus(&self) -> Option<ItemId> {
        self.focus
    }
    pub fn contains(&self, id: ItemId) -> bool {
        self.selected.contains(&id)
    }
    pub fn focus_only(&mut self, id: ItemId, available: &[ItemId]) {
        if available.contains(&id) {
            self.focus = Some(id);
        }
    }
    pub fn choose(&mut self, id: ItemId, available: &[ItemId], modifiers: Modifiers) {
        let Some(end) = available.iter().position(|x| *x == id) else {
            return;
        };
        self.focus = Some(id);
        if self.mode == SelectionMode::None {
            return;
        }
        if self.mode == SelectionMode::Multiple && modifiers.shift {
            let start = self
                .anchor
                .and_then(|a| available.iter().position(|x| *x == a))
                .unwrap_or(end);
            if !modifiers.ctrl {
                self.selected.clear();
            }
            self.selected
                .extend(available[start.min(end)..=start.max(end)].iter().copied());
            if self.anchor.is_none() {
                self.anchor = Some(id);
            }
        } else if self.mode == SelectionMode::Multiple && modifiers.ctrl {
            if !self.selected.remove(&id) {
                self.selected.insert(id);
            }
            self.anchor = Some(id);
        } else {
            self.selected.clear();
            self.selected.insert(id);
            self.anchor = Some(id);
        }
    }
    pub fn select_all(&mut self, available: &[ItemId]) {
        if self.mode == SelectionMode::Multiple {
            self.selected.extend(available.iter().copied());
        }
    }
    pub fn clear(&mut self) {
        self.selected.clear();
        self.anchor = None;
    }
    pub fn reconcile(&mut self, available: &[ItemId]) {
        self.selected.retain(|id| available.contains(id));
        if !self.focus.is_some_and(|id| available.contains(&id)) {
            self.focus = available.first().copied();
        }
        if !self.anchor.is_some_and(|id| available.contains(&id)) {
            self.anchor = self.focus;
        }
    }
}
/// A fixed-height window; callers render only this range plus spacer extents.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Window {
    pub start: usize,
    pub end: usize,
    pub before: usize,
    pub after: usize,
}
pub fn window(
    len: usize,
    offset: usize,
    viewport: usize,
    row_height: usize,
    overscan: usize,
) -> Window {
    let height = row_height.max(1);
    let first = (offset / height).min(len);
    let start = first.saturating_sub(overscan);
    let count = viewport.saturating_add(height - 1) / height;
    let end = first
        .saturating_add(count)
        .saturating_add(overscan)
        .saturating_add(1)
        .min(len);
    Window {
        start,
        end,
        before: start.saturating_mul(height),
        after: (len - end).saturating_mul(height),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}
/// Stable multi-column sort descriptors; comparators belong to the domain.
#[derive(Clone, Debug, Default)]
pub struct SortOrder(pub Vec<(ItemId, SortDirection)>);
impl SortOrder {
    pub fn toggle(&mut self, column: ItemId, additive: bool) {
        let previous = self.0.iter().find(|(id, _)| *id == column).map(|(_, d)| *d);
        if !additive {
            self.0.clear();
        } else {
            self.0.retain(|(id, _)| *id != column);
        }
        match previous {
            None => self.0.push((column, SortDirection::Ascending)),
            Some(SortDirection::Ascending) => self.0.push((column, SortDirection::Descending)),
            Some(SortDirection::Descending) => {}
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_is_visible_order_not_id_order() {
        let mut s = Selection::new(SelectionMode::Multiple);
        let ids = [90, 2, 70, 1];
        s.choose(2, &ids, Modifiers::default());
        s.choose(
            1,
            &ids,
            Modifiers {
                shift: true,
                ctrl: false,
            },
        );
        assert_eq!(s.selected().iter().copied().collect::<Vec<_>>(), [1, 2, 70]);
        s.choose(
            90,
            &ids,
            Modifiers {
                ctrl: true,
                shift: false,
            },
        );
        assert_eq!(s.selected().len(), 4);
    }
    #[test]
    fn unavailable_cannot_select_and_removal_recovers_focus() {
        let mut s = Selection::new(SelectionMode::Single);
        s.choose(3, &[1, 2], Modifiers::default());
        assert_eq!(s.focus(), None);
        s.choose(2, &[1, 2], Modifiers::default());
        s.reconcile(&[1]);
        assert_eq!(s.focus(), Some(1));
        assert!(s.selected().is_empty());
    }
    #[test]
    fn ctrl_navigation_preserves_selection() {
        let mut s = Selection::new(SelectionMode::Multiple);
        s.choose(1, &[1, 2], Modifiers::default());
        s.focus_only(2, &[1, 2]);
        assert!(s.contains(1));
        assert!(!s.contains(2));
    }
    #[test]
    fn windows_are_bounded_even_for_overflow() {
        for len in [0, 1, 100, usize::MAX / 2] {
            let w = window(len, usize::MAX, usize::MAX, 0, 8);
            assert!(w.start <= w.end && w.end <= len);
        }
        assert_eq!(
            window(100, 200, 100, 20, 2),
            Window {
                start: 8,
                end: 18,
                before: 160,
                after: 1640
            }
        );
    }
    #[test]
    fn sort_cycles_and_keeps_secondary_keys() {
        let mut s = SortOrder::default();
        s.toggle(1, false);
        s.toggle(2, true);
        s.toggle(1, true);
        assert_eq!(
            s.0,
            [
                (2, SortDirection::Ascending),
                (1, SortDirection::Descending)
            ]
        );
        s.toggle(1, true);
        assert_eq!(s.0.len(), 1);
    }
}
