//! Async lifecycle and bounded notifications without executors or hidden clocks.
use super::collection::ItemId;
use alloc::{collections::VecDeque, string::String};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ticket(u64);
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TaskStatus {
    #[default]
    Idle,
    Running {
        progress: Option<u16>,
    },
    Succeeded,
    Failed(String),
    Cancelled,
}
#[derive(Clone, Debug, Default)]
pub struct Task {
    generation: u64,
    pub status: TaskStatus,
}
impl Task {
    pub fn start(&mut self) -> Ticket {
        self.generation = self
            .generation
            .checked_add(1)
            .expect("task generation exhausted");
        self.status = TaskStatus::Running { progress: None };
        Ticket(self.generation)
    }
    pub fn progress(&mut self, ticket: Ticket, basis_points: u16) -> bool {
        if !self.current(ticket) {
            return false;
        }
        self.status = TaskStatus::Running {
            progress: Some(basis_points.min(10000)),
        };
        true
    }
    pub fn finish(&mut self, ticket: Ticket, result: Result<(), String>) -> bool {
        if !self.current(ticket) {
            return false;
        }
        self.status = match result {
            Ok(()) => TaskStatus::Succeeded,
            Err(error) => TaskStatus::Failed(error),
        };
        true
    }
    pub fn cancel(&mut self) {
        if matches!(self.status, TaskStatus::Running { .. }) {
            self.status = TaskStatus::Cancelled;
        }
    }
    fn current(&self, ticket: Ticket) -> bool {
        ticket.0 == self.generation && matches!(self.status, TaskStatus::Running { .. })
    }
}
#[derive(Clone, Debug)]
pub struct Notice {
    pub id: ItemId,
    pub title: String,
    pub description: String,
    pub action: Option<ItemId>,
    pub expires_at: Option<u64>,
}
#[derive(Clone, Debug)]
pub struct Notifications {
    capacity: usize,
    entries: VecDeque<Notice>,
}
impl Notifications {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: VecDeque::new(),
        }
    }
    pub fn entries(&self) -> &VecDeque<Notice> {
        &self.entries
    }
    pub fn push(&mut self, notice: Notice) -> bool {
        if let Some(old) = self.entries.iter_mut().find(|n| n.id == notice.id) {
            *old = notice;
            return true;
        }
        // Never silently discard actionable or sticky notices to make room.
        if self.entries.len() >= self.capacity {
            return false;
        }
        self.entries.push_back(notice);
        true
    }
    pub fn dismiss(&mut self, id: ItemId) -> bool {
        if let Some(i) = self.entries.iter().position(|n| n.id == id) {
            self.entries.remove(i);
            true
        } else {
            false
        }
    }
    /// UI owns hover/focus pause policy; invoke expiry with a paused clock or
    /// replace the notice deadline while hovered. No notices expire implicitly.
    pub fn expire(&mut self, now: u64) {
        self.entries
            .retain(|n| n.expires_at.is_none_or(|deadline| deadline > now));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stale_results_cannot_overwrite_new_job() {
        let mut t = Task::default();
        let old = t.start();
        let new = t.start();
        assert!(!t.finish(old, Ok(())));
        assert!(t.finish(new, Ok(())));
        assert!(!t.progress(new, 10));
    }
    #[test]
    fn cancellation_rejects_late_callbacks() {
        let mut t = Task::default();
        let token = t.start();
        t.cancel();
        assert!(!t.finish(token, Err("late".into())));
        assert_eq!(t.status, TaskStatus::Cancelled);
    }
    #[test]
    fn progress_is_bounded_and_terminal_state_stable() {
        let mut t = Task::default();
        let token = t.start();
        assert!(t.progress(token, u16::MAX));
        assert_eq!(
            t.status,
            TaskStatus::Running {
                progress: Some(10000)
            }
        );
        assert!(t.finish(token, Err("failed".into())));
        assert!(!t.finish(token, Ok(())));
    }
    fn notice(id: u64) -> Notice {
        Notice {
            id,
            title: "Message".into(),
            description: String::new(),
            action: None,
            expires_at: None,
        }
    }
    #[test]
    fn full_queue_backpressures_and_updates_existing() {
        let mut n = Notifications::new(1);
        assert!(n.push(notice(1)));
        assert!(!n.push(notice(2)));
        let mut updated = notice(1);
        updated.title = "Updated".into();
        assert!(n.push(updated));
        assert_eq!(n.entries()[0].title, "Updated");
        assert!(n.dismiss(1));
        assert!(n.push(notice(2)));
    }
    #[test]
    fn sticky_notice_never_expires() {
        let mut n = Notifications::new(2);
        n.push(notice(1));
        let mut transient = notice(2);
        transient.expires_at = Some(100);
        n.push(transient);
        n.expire(99);
        assert_eq!(n.entries().len(), 2);
        n.expire(100);
        assert_eq!(n.entries().len(), 1);
        n.expire(u64::MAX);
        assert_eq!(n.entries().len(), 1);
    }
}
