// https://open.kattis.com/problems/unionfind

use std::fmt::Write;
use std::io::{stdin, Read};

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
    let mut output = String::with_capacity(q * 4);

    let mut uf = union_find::UnionFind::<Vec<usize>>::new(n);
    for _ in 0..q {
        let line = lines.next().unwrap().trim();
        let op = &line[0..1];
        let mut ab = line[2..]
            .split(' ')
            .map(|l| l.trim().parse::<usize>().unwrap());
        let (a, b) = (ab.next().unwrap(), ab.next().unwrap());
        match op {
            "=" => uf.join(a, b),
            "?" => writeln!(
                output,
                "{}",
                if uf.root(a) == uf.root(b) {
                    "yes"
                } else {
                    "no"
                }
            )
            .unwrap(),
            _ => (),
        }
    }
    print!("{}", output);
}
