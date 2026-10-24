use super::collection::ItemId;
use alloc::vec::Vec;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverlayKind {
    Menu,
    Popover,
    Dialog,
    AlertDialog,
    Tooltip,
}
#[derive(Clone, Copy, Debug)]
pub struct Overlay {
    pub id: ItemId,
    pub kind: OverlayKind,
    pub restore_focus: Option<ItemId>,
    pub escape_dismiss: bool,
    pub outside_dismiss: bool,
}
impl Overlay {
    pub fn modal(&self) -> bool {
        matches!(self.kind, OverlayKind::Dialog | OverlayKind::AlertDialog)
    }
    pub fn interactive(&self) -> bool {
        self.kind != OverlayKind::Tooltip
    }
}
#[derive(Clone, Debug, Default)]
pub struct OverlayStack {
    layers: Vec<Overlay>,
}
impl OverlayStack {
    pub fn layers(&self) -> &[Overlay] {
        &self.layers
    }
    pub fn push(&mut self, overlay: Overlay) -> bool {
        if self.layers.iter().any(|o| o.id == overlay.id) {
            return false;
        }
        self.layers.push(overlay);
        true
    }
    pub fn active(&self) -> Option<&Overlay> {
        self.layers.iter().rev().find(|o| o.interactive())
    }
    pub fn blocks_background(&self) -> bool {
        self.active().is_some()
    }
    pub fn trapped(&self) -> bool {
        self.layers.iter().any(|o| o.modal())
    }
    /// Only the top interactive layer owns Escape. A non-dismissible alert must
    /// not let Escape fall through and close the dialog beneath it.
    pub fn escape(&mut self) -> Option<Overlay> {
        let index = self.layers.iter().rposition(|o| o.interactive())?;
        if !self.layers[index].escape_dismiss {
            return None;
        }
        let closed = self.layers[index];
        self.layers.truncate(index);
        Some(closed)
    }
    pub fn outside(&mut self) -> Option<Overlay> {
        let index = self.layers.iter().rposition(|o| o.interactive())?;
        if !self.layers[index].outside_dismiss {
            return None;
        }
        let closed = self.layers[index];
        self.layers.truncate(index);
        Some(closed)
    }
    /// Closing a parent also removes its dependent layers. Return its trigger
    /// for restoration; host must check that target is still mounted/available.
    pub fn close(&mut self, id: ItemId) -> Option<Overlay> {
        let index = self.layers.iter().position(|o| o.id == id)?;
        let closed = self.layers[index];
        self.layers.truncate(index);
        Some(closed)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn layer(id: u64, kind: OverlayKind) -> Overlay {
        Overlay {
            id,
            kind,
            restore_focus: Some(id + 100),
            escape_dismiss: true,
            outside_dismiss: false,
        }
    }
    #[test]
    fn nested_escape_and_restore() {
        let mut s = OverlayStack::default();
        s.push(layer(1, OverlayKind::Dialog));
        s.push(layer(2, OverlayKind::Menu));
        assert_eq!(s.escape().unwrap().restore_focus, Some(102));
        assert!(s.trapped());
        assert_eq!(s.escape().unwrap().id, 1);
        assert!(!s.blocks_background());
    }
    #[test]
    fn alert_does_not_leak_escape() {
        let mut s = OverlayStack::default();
        s.push(layer(1, OverlayKind::Dialog));
        let mut a = layer(2, OverlayKind::AlertDialog);
        a.escape_dismiss = false;
        s.push(a);
        assert!(s.escape().is_none());
        assert_eq!(s.layers().len(), 2);
    }
    #[test]
    fn tooltip_never_steals_ownership() {
        let mut s = OverlayStack::default();
        s.push(layer(1, OverlayKind::Tooltip));
        assert!(!s.blocks_background());
        s.push(layer(2, OverlayKind::Menu));
        s.push(layer(3, OverlayKind::Tooltip));
        assert_eq!(s.escape().unwrap().id, 2);
        assert_eq!(s.layers().len(), 1);
    }
    #[test]
    fn duplicate_and_outside_policy() {
        let mut s = OverlayStack::default();
        assert!(s.push(layer(1, OverlayKind::Dialog)));
        assert!(!s.push(layer(1, OverlayKind::Menu)));
        assert!(s.outside().is_none());
        assert_eq!(s.close(1).unwrap().id, 1);
    }
}
