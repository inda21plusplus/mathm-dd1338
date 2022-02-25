// https://open.kattis.com/problems/ultraquicksort

use std::{
    collections::HashMap,
    io::{stdin, Read},
};

use algorithms::FenwickTree;

trait Sorted {
    fn sorted(self) -> Self;
}

impl Sorted for Vec<usize> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut xs: Vec<usize> = input.lines().skip(1).map(|x| x.parse().unwrap()).collect();

    let compressed: HashMap<usize, usize> = xs
        .clone()
        .sorted()
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect();

    xs.iter_mut().for_each(|x| *x = compressed[x]);

    // xs.iter().max().unwrap() < 500 000

    let mut fw = FenwickTree::<[usize; 500000]>::new();

    let sum: usize = xs
        .iter()
        .rev()
        .map(|&x| {
            fw.add(x, 1);
            fw.prefix_sum(x)
        })
        .sum();

    println!("{}", sum);
}
