//! Catalog entries point at the actual constructors used by live tasks.
use crate::commands::Command;
pub struct ComponentDoc {
    pub title: &'static str,
    pub api: &'static str,
    pub source: &'static str,
    pub route: Command,
}
pub const COMPONENTS: &[ComponentDoc] = &[
    ComponentDoc {
        title: "Action / button / icon",
        api: "mirui_kit::widgets::action(actions, id, label, icon, overlay)",
        source: "crates/kit/src/widgets.rs::action",
        route: Command::Overview,
    },
    ComponentDoc {
        title: "Document field / placeholder",
        api: "mirui_kit::widgets::field(actions, document, id, placeholder, overlay, submit)",
        source: "crates/kit/src/widgets.rs::field",
        route: Command::Controls,
    },
    ComponentDoc {
        title: "Numeric slider / keyboard steps",
        api: "mirui_kit::widgets::number(actions, number, id)",
        source: "crates/kit/src/widgets.rs::number",
        route: Command::Controls,
    },
    ComponentDoc {
        title: "Boolean switch",
        api: "mirui_kit::widgets::toggle(actions, toggle, id, label)",
        source: "crates/kit/src/widgets.rs::toggle",
        route: Command::Controls,
    },
    ComponentDoc {
        title: "Draggable scrollbar",
        api: "mirui_kit::scrollbar::ScrollBar { target, vertical, thickness }",
        source: "crates/kit/src/scrollbar.rs",
        route: Command::Collections,
    },
    ComponentDoc {
        title: "Native text document editor",
        api: "mirui_kit::Editor { document: Some(document) }",
        source: "crates/kit/src/widgets.rs::Editor",
        route: Command::Workbench,
    },
    ComponentDoc {
        title: "Input ownership / focus scope",
        api: "mirui_kit::InputGate { actions, overlay } + FocusItem { overlay, order }",
        source: "crates/kit/src/keyboard.rs; crates/kit/src/widgets.rs",
        route: Command::Surfaces,
    },
    ComponentDoc {
        title: "Semantic appearance / typography",
        api: "mirui_kit::{theme, font}; Cargo feature weights-all",
        source: "crates/kit/src/theme.rs; crates/kit/src/font.rs",
        route: Command::Foundations,
    },
];
