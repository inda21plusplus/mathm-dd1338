use std::io::{stdin, Read};

unsafe fn root(parent: &mut [usize], mut node: usize) -> usize {
    let mut r = node;
    while r != *parent.get_unchecked(r) {
        r = *parent.get_unchecked(r);
    }
    while node != r {
        let tmp = node;
        node = *parent.get_unchecked(node);
        *parent.get_unchecked_mut(tmp) = r;
    }
    r
}

unsafe fn join(parent: &mut [usize], rank: &mut [usize], a: usize, b: usize) {
    let mut a = root(parent, a);
    let mut b = root(parent, b);

    if a == b { return; }

    if *rank.get_unchecked(a) < *rank.get_unchecked(b) {
        let tmp = a;
        a = b;
        b = tmp;
    }

    *parent.get_unchecked_mut(b) = a;
    if *rank.get_unchecked(a) == *rank.get_unchecked(b) {
        *rank.get_unchecked_mut(a) += 1;
    }
}

fn main() { unsafe {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).ok();
    let mut lines = input.lines();
    let mut nq = lines.next().unwrap_unchecked().trim().split(' ').map(|l| l.trim().parse::<usize>().unwrap_unchecked());
    let (n, q) = (nq.next().unwrap_unchecked(), nq.next().unwrap_unchecked());
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0usize; n];
    let mut output = String::with_capacity(q * 4);
    for _ in 0..q {
        let line = lines.next().unwrap_unchecked().trim();
        let op = &line[0..1];
        let mut ab = line[2..].split(' ').map(|l| l.trim().parse::<usize>().unwrap_unchecked());
        let (a, b) = (ab.next().unwrap_unchecked(), ab.next().unwrap_unchecked());
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
} }