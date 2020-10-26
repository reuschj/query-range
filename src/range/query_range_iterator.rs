use std::ops::Range;
use std::cmp::min;
use super::utility::{ get_range, Shift, shift_range, shift_range_in_content, is_within };

/// Iterates all found query within given content.
///
/// ### Examples
///
/// **Iteration:**
/// ```
/// use query_range::QueryRangeItr;
///
/// let query = "needle";
/// let content = "haystackneedlehaystackneedlehaystack";
/// let mut occurrences = QueryRangeItr::new(query, content);
/// while let Some(next) = occurrences.next() {
///     println!("{}", &content[next]);
/// }
/// ```
///
/// **Collecting to an Array:**
/// ```
/// use query_range::QueryRangeItr;
///
/// let query = "needle";
/// let content = "haystackneedlehaystackneedlehaystack";
/// let mut occurrences = QueryRangeItr::new(query, content);
/// let needles: Vec<String> = occurrences.map(|range| String::from(&content[range])).collect();
/// ```
/// **Collecting to Strings:**
/// ```
/// use query_range::QueryRangeItr;
///
/// let query = "needle";
/// let content = "haystackneedlehaystackneedlehaystack";
/// let mut occurrences = QueryRangeItr::new(query, content);
/// let needles: Vec<String> = occurrences.collect_strings();
/// ```
///
/// **Transforming the query:**
/// ```
/// use query_range::QueryRangeItr;
///
/// let query = "needle";
/// let content = "haystackneedlehaystackneedlehaystack";
/// let result = QueryRangeItr::transform_query(query, content, |it| it.to_uppercase());
/// ```
///
/// **Reassembling the content:**
/// ```
/// use query_range::QueryRangeItr;
/// use query_range::utility::to_title_case;
///
/// let query = "needle";
/// let content = "haystackneedlehaystackneedlehaystack";
/// let result = QueryRangeItr::transform_all(
///     query,
///     content,
///     |it| it.to_uppercase(), // query transform
///     |it| to_title_case(it), // non-query transform
/// );
/// ```
pub struct QueryRangeItr<'a> {
    inverted: bool,
    query: &'a str,
    current_content: &'a str,
    full_content: &'a str,
    removed_count: usize,
}

// ----------------------------------------------------------------------------------------------- /

/// Basic implementation
impl<'a> QueryRangeItr<'a> {

    /// Private, common initializer method.
    fn new_base(query: &'a str, content: &'a str, inverted: bool) -> QueryRangeItr<'a> {
        Self {
            inverted,
            query,
            current_content: content,
            full_content: content,
            removed_count: 0,
        }
    }

    /// Creates a new iterator with given content or query which will iterate each *found* instance
    /// of the query.
    pub fn new(query: &'a str, content: &'a str) -> QueryRangeItr<'a> {
        Self::new_base(query, content, false)
    }

    /// Creates a new iterator with given content or query which will iterate the content in
    /// between each *found* instance of the query.
    pub fn new_inverted(query: &'a str, content: &'a str) -> QueryRangeItr<'a> {
        Self::new_base(query, content, true)
    }

    /// Collects all iterated ranges and builds an array of strings from the original content at those ranges
    pub fn collect_strings(&mut self) -> Vec<String> {
        let content = self.full_content;
        self.map(|range| String::from(&content[range])).collect()
    }

    /// Gets the next range that matches the given query.
    fn next_standard(&mut self) -> Option<Range<usize>> {
        let current_content = self.current_content;
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

// ----------------------------------------------------------------------------------------------- /

/// Transform methods
impl<'a> QueryRangeItr<'a> {

    /// Reassembles content, but transforms the query content or the non-query content
    ///
    /// **Example:**
    /// ```
    /// use query_range::QueryRangeItr;
    ///
    /// let query = "needle";
    /// let content = "haystackneedlehaystackneedlehaystack";
    /// let result = QueryRangeItr::transform(query, content, |it| it.to_uppercase(), false);
    /// assert_eq!(result, "haystackNEEDLEhaystackNEEDLEhaystack");
    /// ```
    ///
    /// **Parameters:**
    /// - `query` - The search query
    /// - `content` - The content to look for the query in
    /// - `transform` -  A transform closure to run on all query content
    /// - `invert` - If `true`, applies the transform to the non-query content
    pub fn transform<T>(
        query: &'a str,
        content: &'a str,
        transform: T,
        invert: bool,
    ) -> String where T: Fn(&str) -> String {
        let selects = Self::new_base(query, content, invert);
        let non_selects = Self::new_base(query, content, !invert);
        let transform_each = |range: Range<usize>| {
            let start = range.clone().start;
            let original = &content[range];
            let value = transform(original);
            (value, start)
        };
        let transform_rest = |range: Range<usize>| {
            let start = range.clone().start;
            let original = &content[range];
            let value = String::from(original);
            (value, start)
        };
        let mut selected_subs: Vec<(String, usize)>  = selects.map(|range| transform_each(range)).collect();
        let mut non_selected_subs: Vec<(String, usize)> = non_selects.map(|range| transform_rest(range)).collect();
        selected_subs.append(&mut non_selected_subs);
        let mut merged = selected_subs;
        merged.sort_unstable_by(|s1,s2|  s1.1.cmp(&s2.1));
        let strings: Vec<String> = merged.iter().map(|s| s.0.clone()).collect();
        strings.join("")
    }

    /// Reassembles content, but transforms the query content
    ///
    /// **Example:**
    /// ```
    /// use query_range::QueryRangeItr;
    ///
    /// let query = "needle";
    /// let content = "haystackneedlehaystackneedlehaystack";
    /// let result = QueryRangeItr::transform_query(query, content, |it| it.to_uppercase());
    /// assert_eq!(result, "haystackNEEDLEhaystackNEEDLEhaystack");
    /// ```
    ///
    /// **Parameters:**
    /// - `query` - The search query
    /// - `content` - The content to look for the query in
    /// - `transform` -  A transform closure to run on all query content
    pub fn transform_query<T>(
        query: &'a str,
        content: &'a str,
        transform: T,
    ) -> String where T: Fn(&str) -> String {
        Self::transform(query, content, transform, false)
    }

    /// Reassembles content, but transforms the non-query content
    ///
    /// **Example:**
    /// ```
    /// use query_range::QueryRangeItr;
    ///
    /// let query = "needle";
    /// let content = "haystackneedlehaystackneedlehaystack";
    /// let result = QueryRangeItr::transform_other(query, content, |it| it.to_uppercase());
    /// assert_eq!(result, "HAYSTACKneedleHAYSTACKneedleHAYSTACK");
    /// ```
    ///
    /// **Parameters:**
    /// - `query` - The search query
    /// - `content` - The content to look for the query in
    /// - `transform` -  A transform closure to run on all non-query content
    pub fn transform_other<T>(
        query: &'a str,
        content: &'a str,
        transform: T,
    ) -> String where T: Fn(&str) -> String {
        Self::transform(query, content, transform, true)
    }

    /// Reassembles content, but transforms both the query content and the non-query content
    ///
    /// **Example:**
    /// ```
    /// use query_range::QueryRangeItr;
    /// use query_range::utility::to_title_case;
    ///
    /// let query = "needle";
    /// let content = "haystackneedlehaystackneedlehaystack";
    /// let result = QueryRangeItr::transform_all(
    ///     query,
    ///     content,
    ///     |it| it.to_uppercase(),
    ///     |it| to_title_case(it),
    /// );
    /// assert_eq!(result, "HaystackNEEDLEHaystackNEEDLEHaystack");
    /// ```
    ///
    /// **Parameters:**
    /// - `query` - The search query
    /// - `content` - The content to look for the query in
    /// - `transform_query` -  A transform closure to run on all query content
    /// - `transform_non_query` - If `true`, applies the transform to the non-query content
    pub fn transform_all<TQ, TNQ>(
        query: &'a str,
        content: &'a str,
        transform_query: TQ,
        transform_non_query: TNQ,
    ) -> String
        where
            TQ: Fn(&str) -> String,
            TNQ: Fn(&str) -> String,
    {
        let transformed_content = Self::transform(query, content, &transform_query, false);
        let transformed_query = transform_query(query);
        let transformed = QueryRangeItr::transform(&transformed_query, &transformed_content, transform_non_query, true);
        transformed
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
    use super::*;
    use super::super::utility::to_title_case;

    #[test]
    fn can_iterate_iter() {
        let query = "needle";
        let content = "haystackneedlehaystackneedlehaystack";
        let mut occurrences = QueryRangeItr::new(query, content);
        while let Some(next) = occurrences.next() {
            assert_eq!(String::from(&content[next]), "needle");
        }
    }

    #[test]
    fn can_map_iter() {
        let query = "needle";
        let content = "haystackneedlehaystackneedlehaystack";
        let occurrences = QueryRangeItr::new(query, content);
        let needles: Vec<String> = occurrences.map(|range| String::from(&content[range])).collect();
        assert_eq!(needles.len(), 2);
        needles.iter().for_each(|n| assert_eq!(n, "needle"));
    }

    #[test]
    fn can_collect_strings() {
        let query = "needle";
        let content = "haystackneedlehaystackneedlehaystack";
        let mut occurrences = QueryRangeItr::new(query, content);
        let needles: Vec<String> = occurrences.collect_strings();
        assert_eq!(needles.len(), 2);
        needles.iter().for_each(|n| assert_eq!(n, "needle"));
    }

    #[test]
    fn can_transform_query() {
        let query = "needle";
        let content = "haystackneedlehaystackneedlehaystack";
        let result = QueryRangeItr::transform_query(query, content, |it| it.to_uppercase());
        assert_eq!(result, "haystackNEEDLEhaystackNEEDLEhaystack");
    }

    #[test]
    fn can_reassemble_string() {
        let query = "needle";
        let content = "haystackneedlehaystackneedlehaystack";
        let result = QueryRangeItr::transform_all(
            query,
            content,
            |it| it.to_uppercase(),
            |it| to_title_case(it),
        );
        assert_eq!(result, "HaystackNEEDLEHaystackNEEDLEHaystack");
    }
}
