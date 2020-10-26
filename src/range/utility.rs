use std::ops::Range;
use num::{PrimInt};

// Utilities ------------------------------------------------------------------------------------- /

/// Converts a string to title case (first letter capitalized and all the rest lower-case).
pub fn to_title_case(content: &str) -> String {
    if content.len() > 1 {
        format!("{}{}", &content[0..1].to_uppercase(), &content[1..].to_lowercase())
    } else if content.len() == 1 {
        content[0..1].to_uppercase()
    } else {
        String::new()
    }
}

/// Gets first range of given query in given content.
pub fn get_range(query: &str, content: &str) -> Option<Range<usize>> {
    let possible_start = content.find(query);
    if let Some(start) = possible_start {
        let end: usize = start + query.len();
        if end > content.len() {
            None
        } else {
            Some(start..end)
        }
    } else {
        None
    }
}

/// Enum to specify the direction of a shift, up or down with amount (magnitude).
pub enum Shift<T> where T: PrimInt {
    /// Shifts a number/range *up* by specified amount.
    Up(T),
    /// Shifts a number/range *down* by specified amount.
    Down(T),
}

impl<T> Shift<T> where T: PrimInt {

    /// Applies the shift to a single number, producing a new number shifted by that amount.
    /// Returns `None` if any overflow occurs.
    fn apply_to_number(&self, number: T) -> Option<T> {
        match self {
            Shift::Up(amount) => number.checked_add(amount),
            Shift::Down(amount) => number.checked_sub(amount),
        }
    }

    /// Applies the shift to a range, producing a new range shifted by that amount.
    /// Returns `None` if any overflow occurs.
    fn apply_to_range(&self, range: Range<T>) -> Option<Range<T>> {
        let Range { start, end  } = range;
        let (new_start, new_end) = match self {
            Shift::Up(amount) => (start.checked_add(amount), end.checked_add(amount)),
            Shift::Down(amount) => (start.checked_sub(amount), end.checked_sub(amount)),
        };
        if let (Some(new_start), Some(new_end)) = (new_start, new_end) {
            Some(new_start..new_end)
        } else {
            None
        }
    }
}

/// Creates a new range with the start and end values shifted by the given amount.
///
/// ## Example:
/// ```
/// use query_range::{shift_range, Shift};
///
/// let test_str = "this is a test";
///
/// let range = 0..5;
/// assert_eq!(shift_range(range, Shift::Up(2)), Some(2..7));
///
/// let range = 0..5;
/// assert_eq!(shift_range(range, Shift::Up(20)), Some(20..25));
/// ```
pub fn shift_range<T>(range: Range<T>, shift: Shift<T>) -> Option<Range<T>> where T: PrimInt {
    shift.apply_to_range(range)
}

/// Creates a new range with the start and end values shifted by the given amount *and* checks that
/// the range is valid in the given content. If new range falls outside of the given content, `None`
/// will be returned.
///
/// ## Example:
/// ```
/// use query_range::{shift_range_in_content, Shift};
///
/// let test_str = "this is a test";
///
/// let range = 0..5;
/// assert_eq!(shift_range_in_content(range, Shift::Up(2), test_str), Some(2..7));
///
/// let range = 0..5;
/// assert_eq!(shift_range_in_content(range, Shift::Up(20), test_str), None);
/// ```
pub fn shift_range_in_content<T>(range: Range<T>, shift: Shift<T>, content: &str) -> Option<Range<T>> where T: PrimInt {
    let new_range = shift_range(range, shift);
    if let Some(new_range) = new_range {
        if is_within(content, &new_range) {
            Some(new_range)
        } else {
            None
        }
    } else {
        None
    }
}

/// Checks if a closed range exists in given string content.
///
/// ## Example:
/// ```
/// use query_range::is_within;
///
/// let test_str = "this is a test";
///
/// let range = 0..2;
/// assert_eq!(is_within(test_str, &range), true);
///
/// let range02 = 20..25;
/// assert_eq!(is_within(test_str, &range02), false);
/// ```
pub fn is_within<T>(content: &str, range: &Range<T>) -> bool where T: PrimInt {
    let range_end = range.end.to_usize();
    if let Some(range_end) = range_end {
        if range_end > content.len() {
            false
        } else {
            true
        }
    } else {
        false
    }
}

// Tests ----------------------------------------------------------------------------------------- /

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_convert_to_title_case() {
        assert_eq!(to_title_case("fooBarBaz"), "Foobarbaz");
        assert_eq!(to_title_case("f"), "F");
        assert_eq!(to_title_case(""), "");
    }

    #[test]
    fn can_apply_a_shift_to_a_number() {
        let number = 5;
        let shift = Shift::Up(2);
        assert_eq!(shift.apply_to_number(number), Some(7));
        let number = 8;
        let shift = Shift::Down(3);
        assert_eq!(shift.apply_to_number(number), Some(5));
    }

    #[test]
    fn can_apply_a_shift_to_a_range() {
        let range = 0..5;
        let shift = Shift::Up(2);
        assert_eq!(shift.apply_to_range(range), Some(2..7));
        let range = 4..7;
        let shift = Shift::Down(3);
        assert_eq!(shift.apply_to_range(range), Some(1..4));
    }

    #[test]
    fn can_shift_a_range() {
        let range = 0..5;
        let new_range = shift_range(range, Shift::Up(2));
        assert_eq!(new_range, Some(2..7));
        assert_ne!(shift_range(new_range.unwrap(), Shift::Down(3)), Some(0..1));
    }

    #[test]
    fn is_within_is_true_when_range_is_within() {
        let test_str = "012345";
        let range = 0..2;
        assert_eq!(is_within(test_str, &range), true);
    }

    #[test]
    fn is_within_is_false_when_range_end_is_longer_than_string() {
        let test_str = "012345";
        let range = 2..7;
        assert_eq!(is_within(test_str, &range), false);
    }
}