#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Split {
    pub first: u32,
    pub second: u32,
    pub constrained: bool,
}
/// Panel sizes never exceed available space. If both minima cannot fit, preserve
/// the first proportionally rather than overflowing or producing negatives.
pub fn split(available: u32, requested: u32, first_min: u32, second_min: u32) -> Split {
    let minima = first_min as u64 + second_min as u64;
    let constrained = minima > available as u64;
    let first = if constrained {
        ((available as u64 * first_min as u64) / minima) as u32
    } else {
        requested.clamp(first_min, available - second_min)
    };
    Split {
        first,
        second: available - first,
        constrained,
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
/// Anchored surface placement with flip and viewport containment. All
/// intermediate arithmetic is wide; oversize content is clipped to viewport.
pub fn place(anchor: Rect, viewport: Rect, width: u32, height: u32, gap: u32) -> Rect {
    let w = width.min(viewport.width);
    let h = height.min(viewport.height);
    let min_x = viewport.x as i64;
    let min_y = viewport.y as i64;
    let max_x = min_x + viewport.width as i64 - w as i64;
    let max_y = min_y + viewport.height as i64 - h as i64;
    let below = anchor.y as i64 + anchor.height as i64 + gap as i64;
    let above = anchor.y as i64 - gap as i64 - h as i64;
    let y = if below <= max_y { below } else { above };
    Rect {
        x: (anchor.x as i64)
            .clamp(min_x, max_x)
            .clamp(i32::MIN as i64, i32::MAX as i64) as i32,
        y: y.clamp(min_y, max_y)
            .clamp(i32::MIN as i64, i32::MAX as i64) as i32,
        width: w,
        height: h,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn impossible_minima_never_overflow() {
        assert_eq!(
            split(100, 200, 80, 120),
            Split {
                first: 40,
                second: 60,
                constrained: true
            }
        );
        assert_eq!(split(0, 0, 0, 0).first, 0);
        let s = split(u32::MAX, u32::MAX, u32::MAX, u32::MAX);
        assert_eq!(s.first as u64 + s.second as u64, u32::MAX as u64);
    }
    #[test]
    fn clamps_both_panel_limits() {
        assert_eq!(split(500, 450, 100, 100).first, 400);
        assert_eq!(split(500, 10, 100, 100).first, 100);
    }
    #[test]
    fn popover_flips_and_clamps() {
        assert_eq!(
            place(
                Rect {
                    x: 95,
                    y: 90,
                    width: 5,
                    height: 10
                },
                Rect {
                    x: 0,
                    y: 0,
                    width: 100,
                    height: 100
                },
                40,
                30,
                4
            ),
            Rect {
                x: 60,
                y: 56,
                width: 40,
                height: 30
            }
        );
    }
    #[test]
    fn oversized_popup_fits_viewport() {
        assert_eq!(
            place(
                Rect {
                    x: 0,
                    y: 0,
                    width: 1,
                    height: 1
                },
                Rect {
                    x: 5,
                    y: 6,
                    width: 10,
                    height: 20
                },
                100,
                100,
                0
            ),
            Rect {
                x: 5,
                y: 6,
                width: 10,
                height: 20
            }
        );
    }
}
