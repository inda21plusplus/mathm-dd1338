// https://open.kattis.com/problems/fenwick

use std::fmt::Write;
use std::io::{stdin, Read};

use algorithms::FenwickTree;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut nq = lines
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .map(|l| l.trim().parse::<usize>().unwrap());
    let (n, q) = (nq.next().unwrap(), nq.next().unwrap());
    let mut output = String::with_capacity(q * 5);

    let mut fw = FenwickTree::<Vec<isize>>::new(n);
    for _ in 0..q {
        let mut line = lines.next().unwrap().trim().split(' ');

        let op = line.next().unwrap();
        let i = line.next().unwrap().parse::<usize>().unwrap();
        match op {
            "+" => {
                let d = line.next().unwrap().parse::<isize>().unwrap();
                fw.add(i, d);
            }
            "?" => writeln!(output, "{}", fw.prefix_sum(i)).unwrap(),
            _ => (),
        }
    }
    print!("{}", output);
}
