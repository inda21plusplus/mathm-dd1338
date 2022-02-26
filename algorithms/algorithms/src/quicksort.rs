fn partition<T>(xs: &mut [T]) -> usize
where
    T: Ord,
{
    let pivot = 0;
    let mut lo = 1;
    let mut hi = xs.len() - 1;
    while lo < hi {
        if xs[lo] > xs[pivot] && xs[hi] < xs[pivot] {
            xs.swap(lo, hi);
        }
        if xs[lo] <= xs[pivot] {
            lo += 1;
        }
        if xs[hi] >= xs[pivot] {
            hi -= 1;
        }
    }
    if xs[hi] < xs[pivot] {
        xs.swap(pivot, hi);
        hi
    } else {
        xs.swap(pivot, hi - 1);
        hi - 1
    }
}

pub fn quicksort<T>(xs: &mut [T])
where
    T: Ord,
{
    match xs.len() {
        0 | 1 => {}
        2 => {
            if xs[0] > xs[1] {
                xs.swap(0, 1);
            }
        }
        _ => {
            let mid = partition(xs);
            quicksort(&mut xs[..mid]);
            quicksort(&mut xs[mid + 1..]);
        }
    }
}
