// https://open.kattis.com/problems/powerstrings

use std::io::{stdin, Read};

struct Divisors(usize, usize);

impl Divisors {
    fn new(n: usize) -> Self {
        Self(n, 0)
    }
}

impl Iterator for Divisors {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.1 > self.0 {
                break None;
            }
            self.1 += 1;
            if self.0 % self.1 == 0 {
                break Some(self.1);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    for line in input.lines().take_while(|&l| l != ".") {
        let mut res = 0;
        'a: for d in Divisors::new(line.len()) {
            for i in (0..line.len()).step_by(d) {
                if line[0..d] != line[i..i + d] {
                    continue 'a;
                }
            }
            res = d;
        }
        println!("{}", res);
    }
}
