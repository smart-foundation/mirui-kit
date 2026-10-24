//! Real Mirui compositions; no alternate renderer or showcase-only button.
use crate::{
    commands::{COMMANDS, Command, info},
    model::Studio,
};
use alloc::{format, string::ToString};
use mirui::prelude::*;
use mirui::ui::{
    OffscreenRender,
    widgets::{DropShadow, Text},
};
use mirui_kit::{
    InputGate, font, icons,
    split::SplitPane,
    layout::sidebar_toggle,
    scrollbar::ScrollBar,
    theme,
    widgets::{action, action_with_intent, field, number, text_size, toggle},
};
use mirui_kit::{
    controls::{checkbox, choice_group, tree_view},
    feedback::{Severity, alert},
    forms::number_field,
    table::data_table,
    choices::list_box,
    calendar::calendar,
};
use studio_core::{Route, Surface};

#[compose]
fn command(state: model!(Studio), key: Command, overlay: bool) -> Entity {
    let descriptor = info(key);
    ui! {
        action (
            state.actions(),
            key as u16,
            descriptor.label.to_string(),
            descriptor.icon,
            overlay
        )
    }
}
/// Geometry markers shared by native presentation regression tests.
#[component]
pub struct NavigationItem;
#[component]
pub struct PreviewCard;
#[component]
pub struct TaskActions;

/// Bounded task actions avoid turning every command into a full-width bar.
/// The label and target grow with text scale; containing rows wrap natively.
#[compose]
fn compact_command(state: model!(Studio), key: Command, width: u16) -> Entity {
    ui! {
        View (width:${text_size(state.actions().text_scale(),width)},height:${text_size(state.actions().text_scale(),state.actions().control_height())}) {
            command(state,key,false)
        }
    }
}

#[compose]
fn heading(state: model!(Studio), title: &'static str, body: &'static str) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 10) {
            Text (
                title,
                font: font::SEMIBOLD,
                font_size: ${ text_size(state.actions().text_scale(), font::TITLE_SIZE) },
                text_color: ColorToken::OnSurface
            )
            Text (
                body,
                width: Dimension::percent(100),
                paragraph: mirui::ui::widgets::ParagraphStyle::default(),
                font: font::MEDIUM,
                font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
        }
    }
}

#[compose]
pub fn shell(state: model!(Studio)) -> Entity {
    ui! {
        Column (width:Dimension::percent(100), height:Dimension::percent(100), bg_color:theme::SURFACE, align:AlignItems::Center, justify:JustifyContent::Center) {
            View (width:1180, height:720) {
                View (position:Position::Absolute,left:0,top:0,width:1180,height:720) [
                    DropShadow::new(id("studio_window")).with_blur_radius(18).with_offset(0,6).with_opacity(70)
                ]
                Column (id:"studio_window",width:1180,height:720,bg_color:ColorToken::Surface,border_radius:14) [OffscreenRender::new()] {
                    Column (width:Dimension::percent(100),grow:1.0) [InputGate { actions:state.actions(), overlay:false }] {
                        Row (width:Dimension::percent(100),height:${text_size(state.actions().text_scale(),40)},padding:Padding::all(4),align:AlignItems::Center,column_gap:12,bg_color:theme::TOOLBAR) {
                            sidebar_toggle(state.desktop_chrome(),state.actions(),90,false)
                            Text ("NALEDZI / STUDIO",grow:1.0,font:font::SEMIBOLD,font_size:${text_size(state.actions().text_scale(), font::SECTION_SIZE)},text_color:ColorToken::OnSurface)
                            View (width:196,height:${text_size(state.actions().text_scale(),state.actions().control_height())}) { command(state,(Command::Palette),false) }
                            View (width:160,height:${text_size(state.actions().text_scale(),state.actions().control_height())}) { command(state,(Command::Help),false) }
                        }
                        Row (width:Dimension::percent(100),grow:1.0) {
                            Column (width:${text_size(state.actions().text_scale(),state.desktop_chrome().sidebar_width() as u16)},visible:${state.desktop_chrome().sidebar_visible()},height:Dimension::percent(100),padding:Padding::all(12),row_gap:4,bg_color:ColorToken::SurfaceVariant) {
                                walk (0..6).collect::<alloc::vec::Vec<usize>>() with index {
                                    View (width:Dimension::percent(100),height:${text_size(state.actions().text_scale(),state.actions().control_height())},border_radius:8,bg_color:${if state.route() as usize==index {theme::SELECTION} else {theme::CLEAR}}) [NavigationItem] {
                                        action_with_intent(state.actions(),COMMANDS[index].command as u16,COMMANDS[index].label.to_string(),COMMANDS[index].icon,false,(mirui_kit::style::Intent::Quiet))
                                    }
                                }
                                View (grow:1.0)
                                Text ("MIRUI 0.51\nSESSION FIXTURES",font:font::MEDIUM,font_size:${text_size(state.actions().text_scale(), font::CAPTION_SIZE)},text_color:ColorToken::OnSurfaceVariant)
                            }
                            Column (grow:1.0,height:Dimension::percent(100)) {
                              View (width:Dimension::percent(100),grow:1.0) {
                                Scroll (id:"studio_main_scroll",width:Dimension::percent(100),height:Dimension::percent(100),padding:Padding::all(24)) {
                                    Column (width:Dimension::percent(100),row_gap:20) [mirui_kit::sizing::ContentFit] {
                                        if ${state.route()==Route::Overview} { overview(state) }
                                        if ${state.route()==Route::Controls} { controls(state) }
                                        if ${state.route()==Route::Collections} { collections(state) }
                                        if ${state.route()==Route::Workbench} { workbench(state) }
                                        if ${state.route()==Route::Surfaces} { surfaces(state) }
                                        if ${state.route()==Route::Foundations} { foundations(state) }
                                        Text (text:${{let _=state.revision();state.error()}},width:Dimension::percent(100),font:font::MEDIUM,font_size:${text_size(state.actions().text_scale(), font::BODY_SIZE)},text_color:ColorToken::Error)
                                    }
                                }
                                View (position:Position::Absolute,left:@width {width-Fixed::from_int(12)},top:4,width:10,height:@height {height-Fixed::from_int(8)}) [ScrollBar {target:id("studio_main_scroll"),vertical:true,thickness:8}]
                              }
                              Text (text:${{let _=state.revision();state.latest_notice()}},height:${text_size(state.actions().text_scale(),32)},width:Dimension::percent(100),padding:Padding{left:24.into(),right:12.into(),top:4.into(),bottom:4.into()},paragraph:mirui::ui::widgets::ParagraphStyle::label().with_align(mirui::ui::widgets::TextAlign::Start),font:font::MEDIUM,font_size:${text_size(state.actions().text_scale(), font::CAPTION_SIZE)},text_color:ColorToken::OnSurfaceVariant)
                            }
                        }
                    }
                }
                overlay(state)
            }
        }
    }
}
#[compose]
fn overview(state: model!(Studio)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 20) {
            heading (
                state,
                "One kit. Real work.",
                "Explore useful, session-only tasks built from the same components as this shell."
            )
            Row (width: Dimension::percent(100), column_gap: 24) {
                Column (
                    grow: 1.0,
                    row_gap: 12,
                    padding: Padding::all(20),
                    bg_color: ColorToken::SurfaceVariant,
                    border_radius: 12
                ) {
                    Text (
                        "MAKE SOMETHING",
                        font: font::SEMIBOLD,
                        font_size: ${ text_size(state.actions().text_scale(), font::SECTION_SIZE) },
                        text_color: ColorToken::OnSurface
                    )
                    command (state, (Command::Controls), false)
                    command (state, (Command::Collections), false)
                    command (state, (Command::Workbench), false)
                }
                Column (
                    width: 364,
                    row_gap: 12,
                    padding: Padding::all(20),
                    bg_color: ColorToken::SurfaceVariant,
                    border_radius: 12
                ) {
                    Text (
                        "UNDER THE SURFACE",
                        font: font::SEMIBOLD,
                        font_size: ${ text_size(state.actions().text_scale(), font::SECTION_SIZE) },
                        text_color: ColorToken::OnSurface
                    )
                    command (state, (Command::Surfaces), false)
                    command (state, (Command::Foundations), false)
                    command (state, (Command::Catalog), false)
                }
            }
            Text (
                "Honest capabilities",
                font: font::SEMIBOLD,
                font_size: ${ text_size(state.actions().text_scale(), font::SECTION_SIZE) },
                text_color: ColorToken::OnSurface
            )
            Text (
                "Native Mirui documents, selection, gestures, slider and switch. No filesystem, encoder, network or durable persistence is implied. Source and runtime verification are reported separately.",
                width: Dimension::percent(100),
                font: font::MEDIUM,
                font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
            command (state, (Command::Diagnostics), false)
        }
    }
}
#[compose]
fn controls(state: model!(Studio)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 20) {
            heading (
                state,
                "Controls / Export preset",
                "Validate a named preset and calculate a fixture summary. Nothing is encoded or written to disk."
            )
            Row (width: Dimension::percent(100), column_gap: 24, align:AlignItems::FlexStart) {
                Column (grow: 1.0, row_gap: 14) {
                    Text (
                        "Preset name",
                        font: font::MEDIUM,
                        font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                        text_color: ColorToken::OnSurface
                    )
                    field (state.actions(), state.preset(), 100, "Name this preset", false, 0)
                    Text (
                        "Quality",
                        font: font::MEDIUM,
                        font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                        text_color: ColorToken::OnSurface
                    )
                    number (state.actions(), state.quality(), 101)
                    toggle (state.actions(), state.metadata(), 103, "Include sample metadata")
                    Row (width:Dimension::percent(100),wrap:FlexWrap::Wrap,column_gap:8,row_gap:8) [TaskActions] {
                        compact_command(state,(Command::Preview),184)
                        compact_command(state,(Command::SavePreset),218)
                    }
                }
                Column (
                    width: 364,
                    padding: Padding::all(20),
                    row_gap: 16,
                    bg_color: ColorToken::SurfaceVariant,
                    border_radius: 12
                ) [PreviewCard,mirui_kit::sizing::ContentFit] {
                    Text (
                        "PREVIEW CALCULATION",
                        font: font::SEMIBOLD,
                        font_size: ${ text_size(state.actions().text_scale(), font::SECTION_SIZE) },
                        text_color: ColorToken::OnSurface
                    )
                    Text (
                        text: ${
                            {
                                let _ = state.revision();
                                state.preview()
                            }
                        },
                        width: Dimension::percent(100),
                        font: font::MEDIUM,
                        font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                        text_color: ColorToken::OnSurface
                    )
                    command (state, (Command::Tooltip), false)
                }
            }
            heading (state, "Shared control specimens", "Radio-style grouped selection skips unavailable items. Numeric entry validates on Enter or blur; arrow keys step the value.")
            choice_group (state.actions(),state.demo_choices(),300,false,true)
            checkbox (state.actions(),state.demo_check(),301,"Enable optional indexing".into(),false)
            number_field (state.actions(),state.demo_numeric(),302,"Worker count (1–64)".into(),false)
            heading (state,"Option list / Range calendar","Arrow keys move option focus; Space selects. Calendar arrows move days, Page Up/Down changes months, Shift changes years. Dates are local Gregorian values, not a scheduled host task.")
            list_box (state.actions(),state.demo_options(),303,false)
            calendar (state.actions(),state.demo_calendar(),304,false)
            Text (text:${ {let _=state.demo_calendar().revision();state.demo_calendar().selection_label()} },font:font::MEDIUM,font_size:${text_size(state.actions().text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)
            alert (state.actions(),(Severity::Information),"Shared kit specimens".into(),"These controls use the same theme roles and input ownership as the application.".into())
            Text (
                "Recipe: mirui_kit::widgets::{field, number, toggle, action}. Editing any setting cancels the old job. Preview must be requested again.",
                width: Dimension::percent(100),
                font_size: ${ text_size(state.actions().text_scale(), font::CAPTION_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
        }
    }
}
#[compose]
fn collections(state: model!(Studio)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 16) {
            heading (
                state,
                "Collections / Sample assets",
                "Stable IDs, hierarchical folders and inline naming. These are in-memory fixtures, not OS files."
            )
            Text (
                text: ${
                    {
                        let _ = state.revision();
                        state.folder_label()
                    }
                },
                font: font::SEMIBOLD,
                font_size: ${ text_size(state.actions().text_scale(), font::SECTION_SIZE) },
                text_color: ColorToken::OnSurface
            )
            heading (state, "Hierarchy specimen", "Independent in-memory tree. Right/Left expand and collapse; Up/Down move focus; Space toggles selection; Ctrl+A selects visible available nodes.")
            tree_view (state.actions(),state.demo_hierarchy(),310,false)
            heading (state, "Data table specimen", "Read-only text and integer cells with stable row selection. Click a column heading to cycle sorting; Up/Down move focus and Space toggles selection. Header keyboard navigation and virtualization remain pending.")
            data_table (state.actions(),state.demo_table(),320,false)
            field (state.actions(), state.query(), 110, "Filter this folder", false, 0)
            Row (width: Dimension::percent(100), column_gap: 8) {
                View (grow: 1.0) {
                    command (state, (Command::Up), false)
                }
                View (grow: 1.0) {
                    command (state, (Command::NewFolder), false)
                }
                View (grow: 1.0) {
                    command (state, (Command::Options), false)
                }
            }
            Column (width: Dimension::percent(100), row_gap: 6) {
                walk ${
                    {
                        let _ = state.listing_revision();
                        state.rows()
                    }
                } with asset by (asset as u64).wrapping_mul(1_000_003).wrapping_add(state.listing_revision()) {
                    asset_row (state, asset)
                }
                Text (
                    "No matching sample assets",
                    visible: ${
                        {
                            let _ = state.listing_revision();
                            state.rows().is_empty()
                        }
                    },
                    font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                    text_color: ColorToken::OnSurfaceVariant
                )
            }
            if ${
                {
                    let _ = state.revision();
                    state.selected().is_some()
                }
            } {
                Row (width: Dimension::percent(100), column_gap: 8) {
                    View (grow: 1.0) {
                        command (state, (Command::OpenAsset), false)
                    }
                    View (grow: 1.0) {
                        command (state, (Command::Context), false)
                    }
                    View (grow: 1.0) {
                        command (state, (Command::Inspector), false)
                    }
                }
            }
        }
    }
}
#[compose]
fn asset_row(state: model!(Studio), asset: u32) -> Entity {
    ui! {
        Column (
            width: Dimension::percent(100),
            padding: Padding::all(4),
            border_radius: 8,
            row_gap: 8,
            bg_color: ${
                {
                    let _ = state.revision();
                    if state.selected() == Some(asset) {
                        theme::SELECTION
                    } else {
                        ColorToken::Surface
                    }
                }
            }
        ) {
            if ${
                {
                    let _ = state.revision();
                    state.editing() == Some(asset)
                }
            } {
                field (state.actions(), state.name(), 120, "Folder or asset name", false, 0)
                Row (width: Dimension::percent(100), column_gap: 8) {
                    View (grow: 1.0) {
                        command (state, (Command::CommitName), false)
                    }
                    View (grow: 1.0) {
                        command (state, (Command::CancelName), false)
                    }
                }
            } else {
                action (
                    state.actions(),
                    1000 + asset as u16,
                    state.asset_label(asset),
                    if state.is_folder(asset) { &icons::FOLDER } else { &icons::DOCUMENT },
                    false
                )
            }
        }
    }
}
#[compose]
fn workbench(state: model!(Studio)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 16) {
            heading (
                state,
                "Workbench / Session note",
                "A real Mirui TextDocument, native editing and clipboard. Saves last only for this process."
            )
            Text (
                text: ${
                    {
                        let _ = state.revision();
                        state.note_status()
                    }
                },
                font: font::MEDIUM,
                font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
            field (state.actions(), state.note(), 130, "Session note (single line)", false, 0)
            Row (width: Dimension::percent(100), column_gap: 12) {
                View (grow: 1.0) {
                    command (state, (Command::SaveNote), false)
                }
                View (grow: 1.0) {
                    command (state, (Command::AppendParagraph), false)
                }
            }
            Text (
                "Ctrl S saves in session. Leaving with unsaved edits opens a Save / Discard / Cancel guard. Native input is not advertised as a code editor.",
                width: Dimension::percent(100),
                paragraph: mirui::ui::widgets::ParagraphStyle::default(),
                font: font::MEDIUM,
                font_size: ${ text_size(state.actions().text_scale(), font::CAPTION_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
            heading(state,"Resizable workspace","Drag the divider or focus it and use Left/Right. Shift steps faster; Home/End respect panel minima.")
            View (width:Dimension::percent(100),height:180) {
                SplitPane (width:Dimension::percent(100),height:Dimension::percent(100),state:Some(state.demo_split()),actions:Some(state.actions()),key:350u16,overlay:false) {
                    @first {View (width:Dimension::percent(100),height:Dimension::percent(100),padding:Padding::all(12),bg_color:ColorToken::SurfaceVariant) {Text ("Navigation pane",font:font::MEDIUM,font_size:${text_size(state.actions().text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)}}
                    @second {View (width:Dimension::percent(100),height:Dimension::percent(100),padding:Padding::all(12),bg_color:ColorToken::SurfaceVariant) {Text ("Application-owned content",font:font::MEDIUM,font_size:${text_size(state.actions().text_scale(),font::BODY_SIZE)},text_color:ColorToken::OnSurface)}}
                }
            }
        }
    }
}
#[compose]
fn surfaces(state: model!(Studio)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 16) {
            heading (
                state,
                "Surfaces / Safe changes",
                "Review an explicit sample transaction. Backdrops consume dismissal; background input is disabled."
            )
            command (state, (Command::Review), false)
            Text (
                "SURFACE CATALOG",
                font: font::SEMIBOLD,
                font_size: ${ text_size(state.actions().text_scale(), font::SECTION_SIZE) },
                text_color: ColorToken::OnSurface
            )
            Column (width: Dimension::percent(100), row_gap: 8) {
                walk (0..17).collect::<alloc::vec::Vec<usize>>() with index {
                    surface_link (state, index)
                }
            }
        }
    }
}
#[compose]
fn surface_link(state: model!(Studio), index: usize) -> Entity {
    // Surfaces with prerequisites are explained rather than manufactured.
    let surface = Surface::ALL[index];
    let available = !matches!(
        surface,
        Surface::Recovery
            | Surface::Unsaved
            | Surface::Conflict
            | Surface::Inspector
            | Surface::Context
            | Surface::AssetPreview
            | Surface::Move
    );
    ui! {
        View (width:Dimension::percent(100)) {
        if available {
            action (
                state.actions(),
                3000 + index as u16,
                format!("O{:02} / {}", index + 1, surface.title()),
                &icons::LAYERS,
                false
            )
        } else {
            Text (
                format!("O{:02} / {} — open through its real task", index + 1, surface.title()),
                height: 36,
                font: font::MEDIUM,
                font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
        }
        }
    }
}
#[compose]
fn foundations(state: model!(Studio)) -> Entity {
    ui! {
        Column (width: Dimension::percent(100), row_gap: 16) {
            heading (
                state,
                "Foundations / Appearance",
                "Preview a scoped draft. Apply globally, then Keep or Revert within ten seconds."
            )
            Text (
                text: ${
                    {
                        let _ = state.revision();
                        state.appearance_caption()
                    }
                },
                font: font::MEDIUM,
                font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
            Row (width: Dimension::percent(100), column_gap: 12) {
                View (grow: 1.0) {
                    command (state, (Command::Dark), false)
                }
                View (grow: 1.0) {
                    command (state, (Command::Light), false)
                }
                View (grow: 1.0) {
                    command (state, (Command::Smaller), false)
                }
                View (grow: 1.0) {
                    command (state, (Command::Larger), false)
                }
            }
            Column (
                width: Dimension::percent(100),
                padding: Padding::all(24),
                row_gap: 16,
                border_radius: 12,
                bg_color: ${
                    {
                        let _ = state.revision();
                        if state.draft_light() {
                            Color::rgb(248, 249, 252)
                        } else {
                            Color::rgb(24, 27, 34)
                        }
                    }
                }
            ) [mirui_kit::sizing::ContentFit] {
                Text (
                    "Inter / A clear hierarchy",
                    font: font::SEMIBOLD,
                    font_size: ${
                        {
                            let _ = state.revision();
                            state.draft_size() + 8
                        }
                    },
                    text_color: ${
                        {
                            let _ = state.revision();
                            if state.draft_light() {
                                Color::rgb(23, 26, 34)
                            } else {
                                Color::rgb(244, 246, 250)
                            }
                        }
                    }
                )
                Text (
                    "Readable body text. Visible outlines. Semantic roles.",
                    font: font::MEDIUM,
                    font_size: ${
                        {
                            let _ = state.revision();
                            state.draft_size()
                        }
                    },
                    text_color: ${
                        {
                            let _ = state.revision();
                            if state.draft_light() {
                                Color::rgb(23, 26, 34)
                            } else {
                                Color::rgb(244, 246, 250)
                            }
                        }
                    }
                )
            }
            command (state, (Command::ApplyAppearance), false)
            Text (
                "Available static Inter weights: Thin, ExtraLight, Light, Regular, Medium, SemiBold, Bold, ExtraBold, Black. Memory-costed Cargo features select atlases; this is not variable-font support.",
                width: Dimension::percent(100),
                paragraph: mirui::ui::widgets::ParagraphStyle::default(),
                font: font::MEDIUM,
                font_size: ${ text_size(state.actions().text_scale(), font::CAPTION_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
        }
    }
}
#[compose]
fn overlay(state: model!(Studio)) -> Entity {
    ui! {
        View (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: 1180,
            height: 720,
            visible: ${ state.surface().is_some() }
        ) {
            View (
                position: Position::Absolute,
                left: 0,
                top: 0,
                width: 1180,
                height: 720,
                bg_color: theme::DIM
            ) [
                HitTarget,
            ] on Tap { state.actions().activate(Command::Close as u16, true); }
            Column (
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::Center,
                justify: JustifyContent::Center
            ) {
                View (width: 660, height: 580) {
                    View (position: Position::Absolute, left: 0, top: 0, width: 660, height: 580) [
                        DropShadow::new(id("studio_sheet"))
                            .with_blur_radius(14)
                            .with_offset(0, 4)
                            .with_opacity(80),
                    ]
                    Column (
                        id: "studio_sheet",
                        width: 660,
                        height: 580,
                        padding: Padding::all(24),
                        row_gap: 16,
                        bg_color: ColorToken::Surface,
                        border_radius: 14
                    ) [
                        OffscreenRender::new(),
                        HitTarget,
                        InputGate {
                            actions: state.actions(),
                            overlay: true,
                        },
                    ] {
                        Text (
                            text: ${ state.surface().map_or("", Surface::title) },
                            font: font::SEMIBOLD,
                            font_size: ${ text_size(state.actions().text_scale(), font::TITLE_SIZE) },
                            text_color: ColorToken::OnSurface
                        )
                        Scroll (
                            id: "studio_sheet_scroll",
                            width: Dimension::percent(100),
                            grow: 1.0
                        ) {
                            Column (width: Dimension::percent(100), row_gap: 12) {
                                if ${ state.surface() == Some(Surface::Palette) } {
                                    Column (width: Dimension::percent(100), row_gap: 10) {
                                        field (
                                            state.actions(),
                                            state.palette(),
                                            0,
                                            "Filter available commands",
                                            true,
                                            90
                                        )
                                        walk ${
                                            {
                                                let _ = state.revision();
                                                state.palette_rows()
                                            }
                                        } with index by index {
                                            View (
                                                width: Dimension::percent(100),
                                                padding: Padding::all(3),
                                                border_radius: 8,
                                                bg_color: ${
                                                    {
                                                        let _ = state.revision();
                                                        if state.palette_rows().get(state.palette_index()) == Some(&index) {
                                                            theme::SELECTION
                                                        } else {
                                                            ColorToken::Surface
                                                        }
                                                    }
                                                }
                                            ) {
                                                command (state, COMMANDS[index].command, true)
                                            }
                                        }
                                        Text (
                                            "No available commands match.",
                                            visible: ${
                                                {
                                                    let _ = state.revision();
                                                    state.palette_rows().is_empty()
                                                }
                                            },
                                            font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                                            text_color: ColorToken::OnSurfaceVariant
                                        )
                                    }
                                } else {
                                    Text (
                                        text: ${
                                            {
                                                let _ = state.revision();
                                                state.surface_body()
                                            }
                                        },
                                        width: Dimension::percent(100),
                                        font: font::MEDIUM,
                                        font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                                        text_color: ColorToken::OnSurface
                                    )
                                }
                                if ${ state.surface() == Some(Surface::Catalog) } {
                                    Column (width: Dimension::percent(100), row_gap: 12) {
                                        field (
                                            state.actions(),
                                            state.palette(),
                                            0,
                                            "Find a component or API",
                                            true,
                                            0
                                        )
                                        walk ${
                                            {
                                                let _ = state.revision();
                                                state.catalog_rows()
                                            }
                                        } with entry by entry {
                                            catalog_entry (state, entry)
                                        }
                                        Text (
                                            "No matching components",
                                            visible: ${
                                                {
                                                    let _ = state.revision();
                                                    state.catalog_rows().is_empty()
                                                }
                                            },
                                            font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                                            text_color: ColorToken::OnSurfaceVariant
                                        )
                                    }
                                }
                                if ${ state.surface() == Some(Surface::Navigation) } {
                                    Column (width: Dimension::percent(100), row_gap: 8) {
                                        walk (0..6).collect::<alloc::vec::Vec<usize>>() with index {
                                            command (state, COMMANDS[index].command, true)
                                        }
                                    }
                                }
                                if ${ state.surface() == Some(Surface::Unsaved) } {
                                    command (state, (Command::SaveLeave), true)
                                    command (state, (Command::DiscardLeave), true)
                                }
                                if ${ state.surface() == Some(Surface::Confirm) } {
                                    command (state, (Command::ApplyReview), true)
                                }
                                if ${ state.surface() == Some(Surface::Reset) } {
                                    command (state, (Command::ConfirmReset), true)
                                }
                                if ${ state.surface() == Some(Surface::Recovery) } {
                                    command (state, (Command::Keep), true)
                                    command (state, (Command::Revert), true)
                                }
                                if ${ state.surface() == Some(Surface::Options) } {
                                    command (state, (Command::ToggleSort), true)
                                }
                                if ${ state.surface() == Some(Surface::Context) } {
                                    command (state, (Command::OpenAsset), true)
                                    command (state, (Command::Rename), true)
                                    command (state, (Command::Move), true)
                                }
                                if ${ state.surface() == Some(Surface::Move) } {
                                    Column (width: Dimension::percent(100), row_gap: 8) {
                                        walk ${
                                            {
                                                let _ = state.revision();
                                                state.folders()
                                            }
                                        } with folder by folder {
                                            action (
                                                state.actions(),
                                                2000 + folder as u16,
                                                if folder == 0 { "Sample root".into() } else { state.asset_label(folder) },
                                                &icons::FOLDER,
                                                true
                                            )
                                        }
                                    }
                                }
                                Text (
                                    text: ${
                                        {
                                            let _ = state.revision();
                                            state.error()
                                        }
                                    },
                                    font: font::MEDIUM,
                                    font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                                    text_color: ColorToken::Error
                                )
                            }
                        }
                        command (state, (Command::Close), true)
                        View (
                            position: Position::Absolute,
                            left: 638,
                            top: 68,
                            width: 10,
                            height: 420
                        ) [
                            ScrollBar {
                                target: id("studio_sheet_scroll"),
                                vertical: true,
                                thickness: 8,
                            },
                        ]
                    }
                }
            }
        }
    }
}

#[compose]
fn catalog_entry(state: model!(Studio), entry: usize) -> Entity {
    let doc = &crate::catalog::COMPONENTS[entry];
    ui! {
        Column (
            width: Dimension::percent(100),
            row_gap: 8,
            padding: Padding::all(12),
            bg_color: ColorToken::SurfaceVariant,
            border_radius: 8
        ) {
            Text (
                doc.title,
                font: font::SEMIBOLD,
                font_size: ${ text_size(state.actions().text_scale(), font::SECTION_SIZE) },
                text_color: ColorToken::OnSurface
            )
            Text (
                doc.api,
                width: Dimension::percent(100),
                font: font::MEDIUM,
                font_size: ${ text_size(state.actions().text_scale(), font::BODY_SIZE) },
                text_color: ColorToken::OnSurface
            )
            Text (
                doc.source,
                width: Dimension::percent(100),
                font_size: ${ text_size(state.actions().text_scale(), font::CAPTION_SIZE) },
                text_color: ColorToken::OnSurfaceVariant
            )
            action (
                state.actions(),
                doc.route as u16,
                "Open live task".into(),
                info(doc.route).icon,
                true
            )
        }
    }
}
