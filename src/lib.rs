//! # Query Range Iterator
//!
//! This package provides an iterator (conforming to `Iterator`) which finds all ranges of a query
//! within the searched content. The iterator can be collected, mapped or used manually
//! (by calling the `next()` method until no further result is returned).
//!
//! This also exports several range utilities for use with strings.

// Public exports -------------------------------------------------------------------------------- /


pub use range::query_range_iterator::QueryRangeItr;
pub use range::utility;
pub use range::utility::{get_range, Shift, shift_range, shift_range_in_content, is_within};

// Modules --------------------------------------------------------------------------------------- /

mod range;
