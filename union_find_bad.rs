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
    let a = root(parent, a);
    let b = root(parent, b);
    parent[a] = b;
}

fn main() {
    let mut line = String::new();
    stdin().lock().read_line(&mut line).ok();
    let mut nq = line.trim().split(' ').map(|l| l.parse::<usize>().unwrap());
    let (n, q) = (nq.next().unwrap(), nq.next().unwrap());
    let mut parent: Vec<usize> = (0..n).collect();
    for _ in 0..q {
        line.clear();
        stdin().lock().read_line(&mut line).ok();
        let op = &line[0..1];
        let mut ab = line[2..].split(' ').map(|l| l.trim().parse::<usize>().unwrap());
        let (a, b) = (ab.next().unwrap(), ab.next().unwrap());
        match op {
            "=" => join(&mut parent, a, b),
            "?" => println!(
                "{}",
                if root(&mut parent, a) == root(&mut parent, b) { "yes" } else { "no" }
            ),
            _ => (),
        }
    }
}