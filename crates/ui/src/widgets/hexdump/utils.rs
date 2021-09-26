use std::ops::Range;

/// Intersects a range with another.
///
/// If the ranges do not intersect, an empty range (`start == end`) is returned, where `start` is
/// the highest lower bound between them.
pub fn range_intersect<T: Copy + PartialOrd + Ord>(a: Range<T>, b: Range<T>) -> Range<T> {
    use std::cmp::{max, min};

    let m = max(a.start, b.start);
    let n = min(a.end, b.end);

    if m > n {
        m..m
    } else {
        m..n
    }
}

/// Restrict a value to a certain interval.
pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: Copy + PartialOrd,
{
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
