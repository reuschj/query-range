# Query Range

This package provides an iterator (conforming to `Iterator`) which finds all ranges of a query within the searched content. The iterator can be collected to an `Vec<Range<usize>>`, mapped or used manually (by calling the `next()` method until no further result is returned). 

This also exports several range utilities for use with strings.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
query-range = "0.1.0"
```

## Iteration

To use as an iterator:
```rust
 use query_range::QueryRangeItr;

let query = "needle";
let content = "haystackneedlehaystackneedlehaystack";
let mut occurrences = QueryRangeItr::new(query, content);
while let Some(next) = occurrences.next() {
    println!("{}", &content[next]);
}
```

## Collecting to an Array

This collects all ranges to an array of string indices.
```rust
use query_range::QueryRangeItr;

let query = "needle";
let content = "haystackneedlehaystackneedlehaystack";
let mut occurrences = QueryRangeItr::new(query, content);
let needles: Vec<String> = occurrences.map(|range| String::from(&content[range])).collect();
```

## Collecting to Strings

This collects all ranges and extracts the string from the original content at those indices.
```rust
use query_range::QueryRangeItr;

let query = "needle";
let content = "haystackneedlehaystackneedlehaystack";
let mut occurrences = QueryRangeItr::new(query, content);
let needles: Vec<String> = occurrences.collect_strings();
```

## Transforming the query
If the end goal is performing a transform on all the found query, this is a convenience static method to do so.
```rust
use query_range::QueryRangeItr;

let query = "needle";
let content = "haystackneedlehaystackneedlehaystack";
let result = QueryRangeItr::transform_query(query, content, |it| it.to_uppercase());
```

## Reassembling the content
If you also need to transform the non-query content, you can with this static method:
```rust
use query_range::QueryRangeItr;
use query_range::utility::to_title_case;

let query = "needle";
let content = "haystackneedlehaystackneedlehaystack";
let result = QueryRangeItr::transform_all(
    query,
    content,
    |it| it.to_uppercase(), // query transform
    |it| to_title_case(it), // non-query transform
);
```
