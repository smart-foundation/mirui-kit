//! Renderer-independent interaction policies shared by kit components.
//! No host I/O, clocks, global state, Mirui dependency, or application-specific IDs.
extern crate alloc;
pub mod collection;
pub mod forms;
pub mod group;
pub mod layout;
pub mod overlay;
pub mod tree;

pub mod table;
pub mod task;
