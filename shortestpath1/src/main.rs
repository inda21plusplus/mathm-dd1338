use std::fmt::Write;
use std::u32;
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, Read},
};

fn main() {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut lines = buf.lines();
    let mut out = String::with_capacity(1024);

    loop {
        let mut case = lines
            .next()
            .unwrap()
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap());
        let (n, m, q, s) = (
            case.next().unwrap(),
            case.next().unwrap(),
            case.next().unwrap(),
            case.next().unwrap(),
        );

        if n == 0 {
            break;
        }

        let mut adjlist = (0..n).map(|_| vec![]).collect::<Vec<Vec<(usize, u32)>>>();

        for mut edge in (&mut lines).take(m).map(|s| s.split(' ')) {
            let (u, v, w) = (
                edge.next().unwrap().parse::<usize>().unwrap(),
                edge.next().unwrap().parse::<usize>().unwrap(),
                edge.next().unwrap().parse::<u32>().unwrap(),
            );
            adjlist[u].push((v, w));
        }

        let mut dist = [u32::MAX; 10000];
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), s));
        dist[s] = 0;

        while let Some((d, node)) = heap.pop().map(|r| (r.0 .0, r.1)) {
            for &(neighbor, w) in &adjlist[node] {
                if d + w < dist[neighbor] {
                    dist[neighbor] = d + w;
                    heap.push((Reverse(d + w), neighbor));
                }
            }
        }

        for q in (&mut lines).take(q).map(|i| i.parse::<usize>().unwrap()) {
            if dist[q] == u32::MAX {
                writeln!(&mut out, "Impossible").unwrap();
            } else {
                writeln!(&mut out, "{}", dist[q]).unwrap();
            }
        }
    }

    print!("{}", out);
}
