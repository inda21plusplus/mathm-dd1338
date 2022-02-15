use crate::binary_search;

pub fn longest_increasing_subsequence<T>(xs: &[T]) -> Vec<usize>
where
    T: Ord,
{
    let mut p = vec![0; xs.len()]; // p[i] comes before i in a lis ending in xs[i]
    let mut m = vec![0; xs.len() + 1]; // m[j] is last i in lis of len j among all xs[..i]

    let mut l = 0;
    for i in 0..xs.len() {
        let j = binary_search(1..l + 1, |j| xs[m[j]] < xs[i]);

        m[j] = i;
        p[i] = m[j - 1];

        l = l.max(j);
    }

    let mut lis = vec![0; l];
    let mut x = m[l];
    for i in (0..l).rev() {
        lis[i] = x;
        x = p[x];
    }

    lis
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            longest_increasing_subsequence(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).as_slice(),
        );
        assert_eq!(1, longest_increasing_subsequence(&[1; 10]).len());
        assert_eq!(
            [0, 1, 5, 6, 8],
            longest_increasing_subsequence(&[5, 19, 5, 81, 50, 28, 29, 1, 83, 23]).as_slice(),
        );
        assert_eq!(
            [1, 2, 4, 7],
            longest_increasing_subsequence(&[10, 5, 8, 3, 9, 4, 12, 11]).as_slice(),
        );
    }
}
