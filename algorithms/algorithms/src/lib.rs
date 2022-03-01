#![feature(test, array_zip)]

mod binary_search;
mod fenwick;
mod longest_increasing_subsequence;
mod quicksort;
mod segtree;
mod union_find;

pub mod geometry;

pub use binary_search::binary_search;
pub use fenwick::FenwickTree;
pub use longest_increasing_subsequence::longest_increasing_subsequence;
pub use quicksort::quicksort;
pub use segtree::SegmentTree;
pub use union_find::UnionFind;
