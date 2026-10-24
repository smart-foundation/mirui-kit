use alloc::{string::String, vec::Vec};
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckState {
    #[default]
    Unchecked,
    Checked,
    Mixed,
}
impl CheckState {
    pub fn toggle(self) -> Self {
        if self == Self::Checked {
            Self::Unchecked
        } else {
            Self::Checked
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct FieldState {
    pub required: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub touched: bool,
    pub submitted: bool,
    errors: Vec<String>,
}
impl FieldState {
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
    pub fn invalid(&self) -> bool {
        !self.disabled && !self.errors.is_empty()
    }
    pub fn show_errors(&self) -> bool {
        self.invalid() && (self.touched || self.submitted)
    }
    pub fn validate(&mut self, value: &str, errors: impl IntoIterator<Item = String>) {
        self.errors.clear();
        if self.disabled {
            return;
        }
        if self.required && value.trim().is_empty() {
            self.errors.push("This field is required.".into());
        }
        self.errors.extend(errors);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumberError {
    Empty,
    Invalid,
    OutOfRange,
    InvalidBounds,
}
/// Fixed decimal integer units; avoids floating-point drift and works no_std.
#[derive(Clone, Debug)]
pub struct NumberState {
    value: i64,
    min: i64,
    max: i64,
    step: i64,
    pub disabled: bool,
    pub read_only: bool,
}
impl NumberState {
    pub fn new(value: i64, min: i64, max: i64, step: i64) -> Result<Self, NumberError> {
        if min > max || step <= 0 {
            return Err(NumberError::InvalidBounds);
        }
        if value < min || value > max {
            return Err(NumberError::OutOfRange);
        }
        Ok(Self {
            value,
            min,
            max,
            step,
            disabled: false,
            read_only: false,
        })
    }
    pub fn value(&self) -> i64 {
        self.value
    }
    pub fn commit(&mut self, draft: &str) -> Result<bool, NumberError> {
        if self.disabled || self.read_only {
            return Ok(false);
        }
        let text = draft.trim();
        if text.is_empty() {
            return Err(NumberError::Empty);
        }
        let value = text.parse::<i64>().map_err(|_| NumberError::Invalid)?;
        if value < self.min || value > self.max {
            return Err(NumberError::OutOfRange);
        }
        let changed = value != self.value;
        self.value = value;
        Ok(changed)
    }
    pub fn step(&mut self, direction: i32, page: bool) {
        if self.disabled || self.read_only {
            return;
        }
        let delta = self.step.saturating_mul(if page { 10 } else { 1 });
        self.value = if direction < 0 {
            self.value.saturating_sub(delta)
        } else {
            self.value.saturating_add(delta)
        }
        .clamp(self.min, self.max);
    }
    pub fn boundary(&mut self, end: bool) {
        if !self.disabled && !self.read_only {
            self.value = if end { self.max } else { self.min };
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}
impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Option<Self> {
        if year == 0
            || year > 9999
            || !(1..=12).contains(&month)
            || day == 0
            || day > Self::days_in_month(year, month)
        {
            return None;
        }
        Some(Self { year, month, day })
    }
    pub fn parts(self) -> (u16, u8, u8) {
        (self.year, self.month, self.day)
    }
    pub fn days_in_month(year: u16, month: u8) -> u8 {
        match month {
            2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => 0,
        }
    }
    pub fn add_days(self, amount: i32) -> Option<Self> {
        // Calendar UI uses short increments; cap untrusted arithmetic work.
        if amount.unsigned_abs() > 36600 {
            return None;
        }
        let (mut y, mut m, mut d) = self.parts();
        for _ in 0..amount.unsigned_abs() {
            if amount > 0 {
                if d < Self::days_in_month(y, m) {
                    d += 1;
                } else {
                    d = 1;
                    if m < 12 {
                        m += 1;
                    } else {
                        m = 1;
                        y = y.checked_add(1)?;
                    }
                }
            } else if d > 1 {
                d -= 1;
            } else {
                if m > 1 {
                    m -= 1;
                } else {
                    m = 12;
                    y = y.checked_sub(1)?;
                }
                d = Self::days_in_month(y, m);
            }
        }
        Self::new(y, m, d)
    }
    pub fn add_months(self, amount: i32) -> Option<Self> {
        let month = (self.year as i32 - 1)
            .checked_mul(12)?
            .checked_add(self.month as i32 - 1)?
            .checked_add(amount)?;
        if !(0..9999 * 12).contains(&month) {
            return None;
        }
        let y = (month / 12 + 1) as u16;
        let m = (month % 12 + 1) as u8;
        Self::new(y, m, self.day.min(Self::days_in_month(y, m)))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}
impl Time {
    pub fn new(hour: u8, minute: u8, second: u8) -> Option<Self> {
        if hour < 24 && minute < 60 && second < 60 {
            Some(Self {
                hour,
                minute,
                second,
            })
        } else {
            None
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
impl Rgba {
    pub fn parse_hex(text: &str) -> Option<Self> {
        let s = text.strip_prefix('#').unwrap_or(text);
        if !s.is_ascii() || !matches!(s.len(), 6 | 8) {
            return None;
        }
        Some(Self {
            red: u8::from_str_radix(&s[0..2], 16).ok()?,
            green: u8::from_str_radix(&s[2..4], 16).ok()?,
            blue: u8::from_str_radix(&s[4..6], 16).ok()?,
            alpha: if s.len() == 8 {
                u8::from_str_radix(&s[6..8], 16).ok()?
            } else {
                255
            },
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_checkbox_activates_checked() {
        assert_eq!(CheckState::Mixed.toggle(), CheckState::Checked);
        assert_eq!(CheckState::Checked.toggle(), CheckState::Unchecked);
    }
    #[test]
    fn validation_is_not_premature_and_disabled_is_exempt() {
        let mut f = FieldState {
            required: true,
            ..Default::default()
        };
        f.validate("", []);
        assert!(f.invalid());
        assert!(!f.show_errors());
        f.touched = true;
        assert!(f.show_errors());
        f.disabled = true;
        assert!(!f.invalid());
    }
    #[test]
    fn number_invalid_draft_does_not_destroy_value() {
        let mut n = NumberState::new(50, 0, 100, 5).unwrap();
        assert_eq!(n.commit("-"), Err(NumberError::Invalid));
        assert_eq!(n.commit("101"), Err(NumberError::OutOfRange));
        assert_eq!(n.value(), 50);
        n.step(1, true);
        assert_eq!(n.value(), 100);
        n.step(1, false);
        assert_eq!(n.value(), 100);
    }
    #[test]
    fn read_only_and_extreme_number_steps() {
        let mut n = NumberState::new(0, i64::MIN, i64::MAX, i64::MAX).unwrap();
        n.step(1, true);
        assert_eq!(n.value(), i64::MAX);
        n.read_only = true;
        n.step(-1, false);
        assert_eq!(n.value(), i64::MAX);
    }
    #[test]
    fn leap_century_and_month_clamping() {
        assert!(Date::new(1900, 2, 29).is_none());
        assert!(Date::new(2000, 2, 29).is_some());
        assert_eq!(
            Date::new(2024, 1, 31)
                .unwrap()
                .add_months(1)
                .unwrap()
                .parts(),
            (2024, 2, 29)
        );
        assert_eq!(
            Date::new(2024, 3, 1).unwrap().add_days(-1).unwrap().parts(),
            (2024, 2, 29)
        );
    }
    #[test]
    fn date_bounds_and_large_inputs_are_safe() {
        assert!(Date::new(1, 1, 1).unwrap().add_days(-1).is_none());
        assert!(Date::new(9999, 12, 31).unwrap().add_days(1).is_none());
        assert!(
            Date::new(2024, 1, 1)
                .unwrap()
                .add_months(i32::MAX)
                .is_none()
        );
        assert!(Date::new(2024, 1, 1).unwrap().add_days(i32::MIN).is_none());
    }
    #[test]
    fn time_and_color_validation() {
        assert!(Time::new(24, 0, 0).is_none());
        assert_eq!(Rgba::parse_hex("#ff008080").unwrap().alpha, 128);
        assert!(Rgba::parse_hex("é00000").is_none());
        assert!(Rgba::parse_hex("#zz0000").is_none());
    }
}
