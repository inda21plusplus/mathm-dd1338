use std::io::{stdin, BufRead};

fn root(parent: &mut [usize], mut node: usize) -> usize {
    let mut r = node;
    while r != parent[r] {
        r = parent[r];
    }
    while node != r {
        let tmp = node;
        node = parent[node];
        parent[tmp] = r;
    }
    r
}

fn join(parent: &mut [usize], a: usize, b: usize) {
    parent[root(parent, a)] = root(parent, b);
}

fn main() {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).ok();
    let mut nq = line.trim().split(' ').map(|l| l.parse::<usize>().unwrap());
    let (n, q) = (nq.next().unwrap(), nq.next().unwrap());
    let mut parent: Vec<usize> = (0..n).collect();
    loop {

    }
}