//! One registry drives palette discovery and dispatch. Every command has an icon.
use mirui::render::path::Path;
use mirui_kit::icons;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Command {
    Overview = 1,
    Controls,
    Collections,
    Workbench,
    Surfaces,
    Foundations,
    Palette,
    Inspector,
    Options,
    Help,
    Diagnostics,
    Catalog,
    Reset,
    Notices,
    NewFolder,
    Rename,
    Preview,
    SavePreset,
    SaveNote,
    Review,
    ApplyAppearance,
    Dark,
    Light,
    Smaller,
    Larger,
    Up,
    OpenAsset,
    Move,
    CommitName,
    CancelName,
    Close,
    Keep,
    Revert,
    SaveLeave,
    DiscardLeave,
    ApplyReview,
    ConfirmReset,
    ToggleSort,
    AppendParagraph,
    Context,
    Tooltip,
}
#[derive(Clone, Copy)]
pub struct CommandInfo {
    pub command: Command,
    pub label: &'static str,
    pub icon: &'static Path,
}
macro_rules! commands { ($( $key:ident => ($label:literal, $icon:ident) ),* $(,)?) => { pub const COMMANDS: &[CommandInfo] = &[$(CommandInfo { command: Command::$key, label: $label, icon: &icons::$icon }),*]; }; }
commands! {
 Overview => ("Overview", HOME), Controls => ("Controls", GEAR), Collections => ("Collections", FOLDER),
 Workbench => ("Workbench", RENAME), Surfaces => ("Surfaces", LAYERS), Foundations => ("Foundations", SUN),
 Palette => ("Find a command", SEARCH), Inspector => ("Inspect selection", INFO), Options => ("Collection options", GEAR),
 Help => ("Help & keyboard", INFO), Diagnostics => ("Runtime capabilities", CODE), Catalog => ("Component catalog", GRID),
 Reset => ("Reset sample session", TRASH), Notices => ("Notice history", CLOCK), NewFolder => ("New folder", NEW_FOLDER),
 Rename => ("Rename selection", RENAME), Preview => ("Calculate preview", IMAGE), SavePreset => ("Save preset to session", CHECK),
 SaveNote => ("Save note to session", CHECK), Review => ("Review sample change", LAYERS), ApplyAppearance => ("Apply appearance", CHECK),
 Dark => ("Dark preview", MOON), Light => ("Light preview", SUN), Smaller => ("Smaller text", CHEVRON_LEFT), Larger => ("Larger text", CHEVRON_RIGHT),
 Up => ("Parent folder", CHEVRON_LEFT), OpenAsset => ("Open selection", FOLDER_OPEN), Move => ("Move selection", FOLDER),
 CommitName => ("Save name", CHECK), CancelName => ("Cancel naming", CLOSE), Close => ("Close / cancel", CLOSE),
 Keep => ("Keep appearance", CHECK), Revert => ("Revert appearance", CHEVRON_LEFT), SaveLeave => ("Save and leave", CHECK),
 DiscardLeave => ("Discard and leave", TRASH), ApplyReview => ("Apply sample change", CHECK), ConfirmReset => ("Reset this session", TRASH),
 ToggleSort => ("Reverse name order", SORT), AppendParagraph => ("Append a paragraph", ADD), Context => ("Selection actions", MORE), Tooltip => ("What is this control?", INFO),
}
pub fn info(command: Command) -> &'static CommandInfo {
    COMMANDS
        .iter()
        .find(|c| c.command == command)
        .expect("exhaustive command registry")
}
pub fn from_key(key: u16) -> Option<Command> {
    COMMANDS
        .iter()
        .find(|c| c.command as u16 == key)
        .map(|c| c.command)
}
