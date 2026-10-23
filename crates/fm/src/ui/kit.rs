//! Small shared building blocks for the chrome.

use alloc::string::ToString;

use mirui::anim::{ease, Motion, PlayMode, Tween};
use mirui::prelude::*;
use mirui::render::command::DrawCommand;
use mirui::render::path::Path;
use mirui::types::{Fixed, Rect};
use mirui::ui::dirty::Dirty;
use mirui::ui::widgets::{Icon, Text};

use crate::font::MEDIUM;
use crate::icons;
use crate::model::{FmState, ViewMode};
use crate::text;
use crate::theme;

/// Padding with explicit edges.
pub const fn pad(top: i32, right: i32, bottom: i32, left: i32) -> Padding {
    Padding {
        top: Dimension::px(top),
        right: Dimension::px(right),
        bottom: Dimension::px(bottom),
        left: Dimension::px(left),
    }
}

/// Padding on the horizontal edges only.
pub const fn pad_h(v: i32) -> Padding {
    pad(0, v, 0, v)
}

/// Padding on the vertical edges only.
pub const fn pad_v(v: i32) -> Padding {
    pad(v, 0, v, 0)
}

/// A small rounded pill that displays a count.
#[compose]
pub fn badge(count: u32) -> Entity {
    ui! {
        View (
            width: Dimension::Auto,
            height: 18,
            border_radius: 9,
            bg_color: theme::BADGE,
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            padding: pad_h(6),
        ) [
            HitTarget
        ] {
            Text (
                text: ${ count.to_string() },
                font_size: text::SMALL,
                text_color: theme::BADGE_TEXT,
            )
        }
    }
}

/// A centered icon + label for empty grids.
#[compose]
pub fn empty_state(icon: &'static Path, title: &'static str, body: &'static str) -> Entity {
    ui! {
        Column (
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            row_gap: 10,
        ) {
            Icon (
                path: icon.clone(),
                color: ColorToken::OnSurfaceVariant,
                width: 48,
                height: 48
            )
            Text (
                text: title,
                font: MEDIUM,
                font_size: text::BODY,
                text_color: ColorToken::OnSurface
            )
            Text (
                text: body,
                font_size: text::SMALL,
                text_color: ColorToken::OnSurfaceVariant
            )
        }
    }
}

/// A shimmering placeholder shown while content loads.
#[component]
pub struct Skeleton {
    pub width: i32,
    pub height: i32,
}

/// The current shimmer offset (0..1) of a skeleton entity.
pub struct ShimmerT(pub Fixed);

impl Component for ShimmerT {}

/// Advances each skeleton's looping shimmer motion and writes the offset.
#[animate(component = ShimmerMotion)]
fn apply_shimmer(
    entity: entity!(),
    value: motion!(),
    track: component!(mut ShimmerT),
    commands: commands!(),
) {
    track.0 = value;
    commands.insert(entity, Dirty);
}

/// Seeds a looping shimmer motion on each skeleton that has none yet.
#[system(order = Animation, expect = Skeleton)]
fn seed_shimmer(
    skeletons: query!(Entity, Skeleton; without ShimmerMotion;),
    commands: commands!(),
) {
    for (entity, _) in skeletons {
        commands.insert(entity, ShimmerMotion(Motion::Tween(
            Tween::new(Fixed::ZERO, Fixed::ONE, 1600, &ease::IN_OUT_QUAD, PlayMode::Loop),
        )));
        commands.insert(entity, ShimmerT(Fixed::ZERO));
    }
}

#[view(
    component = Skeleton,
    name = "Skeleton",
    priority = 50,
    systems = [seed_shimmer::system(), ShimmerMotion::system()]
)]
fn skeleton_view(
    rows: query!(Skeleton, optional ShimmerT),
    entity: entity!(),
    theme: res!(Theme),
    paint: paint!(),
) {
    let Some((_skeleton, shimmer)) = rows.get(entity) else {
        return;
    };
    let state = paint.state();
    let rect = paint.rect();
    let t = shimmer.map(|s| s.0).unwrap_or(Fixed::ZERO);

    paint.draw(&DrawCommand::Fill {
        area: rect,
        transform: paint.transform(),
        quad: paint.quad(),
        color: ThemedColor::Token(theme::SKELETON).resolve_in(&theme, state),
        radius: Fixed::from_int(6),
        opa: 255,
    });

    let bar_w = (rect.w / Fixed::from_int(3)).max(Fixed::ZERO);
    let bar_x = rect.x + (rect.w - bar_w) * t;
    let bar_rect = Rect {
        x: bar_x,
        y: rect.y,
        w: bar_w,
        h: rect.h,
    };
    paint.draw(&DrawCommand::Fill {
        area: bar_rect,
        transform: paint.transform(),
        quad: paint.quad(),
        color: ThemedColor::Token(theme::SKELETON_SHIMMER).resolve_in(&theme, state),
        radius: Fixed::from_int(6),
        opa: 255,
    });
}

/// A hover text bubble.
#[compose]
pub fn tooltip(text: &'static str) -> Entity {
    ui! {
        View (
            width: Dimension::Auto,
            height: 22,
            border_radius: 6,
            bg_color: theme::TOOLTIP_BG,
            padding: pad_h(8),
            align: AlignItems::Center,
        ) [
            HitTarget
        ] {
            Text (
                text: text,
                font_size: text::SMALL,
                text_color: theme::TOOLTIP_TEXT,
            )
        }
    }
}

/// Icon grid / list segmented control for the view switcher.
#[compose]
pub fn segmented_control(fm: model!(FmState)) -> Entity {
    ui! {
        Row (
            width: 62,
            height: 26,
            border_radius: 7,
            bg_color: theme::SEGMENTED_BG,
            column_gap: 2,
            padding: pad_v(2),
        ) {
            View (
                id: "toolbar_view_icons",
                width: 28,
                height: 22,
                border_radius: 5,
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                bg_color: ${ if fm.view() == ViewMode::Icons { theme::SEGMENTED_SELECTED } else { theme::CLEAR } },
            ) [
                HitTarget
            ] {
                Icon (path: icons::GRID.clone(), color: ColorToken::OnSurface, width: 14, height: 14)
            } on Tap {
                fm.set_view(ViewMode::Icons);
            }
            View (
                id: "toolbar_view_list",
                width: 28,
                height: 22,
                border_radius: 5,
                align: AlignItems::Center,
                justify: JustifyContent::Center,
                bg_color: ${ if fm.view() == ViewMode::List { theme::SEGMENTED_SELECTED } else { theme::CLEAR } },
            ) [
                HitTarget
            ] {
                Icon (path: icons::LIST.clone(), color: ColorToken::OnSurface, width: 14, height: 14)
            } on Tap {
                fm.set_view(ViewMode::List);
            }
        }
    }
}
