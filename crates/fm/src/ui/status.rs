//! The status bar at the bottom of the window, and the notification stack.
//!
//! The stack is drawn above the status bar: every notification is a rounded
//! card with an icon, a title and an optional body. A click on a card
//! dismisses that notification, so the stack never covers more of the window
//! than it has to.

use mirui::prelude::*;
use mirui::ui::IgnoreHitTest;
use mirui::ui::OffscreenRender;
use mirui::ui::widgets::{DropShadow, Icon, ProgressBar, Text};

use crate::icons;
use crate::model::{FmState, NoticeKind};
use crate::text;
use crate::theme;
use crate::ui::kit::{pad, pad_h};

/// The height of the status bar, in logical pixels.
///
/// The notification stack keeps this much distance from the bottom edge, so
/// it never covers the status bar.
pub const BAR_HEIGHT: i32 = 26;

/// Item count and free space.
#[compose]
pub fn bar(fm: model!(FmState)) -> Entity {
    ui! {
        Row (
            width: Dimension::percent(100),
            height: 26,
            bg_color: theme::TOOLBAR,
            align: AlignItems::Center,
            column_gap: 10,
            padding: pad_h(12)
        ) {
            Text (
                text: ${ { let _ = fm.shown(); let _ = fm.total(); fm.count_label() } },
                font_size: text::SMALL,
                text_color: ColorToken::OnSurfaceVariant
            )
            Text (
                text: ${ { let _ = fm.sort(); fm.sort_label() } },
                font_size: text::SMALL,
                text_color: ColorToken::OnSurfaceVariant
            )
            View (grow: 1.0)
            Text (
                text: "128.4 GB available",
                font_size: text::SMALL,
                text_color: ColorToken::OnSurfaceVariant
            )
            ProgressBar (
                width: 110,
                height: 5,
                value: 0.62,
                track_color: ColorToken::Outline,
                fill_color: ColorToken::Primary
            )
        }
    }
}

/// The notifications, stacked in the bottom right corner.
#[compose]
pub fn notices(fm: model!(FmState)) -> Entity {
    ui! {
        View (
            position: Position::Absolute,
            left: 0,
            top: 0,
            width: Dimension::percent(100),
            height: Dimension::percent(100),
            align: AlignItems::FlexEnd,
            justify: JustifyContent::FlexEnd,
            padding: crate::ui::kit::pad(0, 16, BAR_HEIGHT + 12, 0),
            visible: ${ fm.notice_count() > 0 }
        ) [
            // The layer covers the whole window, so it must not swallow the
            // clicks underneath it. Only the cards are hit targets.
            IgnoreHitTest
        ] {
            Column (
                width: Dimension::percent(100),
                height: Dimension::percent(100),
                align: AlignItems::FlexEnd,
                justify: JustifyContent::FlexEnd,
                row_gap: 8
            ) {
                // Notification 0
                View (
                    width: 320,
                    height: ${ { let _ = fm.notice_seq(); if fm.notice_body(0).is_empty() { 38 } else { 54 } } },
                    visible: ${ { let _ = fm.notice_seq(); fm.notice_count() > 0 } }
                ) {
                    View (
                        id: "notice_0",
                        position: Position::Absolute,
                        left: 0,
                        top: 0,
                        width: 320,
                        height: ${ { let _ = fm.notice_seq(); if fm.notice_body(0).is_empty() { 38 } else { 54 } } },
                        border_radius: 10,
                        bg_color: theme::SURFACE,
                        border_color: theme::SEPARATOR,
                        border_width: 1,
                        padding: pad(10, 12, 10, 12)
                    ) [
                        OffscreenRender::new(),
                        HitTarget
                    ] {
                        Row (
                            width: Dimension::percent(100),
                            height: ${ { let _ = fm.notice_seq(); if fm.notice_body(0).is_empty() { 18 } else { 34 } } },
                            align: AlignItems::Center,
                            column_gap: 10
                        ) {
                            notice_icon (fm, 0)
                            Column (
                                grow: 1.0,
                                height: ${ { let _ = fm.notice_seq(); if fm.notice_body(0).is_empty() { 14 } else { 32 } } },
                                row_gap: 2
                            ) {
                                Text (
                                    text: ${ { let _ = fm.notice_seq(); fm.notice_title(0) } },
                                    width: Dimension::percent(100),
                                    font_size: text::SMALL,
                                    text_color: ColorToken::OnSurface
                                )
                                Text (
                                    text: ${ { let _ = fm.notice_seq(); fm.notice_body(0) } },
                                    width: Dimension::percent(100),
                                    font_size: text::SMALL,
                                    text_color: ColorToken::OnSurfaceVariant,
                                    visible: ${ { let _ = fm.notice_seq(); !fm.notice_body(0).is_empty() } }
                                )
                            }
                        }
                    } on Tap {
                        fm.dismiss(0);
                    }
                    View (
                        id: "notice_shadow_0",
                        position: Position::Absolute,
                        left: 0,
                        top: 0,
                        width: 320,
                        height: ${ { let _ = fm.notice_seq(); if fm.notice_body(0).is_empty() { 38 } else { 54 } } }
                    ) [
                        DropShadow::new(id("notice_0"))
                            .with_blur_radius(Fixed::from_int(10))
                            .with_offset(Fixed::ZERO, Fixed::from_int(3))
                            .with_opacity(80)
                    ]
                }
                // Notification 1
                View (
                    width: 320,
                    height: ${ { let _ = fm.notice_seq(); if fm.notice_body(1).is_empty() { 38 } else { 54 } } },
                    visible: ${ { let _ = fm.notice_seq(); fm.notice_count() > 1 } }
                ) {
                    View (
                        id: "notice_1",
                        position: Position::Absolute,
                        left: 0,
                        top: 0,
                        width: 320,
                        height: ${ { let _ = fm.notice_seq(); if fm.notice_body(1).is_empty() { 38 } else { 54 } } },
                        border_radius: 10,
                        bg_color: theme::SURFACE,
                        border_color: theme::SEPARATOR,
                        border_width: 1,
                        padding: pad(10, 12, 10, 12)
                    ) [
                        OffscreenRender::new(),
                        HitTarget
                    ] {
                        Row (
                            width: Dimension::percent(100),
                            height: ${ { let _ = fm.notice_seq(); if fm.notice_body(1).is_empty() { 18 } else { 34 } } },
                            align: AlignItems::Center,
                            column_gap: 10
                        ) {
                            notice_icon (fm, 1)
                            Column (
                                grow: 1.0,
                                height: ${ { let _ = fm.notice_seq(); if fm.notice_body(1).is_empty() { 14 } else { 32 } } },
                                row_gap: 2
                            ) {
                                Text (
                                    text: ${ { let _ = fm.notice_seq(); fm.notice_title(1) } },
                                    width: Dimension::percent(100),
                                    font_size: text::SMALL,
                                    text_color: ColorToken::OnSurface
                                )
                                Text (
                                    text: ${ { let _ = fm.notice_seq(); fm.notice_body(1) } },
                                    width: Dimension::percent(100),
                                    font_size: text::SMALL,
                                    text_color: ColorToken::OnSurfaceVariant,
                                    visible: ${ { let _ = fm.notice_seq(); !fm.notice_body(1).is_empty() } }
                                )
                            }
                        }
                    } on Tap {
                        fm.dismiss(1);
                    }
                    View (
                        id: "notice_shadow_1",
                        position: Position::Absolute,
                        left: 0,
                        top: 0,
                        width: 320,
                        height: ${ { let _ = fm.notice_seq(); if fm.notice_body(1).is_empty() { 38 } else { 54 } } }
                    ) [
                        DropShadow::new(id("notice_1"))
                            .with_blur_radius(Fixed::from_int(10))
                            .with_offset(Fixed::ZERO, Fixed::from_int(3))
                            .with_opacity(80)
                    ]
                }
                // Notification 2
                View (
                    width: 320,
                    height: ${ { let _ = fm.notice_seq(); if fm.notice_body(2).is_empty() { 38 } else { 54 } } },
                    visible: ${ { let _ = fm.notice_seq(); fm.notice_count() > 2 } }
                ) {
                    View (
                        id: "notice_2",
                        position: Position::Absolute,
                        left: 0,
                        top: 0,
                        width: 320,
                        height: ${ { let _ = fm.notice_seq(); if fm.notice_body(2).is_empty() { 38 } else { 54 } } },
                        border_radius: 10,
                        bg_color: theme::SURFACE,
                        border_color: theme::SEPARATOR,
                        border_width: 1,
                        padding: pad(10, 12, 10, 12)
                    ) [
                        OffscreenRender::new(),
                        HitTarget
                    ] {
                        Row (
                            width: Dimension::percent(100),
                            height: ${ { let _ = fm.notice_seq(); if fm.notice_body(2).is_empty() { 18 } else { 34 } } },
                            align: AlignItems::Center,
                            column_gap: 10
                        ) {
                            notice_icon (fm, 2)
                            Column (
                                grow: 1.0,
                                height: ${ { let _ = fm.notice_seq(); if fm.notice_body(2).is_empty() { 14 } else { 32 } } },
                                row_gap: 2
                            ) {
                                Text (
                                    text: ${ { let _ = fm.notice_seq(); fm.notice_title(2) } },
                                    width: Dimension::percent(100),
                                    font_size: text::SMALL,
                                    text_color: ColorToken::OnSurface
                                )
                                Text (
                                    text: ${ { let _ = fm.notice_seq(); fm.notice_body(2) } },
                                    width: Dimension::percent(100),
                                    font_size: text::SMALL,
                                    text_color: ColorToken::OnSurfaceVariant,
                                    visible: ${ { let _ = fm.notice_seq(); !fm.notice_body(2).is_empty() } }
                                )
                            }
                        }
                    } on Tap {
                        fm.dismiss(2);
                    }
                    View (
                        id: "notice_shadow_2",
                        position: Position::Absolute,
                        left: 0,
                        top: 0,
                        width: 320,
                        height: ${ { let _ = fm.notice_seq(); if fm.notice_body(2).is_empty() { 38 } else { 54 } } }
                    ) [
                        DropShadow::new(id("notice_2"))
                            .with_blur_radius(Fixed::from_int(10))
                            .with_offset(Fixed::ZERO, Fixed::from_int(3))
                            .with_opacity(80)
                    ]
                }
            }
        }
    }
}

/// The notification stack under its older name.
///
/// The shell still calls [`toast`]: the stack took the place of the single
/// message that used to live there.
#[compose]
pub fn toast(fm: model!(FmState)) -> Entity {
    ui! {
        notices (fm)
    }
}

/// One notification: an icon, a title and an optional body.
///
/// The card grows a line when the notification has a body: a plain view has
/// no height of its own, so the height is spelled out.
#[compose]
fn notice_card(fm: model!(FmState), index: usize) -> Entity {
    ui! {
        View (
            width: 320,
            height: ${ { let _ = fm.notice_seq(); if fm.notice_body(index).is_empty() { 38 } else { 54 } } },
            border_radius: 10,
            bg_color: theme::SURFACE,
            border_color: theme::SEPARATOR,
            border_width: 1,
            padding: pad(10, 12, 10, 12),
            visible: ${ { let _ = fm.notice_seq(); fm.notice_count() > index as u32 } }
        ) [
            HitTarget
        ] {
            Row (
                width: Dimension::percent(100),
                height: ${ { let _ = fm.notice_seq(); if fm.notice_body(index).is_empty() { 18 } else { 34 } } },
                align: AlignItems::Center,
                column_gap: 10
            ) {
                notice_icon (fm, index)
                Column (
                    grow: 1.0,
                    height: ${ { let _ = fm.notice_seq(); if fm.notice_body(index).is_empty() { 14 } else { 32 } } },
                    row_gap: 2
                ) {
                    Text (
                        text: ${ { let _ = fm.notice_seq(); fm.notice_title(index) } },
                        width: Dimension::percent(100),
                        font_size: text::SMALL,
                        text_color: ColorToken::OnSurface
                    )
                    Text (
                        text: ${ { let _ = fm.notice_seq(); fm.notice_body(index) } },
                        width: Dimension::percent(100),
                        font_size: text::SMALL,
                        text_color: ColorToken::OnSurfaceVariant,
                        visible: ${ { let _ = fm.notice_seq(); !fm.notice_body(index).is_empty() } }
                    )
                }
            }
        } on Tap {
            fm.dismiss(index);
        }
    }
}

/// The icon of a notification: one per kind.
#[compose]
fn notice_icon(fm: model!(FmState), index: usize) -> Entity {
    ui! {
        View (width: 18, height: 18) {
            match ${ { let _ = fm.notice_seq(); fm.notice_kind(index) } } {
                NoticeKind::Info => {
                    Icon (
                        path: icons::INFO.clone(),
                        color: ColorToken::Primary,
                        width: 18,
                        height: 18
                    )
                }
                NoticeKind::Success => {
                    Icon (
                        path: icons::CHECK_CIRCLE.clone(),
                        color: ColorToken::Success,
                        width: 18,
                        height: 18
                    )
                }
                NoticeKind::Warning => {
                    Icon (
                        path: icons::INFO.clone(),
                        color: ColorToken::Secondary,
                        width: 18,
                        height: 18
                    )
                }
                NoticeKind::Error => {
                    Icon (
                        path: icons::CLOSE.clone(),
                        color: ColorToken::Error,
                        width: 18,
                        height: 18
                    )
                }
            }
        }
    }
}
