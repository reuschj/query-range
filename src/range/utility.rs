use std::ops::Range;
use std::cmp::min;

// Utilities ------------------------------------------------------------------------------------- /

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

/// Creates a new range shifted by the given amount
pub fn shift_range(range: Range<usize>, amount: usize, content: Option<&str>) -> Option<Range<usize>> {
    let Range { start, end  } = range;
    let new_start = start + amount;
    let new_end = end + amount;
    let new_range = new_start..new_end;
    if let Some(content) = content {
        if is_within(content, &new_range) {
            Some(new_range)
        } else {
            None
        }
    } else {
        Some(new_range)
    }
}

/// Checks if range exists in given content.
pub fn is_within(content: &str, range: &Range<usize>) -> bool {
    if range.end > content.len() {
        false
    } else {
        true
    }
}