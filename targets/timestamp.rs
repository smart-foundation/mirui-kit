//! The wall clock name of a capture.
//!
//! Both hosts write a screenshot, and both name it after the moment it was
//! taken, so a second capture never overwrites the first: the name carries
//! the year, the month, the day, the hour, the minute and the second.
//!
//! The stamp is the UTC wall clock. `SystemTime` carries no zone, and the
//! standard library has no time zone database, so the name says what the
//! clock says and not what the calendar on the wall says.
//!
//! The module is shared by the two hosts: both are `std` binaries, and a
//! file that both include is cheaper than a crate that both depend on.

use std::time::{SystemTime, UNIX_EPOCH};

/// The parts of a wall clock stamp.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Stamp {
    /// The year, for example 2026.
    pub year: i64,
    /// The month, from 1 to 12.
    pub month: u32,
    /// The day of the month, from 1 to 31.
    pub day: u32,
    /// The hour, from 0 to 23.
    pub hour: u32,
    /// The minute, from 0 to 59.
    pub minute: u32,
    /// The second, from 0 to 59.
    pub second: u32,
}

/// The stamp of a moment in time.
///
/// A moment before the epoch is stamped as the epoch itself: a capture
/// cannot happen before the machine booted.
pub fn stamp(time: SystemTime) -> Stamp {
    let seconds = match time.duration_since(UNIX_EPOCH) {
        Ok(elapsed) => elapsed.as_secs() as i64,
        Err(_) => 0,
    };
    let days = seconds.div_euclid(86_400);
    let rest = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    Stamp {
        year,
        month,
        day,
        hour: (rest / 3_600) as u32,
        minute: ((rest % 3_600) / 60) as u32,
        second: (rest % 60) as u32,
    }
}

/// The stamp as text: `2026-10-22-13-04-55`.
pub fn format_stamp(stamp: &Stamp) -> String {
    format!(
        "{:04}-{:02}-{:02}-{:02}-{:02}-{:02}",
        stamp.year, stamp.month, stamp.day, stamp.hour, stamp.minute, stamp.second
    )
}

/// The name of a capture file: `screenshot-2026-10-22-13-04-55.png`.
pub fn capture_file(prefix: &str, time: SystemTime) -> String {
    format!("{prefix}-{}.png", format_stamp(&stamp(time)))
}

/// The stamp a capture name ends with.
///
/// The name is a prefix and six numbers: `screenshot-2026-10-22-13-04-55`.
/// The prefix may itself contain a dash, so the stamp is read from the end.
// The hosts only write names; the tests read them back.
#[allow(dead_code)]
pub fn parse_stamp(name: &str) -> Option<Stamp> {
    let mut parts = name.rsplitn(7, '-');
    let second = parts.next()?.parse().ok()?;
    let minute = parts.next()?.parse().ok()?;
    let hour = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;
    let month = parts.next()?.parse().ok()?;
    let year = parts.next()?.parse().ok()?;
    // The name has to have a prefix: a bare stamp is not a capture name.
    parts.next()?;
    Some(Stamp {
        year,
        month,
        day,
        hour,
        minute,
        second,
    })
}

/// The civil date of a day count since 1970-01-01.
///
/// The count is turned into the four hundred year era it falls in, the day
/// of the era into a year and a day of the year, and the day of the year
/// into a month and a day of the month.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    // The days are shifted so that the era starts on 0000-03-01.
    let shifted = days + 719_468;
    // The division truncates towards zero, so the negative side is moved
    // down by a whole era first.
    let era = if shifted >= 0 {
        shifted / 146_097
    } else {
        (shifted - 146_096) / 146_097
    };
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    // March is the first month, so the leap day lands at the end of the year.
    let month_of_year = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_of_year + 2) / 5 + 1) as u32;
    let month = if month_of_year < 10 {
        month_of_year + 3
    } else {
        month_of_year - 9
    } as u32;
    (if month <= 2 { year + 1 } else { year }, month, day)
}
