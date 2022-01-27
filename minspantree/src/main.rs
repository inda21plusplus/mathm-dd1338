use std::fmt::Write;
use std::io::{stdin, Read};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Edge {
    w: i32,
    u: usize,
    v: usize,
}

fn root(parent: &mut [usize], mut a: usize) -> usize {
    let mut r = a;
    while parent[r] != r {
        r = parent[r];
    }
    while a != r {
        let b = a;
        a = parent[a];
        parent[b] = r;
    }
    return a;
}

fn join(parent: &mut [usize], rank: &mut [usize], a: usize, b: usize) {
    let mut a_root = root(parent, a);
    let mut b_root = root(parent, b);

    if rank[a_root] < rank[b_root] {
        let tmp = a_root;
        a_root = b_root;
        b_root = tmp;
    }

    parent[b_root] = a_root;
    if rank[a_root] == rank[b_root] {
        rank[a_root] += 1;
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut ints = input
        .split(|c| c == '\n' || c == ' ')
        .map(|i| i.parse::<i32>().unwrap());
    let mut output = String::with_capacity(4096);

    loop {
        const N: usize = 20000;
        const M: usize = 30000;
        let (n, m) = (ints.next().unwrap() as usize, ints.next().unwrap() as usize);
        if n == 0 {
            break;
        }

        let mut edges = [Edge { w: 0, u: 0, v: 0 }; M];
        for i in 0..m {
            let (u, v, w) = (
                ints.next().unwrap() as usize,
                ints.next().unwrap() as usize,
                ints.next().unwrap(),
            );
            edges[i] = Edge { u, v, w };
        }

        edges.sort();

        let mut parent = [0; N];
        for (i, p) in parent.iter_mut().enumerate() {
            *p = i;
        }
        let mut rank = [0; N];

        let mut tw = 0;
        let mut ue = Vec::with_capacity(m);
        for &Edge { u, v, w } in edges.iter() {
            if root(&mut parent, u) == root(&mut parent, v) {
                continue;
            }

            join(&mut parent, &mut rank, u, v);
            tw += w;
            ue.push((u.min(v), u.max(v)));
        }
        let root0 = root(&mut parent, 0);
        let mut ok = true;
        for u in 1..n {
            if root(&mut parent, u) != root0 {
                ok = false;
                break;
            }
        }
        if ok {
            ue.sort();
            writeln!(output, "{}", tw).unwrap();
            for (u, v) in ue {
                writeln!(output, "{} {}", u, v).unwrap();
            }
        } else {
            writeln!(output, "Impossible").unwrap();
        }
    }

    print!("{}", output);
}
