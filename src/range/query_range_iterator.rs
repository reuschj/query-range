use std::ops::Range;
use std::cmp::min;
use super::utility::{get_range, Shift, shift_range, shift_range_in_content, is_within };

/// Iterates all found query within given content.
pub struct QueryRangeItr<'a> {
    inverted: bool,
    query: &'a str,
    current_content: &'a str,
    full_content: &'a str,
    removed_count: usize,
}

// Basic implementation -------------------------------------------------------------------------- /

impl<'a> QueryRangeItr<'a> {

    fn new_base(content: &'a str, query: &'a str, inverted: bool) -> QueryRangeItr<'a> {
        Self {
            inverted,
            query,
            current_content: content,
            full_content: content,
            removed_count: 0,
        }
    }

    pub fn new(content: &'a str, query: &'a str) -> QueryRangeItr<'a> {
        Self::new_base(content, query, false)
    }

    pub fn new_inverted(content: &'a str, query: &'a str) -> QueryRangeItr<'a> {
        Self::new_base(content, query, true)
    }

    /// Gets the next range that matches the given query.
    fn next_standard(&mut self) -> Option<Range<usize>> {
        let current_content = self.current_content;
        // let query = self.query;
        // let start_i = current_content.find(query);
        let possible_range = get_range(self.query, current_content);
        if let Some(range) = possible_range {
            if is_within(current_content, &range) {
                let next_start = range.end;
                let possible_range = shift_range_in_content(range, Shift::Up(self.removed_count), self.full_content);
                if let Some(range) = possible_range {
                    let start_len = current_content.len();
                    self.current_content = &current_content[next_start..];
                    self.removed_count += start_len - self.current_content.len();
                    Some(range)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Gets the next range that doesn't match the given query.
    fn next_inverted(&mut self) -> Option<Range<usize>> {
        let current_content = self.current_content;
        let start_index: usize = 0;
        let len = current_content.len();
        let range = get_range(self.query, current_content).unwrap_or(len..len);
        let end_index = range.start;
        let next_start: usize = min(range.end, len);
        let possible_range = shift_range(start_index..end_index, Shift::Up(self.removed_count));
        self.current_content = &current_content[next_start..];
        let start_length = len;
        self.removed_count += start_length - self.current_content.len();
        if current_content.len() > 0 {
            possible_range
        } else {
            None
        }
    }
}

// Iterator implementation ----------------------------------------------------------------------- /

impl<'a> Iterator for QueryRangeItr<'a> {
    type Item = Range<usize>;

    /// Gets next range of the query in the content.
    fn next(&mut self) -> Option<Self::Item> {
        if self.inverted {
            self.next_inverted()
        } else {
            self.next_standard()
        }
    }
}

// Tests ----------------------------------------------------------------------------------------- /

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
