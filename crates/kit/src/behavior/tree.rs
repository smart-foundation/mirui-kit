use super::collection::{ItemId, Modifiers, Selection, SelectionMode};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::String,
    vec::Vec,
};
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub id: ItemId,
    pub parent: Option<ItemId>,
    pub label: String,
    pub branch: bool,
    pub disabled: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeError {
    Duplicate(ItemId),
    MissingParent(ItemId),
    Cycle(ItemId),
    LeafParent(ItemId),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeRow {
    pub id: ItemId,
    pub depth: usize,
    pub position: usize,
    pub siblings: usize,
}
#[derive(Clone, Copy, Debug)]
pub enum TreeKey {
    Up,
    Down,
    Home,
    End,
    Left,
    Right,
    Space,
    SelectAll,
}
#[derive(Clone, Debug)]
pub struct TreeState {
    nodes: Vec<TreeNode>,
    expanded: BTreeSet<ItemId>,
    pub selection: Selection,
}
impl TreeState {
    pub fn new(nodes: Vec<TreeNode>, mode: SelectionMode) -> Result<Self, TreeError> {
        Self::validate(&nodes)?;
        let mut state = Self {
            nodes,
            expanded: BTreeSet::new(),
            selection: Selection::new(mode),
        };
        state.reconcile();
        Ok(state)
    }
    fn validate(nodes: &[TreeNode]) -> Result<(), TreeError> {
        let mut map = BTreeMap::new();
        for node in nodes {
            if map.insert(node.id, node).is_some() {
                return Err(TreeError::Duplicate(node.id));
            }
        }
        // Each parent chain is checked once using a finished set; no recursion.
        let mut finished = BTreeSet::new();
        for node in nodes {
            let mut path = BTreeSet::new();
            let mut current = Some(node.id);
            while let Some(id) = current {
                if finished.contains(&id) {
                    break;
                }
                if !path.insert(id) {
                    return Err(TreeError::Cycle(id));
                }
                let n = map.get(&id).ok_or(TreeError::MissingParent(id))?;
                if let Some(parent) = n.parent {
                    let p = map.get(&parent).ok_or(TreeError::MissingParent(parent))?;
                    if !p.branch {
                        return Err(TreeError::LeafParent(parent));
                    }
                }
                current = n.parent;
            }
            finished.extend(path);
        }
        Ok(())
    }
    pub fn node(&self, id: ItemId) -> Option<&TreeNode> {
        self.nodes.iter().find(|n| n.id == id)
    }
    pub fn expanded(&self, id: ItemId) -> bool {
        self.expanded.contains(&id)
    }
    /// Depth-first rows, preserving input sibling order. Empty branches can be
    /// expanded; consumers may load children then atomically replace the model.
    pub fn rows(&self) -> Vec<TreeRow> {
        let mut children: BTreeMap<Option<ItemId>, Vec<ItemId>> = BTreeMap::new();
        for n in &self.nodes {
            children.entry(n.parent).or_default().push(n.id);
        }
        let mut pending = Vec::new();
        let mut rows = Vec::new();
        if let Some(roots) = children.get(&None) {
            for (i, id) in roots.iter().enumerate().rev() {
                pending.push(TreeRow {
                    id: *id,
                    depth: 0,
                    position: i + 1,
                    siblings: roots.len(),
                });
            }
        }
        while let Some(row) = pending.pop() {
            let id = row.id;
            let depth = row.depth;
            rows.push(row);
            if self.expanded.contains(&id) {
                if let Some(kids) = children.get(&Some(id)) {
                    for (i, id) in kids.iter().enumerate().rev() {
                        pending.push(TreeRow {
                            id: *id,
                            depth: depth + 1,
                            position: i + 1,
                            siblings: kids.len(),
                        });
                    }
                }
            }
        }
        rows
    }
    pub fn available(&self) -> Vec<ItemId> {
        self.rows()
            .iter()
            .filter(|r| self.node(r.id).is_some_and(|n| !n.disabled))
            .map(|r| r.id)
            .collect()
    }
    fn reconcile(&mut self) {
        let existing: Vec<_> = self
            .nodes
            .iter()
            .filter(|n| !n.disabled)
            .map(|n| n.id)
            .collect();
        self.selection.reconcile(&existing);
        let visible = self.available();
        if !self
            .selection
            .focus()
            .is_some_and(|id| visible.contains(&id))
        {
            if let Some(id) = visible.first() {
                self.selection.focus_only(*id, &visible);
            }
        }
    }
    pub fn replace(&mut self, nodes: Vec<TreeNode>) -> Result<(), TreeError> {
        Self::validate(&nodes)?; // Invalid updates never corrupt the old tree.
        self.nodes = nodes;
        let branches: BTreeSet<_> = self
            .nodes
            .iter()
            .filter(|n| n.branch)
            .map(|n| n.id)
            .collect();
        self.expanded.retain(|id| branches.contains(id));
        self.reconcile();
        Ok(())
    }
    pub fn toggle(&mut self, id: ItemId) {
        if !self.node(id).is_some_and(|n| n.branch && !n.disabled) {
            return;
        }
        if !self.expanded.remove(&id) {
            self.expanded.insert(id);
        }
        let available = self.available();
        if self
            .selection
            .focus()
            .is_some_and(|f| !available.contains(&f))
        {
            self.selection.focus_only(id, &available);
        }
        self.reconcile();
    }
    pub fn choose(&mut self, id: ItemId, modifiers: Modifiers) {
        self.selection.choose(id, &self.available(), modifiers);
    }
    pub fn key(&mut self, key: TreeKey, modifiers: Modifiers) {
        let ids = self.available();
        if ids.is_empty() {
            return;
        }
        let current = self.selection.focus().unwrap_or(ids[0]);
        let index = ids.iter().position(|id| *id == current).unwrap_or(0);
        let target = match key {
            TreeKey::Home => Some(ids[0]),
            TreeKey::End => ids.last().copied(),
            TreeKey::Up => Some(ids[index.saturating_sub(1)]),
            TreeKey::Down => Some(ids[(index + 1).min(ids.len() - 1)]),
            TreeKey::Left => {
                if self.expanded(current) {
                    self.toggle(current);
                    None
                } else {
                    self.node(current)
                        .and_then(|n| n.parent)
                        .filter(|p| ids.contains(p))
                }
            }
            TreeKey::Right => {
                if self.node(current).is_some_and(|n| n.branch) && !self.expanded(current) {
                    self.toggle(current);
                    None
                } else {
                    self.nodes
                        .iter()
                        .find(|n| n.parent == Some(current) && !n.disabled)
                        .map(|n| n.id)
                }
            }
            TreeKey::Space => {
                self.choose(
                    current,
                    Modifiers {
                        ctrl: true,
                        shift: modifiers.shift,
                    },
                );
                None
            }
            TreeKey::SelectAll => {
                self.selection.select_all(&ids);
                None
            }
        };
        if let Some(id) = target {
            if modifiers.shift {
                self.choose(id, modifiers);
            } else {
                self.selection.focus_only(id, &ids);
            }
        }
    }
    /// Host supplies the accumulated prefix and its timeout; no hidden wall clock.
    pub fn typeahead(&mut self, prefix: &str) {
        if prefix.is_empty() {
            return;
        }
        let ids = self.available();
        let start = self
            .selection
            .focus()
            .and_then(|id| ids.iter().position(|i| *i == id))
            .map(|i| i + 1)
            .unwrap_or(0);
        for offset in 0..ids.len() {
            let id = ids[(start + offset) % ids.len()];
            if self
                .node(id)
                .is_some_and(|n| n.label.to_lowercase().starts_with(&prefix.to_lowercase()))
            {
                self.selection.focus_only(id, &ids);
                break;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    fn nodes() -> Vec<TreeNode> {
        vec![
            TreeNode {
                id: 1,
                parent: None,
                label: "Root".into(),
                branch: true,
                disabled: false,
            },
            TreeNode {
                id: 2,
                parent: Some(1),
                label: "Child".into(),
                branch: false,
                disabled: false,
            },
            TreeNode {
                id: 3,
                parent: None,
                label: "Other".into(),
                branch: false,
                disabled: false,
            },
        ]
    }
    #[test]
    fn expand_collapse_and_parent_navigation() {
        let mut t = TreeState::new(nodes(), SelectionMode::Multiple).unwrap();
        t.key(TreeKey::Right, Modifiers::default());
        assert_eq!(t.rows().len(), 3);
        t.key(TreeKey::Right, Modifiers::default());
        assert_eq!(t.selection.focus(), Some(2));
        t.key(TreeKey::Left, Modifiers::default());
        assert_eq!(t.selection.focus(), Some(1));
        t.key(TreeKey::Left, Modifiers::default());
        assert_eq!(t.rows().len(), 2);
    }
    #[test]
    fn collapse_recovers_descendant_focus() {
        let mut t = TreeState::new(nodes(), SelectionMode::Multiple).unwrap();
        t.toggle(1);
        t.choose(2, Modifiers::default());
        t.toggle(1);
        assert_eq!(t.selection.focus(), Some(1));
        assert!(t.selection.contains(2));
    }
    #[test]
    fn invalid_updates_are_atomic() {
        let mut t = TreeState::new(nodes(), SelectionMode::Single).unwrap();
        let mut n = nodes();
        n[0].parent = Some(1);
        assert_eq!(t.replace(n), Err(TreeError::Cycle(1)));
        assert_eq!(t.rows().len(), 2);
    }
    #[test]
    fn invalid_topology_rejected() {
        let mut n = nodes();
        n[1].parent = Some(77);
        assert!(matches!(
            TreeState::new(n, SelectionMode::Single),
            Err(TreeError::MissingParent(77))
        ));
        let mut n = nodes();
        n[0].branch = false;
        assert!(matches!(
            TreeState::new(n, SelectionMode::Single),
            Err(TreeError::LeafParent(1))
        ));
    }
    #[test]
    fn typeahead_wraps_without_selecting() {
        let mut t = TreeState::new(nodes(), SelectionMode::Multiple).unwrap();
        t.typeahead("o");
        assert_eq!(t.selection.focus(), Some(3));
        assert!(t.selection.selected().is_empty());
        t.typeahead("r");
        assert_eq!(t.selection.focus(), Some(1));
    }
    #[test]
    fn row_semantics_have_levels_positions_and_sizes() {
        let mut t = TreeState::new(nodes(), SelectionMode::Single).unwrap();
        t.toggle(1);
        assert_eq!(
            t.rows()[1],
            TreeRow {
                id: 2,
                depth: 1,
                position: 1,
                siblings: 1
            }
        );
        assert_eq!(t.rows()[2].position, 2);
    }
}
