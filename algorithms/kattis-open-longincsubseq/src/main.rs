// https://open.kattis.com/problems/longincsubseq

use std::fmt::Write;
use std::io::{stdin, Read};

use algorithms::longest_increasing_subsequence;

fn main() {
    let input = &mut String::new();
    stdin().lock().read_to_string(input).unwrap();
    let mut lines = input.lines();
    while let Some(_) = lines.next() {
        let xs: Vec<i32> = lines
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();

        let lis = longest_increasing_subsequence(&xs);

        println!("{}", lis.len());
        println!(
            "{}",
            lis.iter()
                .fold(String::with_capacity(10 * lis.len()), |mut acc, x| {
                    write!(acc, "{} ", x).unwrap();
                    acc
                })
        );
    }
}
