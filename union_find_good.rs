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

fn join(parent: &mut [usize], rank: &mut [usize], a: usize, b: usize) {
    let mut a = root(parent, a);
    let mut b = root(parent, b);
    if rank[a] < rank[b] {
        let tmp = a;
        a = b;
        b = tmp;
    }
    parent[a] = b;
    if rank[a] == rank[b] {
        rank[a] += 1;
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_read_to_string(&mut input).ok();
    let mut lines = input.lines();
    let mut nq = lines.next().unwrap().trim().split(' ').map(|l| l.trim().parse::<usize>().unwrap());
    let (n, q) = (nq.next().unwrap(), nq.next().unwrap());
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0usize; n];
    let mut output = String::with_capacity(q * 4);
    for _ in 0..q {
        let line = lines.next().unwrap().trim();
        let op = &line[0..1];
        let mut ab = line[2..].split(' ').map(|l| l.trim().parse::<usize>().unwrap());
        let (a, b) = (ab.next().unwrap(), ab.next().unwrap());
        match op {
            "=" => join(&mut parent, &mut rank, a, b),
            "?" => output.push_str(
                if root(&mut parent, a) == root(&mut parent, b) {
                    "yes\n"
                } else {
                    "no\n"
                },
            ),
            _ => (),
        }
    }
    println!("{}", output);
}