// https://open.kattis.com/problems/bank

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut ints = input
        .split(|c| c == '\n' || c == ' ')
        .map(|i| i.parse::<usize>().unwrap());
    let mut get = || ints.next().unwrap();

    let (n, t) = (get(), get());
    const N: usize = 10000;
    const T: usize = 47;

    let mut q: Vec<_> = (0..n).map(|_| (get(), get())).collect();
    q.sort_by_key(|p| p.1);

    let mut m = [[0; T]; N];

    for p in 0..n {
        for x in 0..t {
            let skip = if p > 0 { m[p - 1][x] } else { 0 };
            let same = if x > 0 { m[p][x - 1] } else { 0 };
            let serve = if x <= q[p].1 {
                q[p].0 + if x > 0 { m[p - 1][x - 1] } else { 0 }
            } else {
                0
            };
            m[p][x] = skip.max(same).max(serve);
        }
    }

    println!("{}", m[n - 1][t - 1]);
}
