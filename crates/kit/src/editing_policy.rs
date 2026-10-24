//! Allocation-free selection-popover policy and viewport placement.
pub struct SelectionSession<T> {
    pub current: Option<T>,
    dismissed: Option<T>,
    pub visible: bool,
    pointer_down: bool,
}
impl<T> Default for SelectionSession<T> {
    fn default() -> Self {
        Self {
            current: None,
            dismissed: None,
            visible: false,
            pointer_down: false,
        }
    }
}
impl<T: Copy + Eq> SelectionSession<T> {
    pub fn sync(&mut self, current: Option<T>, pointer_down: bool) {
        let unchanged = self.current == current;
        if !unchanged {
            self.dismissed = None;
        }
        // Keep a mounted toolbar through its own pointer press. A new/dragged
        // selection waits for release, instead of stealing the selection drag.
        self.visible = current.is_some()
            && current != self.dismissed
            && (!pointer_down || (unchanged && self.visible));
        self.current = current;
    }
    /// Only the initial outside press dismisses. A held pointer cannot reopen
    /// the toolbar or transfer its ownership to another interaction.
    pub fn pointer(&mut self, down: bool, inside: bool) -> bool {
        let outside_press = down && !self.pointer_down && !inside;
        self.pointer_down = down;
        if outside_press {
            self.dismiss();
        }
        outside_press
    }
    pub fn dismiss(&mut self) {
        self.dismissed = self.current;
        self.visible = false;
    }
}

/// Prefer above the field, otherwise below; clamp both axes to the viewport.
/// Native editor caret geometry is private in Mirui 0.51, so do not fake it.
pub fn place(
    x: i32,
    y: i32,
    field_height: i32,
    width: i32,
    height: i32,
    viewport_width: i32,
    viewport_height: i32,
) -> (i32, i32) {
    let max_x = (viewport_width - width - 8).max(0);
    let max_y = (viewport_height - height - 8).max(0);
    let preferred_y = if y >= height + 16 {
        y - height - 8
    } else {
        y + field_height + 8
    };
    (
        x.clamp(8.min(max_x), max_x),
        preferred_y.clamp(8.min(max_y), max_y),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn session() -> SelectionSession<u32> {
        SelectionSession::default()
    }
    #[test]
    fn drag_waits_for_release() {
        let mut s = session();
        s.sync(Some(1), true);
        assert!(!s.visible);
        s.sync(Some(2), true);
        assert!(!s.visible);
        s.sync(Some(2), false);
        assert!(s.visible);
    }
    #[test]
    fn toolbar_survives_its_own_press() {
        let mut s = session();
        s.sync(Some(1), false);
        s.sync(Some(1), true);
        assert!(s.visible);
    }
    #[test]
    fn escape_does_not_reopen_on_idle_or_press() {
        let mut s = session();
        s.sync(Some(1), false);
        s.dismiss();
        for down in [false, true, false, false] {
            s.sync(Some(1), down);
            assert!(!s.visible);
        }
        s.sync(Some(2), false);
        assert!(s.visible);
    }
    #[test]
    fn collapsed_selection_or_removed_owner_closes() {
        let mut s = session();
        s.sync(Some(1), false);
        s.sync(None, false);
        assert!(!s.visible);
    }
    #[test]
    fn keyboard_selection_opens_without_pointer() {
        let mut s = session();
        s.sync(Some(1), false);
        assert!(s.visible);
    }
    #[test]
    fn new_owner_resets_dismissal() {
        let mut s = session();
        s.sync(Some(1), false);
        s.dismiss();
        s.sync(None, false);
        s.sync(Some(1), false);
        assert!(s.visible);
    }
    #[test]
    fn outside_press_dismisses_until_selection_changes() {
        let mut s = session();
        s.sync(Some(1), false);
        assert!(s.pointer(true, false));
        s.sync(Some(1), true);
        assert!(!s.visible);
        assert!(!s.pointer(true, false));
        s.pointer(false, false);
        s.sync(Some(1), false);
        assert!(!s.visible);
    }
    #[test]
    fn inside_press_does_not_dismiss() {
        let mut s = session();
        s.sync(Some(1), false);
        assert!(!s.pointer(true, true));
        s.sync(Some(1), true);
        assert!(s.visible);
    }
    #[test]
    fn top_edge_flips_below() {
        assert_eq!(place(12, 4, 24, 408, 48, 1180, 720), (12, 36));
    }
    #[test]
    fn bottom_right_clamps() {
        assert_eq!(place(1100, 700, 24, 408, 48, 1180, 720), (764, 644));
    }
    #[test]
    fn short_viewport_never_panics() {
        assert_eq!(place(-10, -10, 24, 408, 48, 100, 30), (0, 0));
    }
}
