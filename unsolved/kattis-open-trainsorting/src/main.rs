// https://open.kattis.com/problems/trainsorting

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut ints = input.lines().map(|l| l.parse::<u16>().unwrap());
    let mut get = || ints.next().unwrap();

    let n = get();
    // const N: u16 = 2000;

    let ws = (0..n).map(|_| get());
}
