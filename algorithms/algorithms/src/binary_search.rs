use std::ops::Range;

/// Given a range r = [i, j)
/// and a predicate p : r -> bool
///     where a < b => P(b) -> P(a)
/// i.e. `r.map(p)` contains none or some occurances of `true` and then none or some occurances of
/// `false`,
/// returns the first x in [i, j) where p(x) == false or j if p(j - 1) is true.
///
/// # Time complexity
/// *O*(log((*r*.*end*-*r*.*start*) * *T*)) where *T* is the time complexity of *p*
///
/// # Examples
/// To find the index of an element in a sorted array:
/// ```rust
/// # use algorithms::binary_search;
/// let xs = [3, 3, 4, 9];
/// assert_eq!(2, binary_search(0..xs.len(), |i| xs[i] < 4));
/// ```
///
/// To find the index at which a given element should be inserted such that a sorted array remains
/// sorted:
/// ```rust
/// # use algorithms::binary_search;
/// # for e in [0, 1, 2, 3, 4, 5, 6] {
/// let mut xs = vec![1, 2, 2, 5];
/// let i = binary_search(0..xs.len(), |i| xs[i] < e);
/// xs.insert(i, e);
/// for i in 1..xs.len() {
///     assert!(xs[i - 1] <= xs[i]);
/// }
/// # }
/// ```
pub fn binary_search<P>(mut r: Range<usize>, p: P) -> usize
where
    P: Fn(usize) -> bool,
{
    while r.start < r.end {
        let mid = r.start + (r.end - r.start) / 2;
        if p(mid) {
            r.start = mid + 1;
        } else {
            r.end = mid;
        }
    }
    r.start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let xs = [0usize; 0];
        assert_eq!(0, binary_search(0..xs.len(), |i| xs[i] < 10));
    }

    #[test]
    fn finds_first() {
        let xs = [1, 4, 5usize];
        assert_eq!(0, binary_search(0..xs.len(), |i| xs[i] < 1));
        let xs = [4, 4, 8usize];
        assert_eq!(0, binary_search(0..xs.len(), |i| xs[i] < 4));
    }

    #[test]
    fn finds_last() {
        let xs = [1, 4, 5usize];
        assert_eq!(2, binary_search(0..xs.len(), |i| xs[i] < 5));
        let xs = [4, 8usize];
        assert_eq!(1, binary_search(0..xs.len(), |i| xs[i] < 8));
    }

    #[test]
    fn finds_middle() {
        let xs = [1, 4, 5usize];
        assert_eq!(1, binary_search(0..xs.len(), |i| xs[i] < 4));
        let xs = [2, 4, 6, 6, 10usize];
        assert_eq!(2, binary_search(0..xs.len(), |i| xs[i] < 6));
    }

    #[test]
    fn between() {
        let xs = [1, 4, 6usize];
        assert_eq!(2, binary_search(0..xs.len(), |i| xs[i] < 5));
        let xs = [2, 4, 6, 6, 10usize];
        assert_eq!(2, binary_search(0..xs.len(), |i| xs[i] < 5));
        let xs = [2, 4, 6, 6, 10usize];
        assert_eq!(4, binary_search(0..xs.len(), |i| xs[i] < 9));
    }
}
