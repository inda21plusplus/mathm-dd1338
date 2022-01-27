use std::fmt::Write;
use std::i64;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut ints = input
        .split(|c| c == ' ' || c == '\n')
        .map(|s| s.trim().parse::<i64>().unwrap());
    let mut output = String::with_capacity(11000);

    loop {
        const N: usize = 150;
        let (n, m, q) = (
            ints.next().unwrap() as usize,
            ints.next().unwrap(),
            ints.next().unwrap(),
        );
        if n == 0 {
            break;
        }

        let mut dist = [[i64::MAX; N]; N];

        for _ in 0..m {
            let (u, v, w) = (
                ints.next().unwrap() as usize,
                ints.next().unwrap() as usize,
                ints.next().unwrap(),
            );

            dist[u][v] = dist[u][v].min(w);
        }

        for v in 0..n {
            dist[v][v] = 0;
        }

        for k in 0..n {
            for a in 0..n {
                for b in 0..n {
                    if dist[a][k] < i64::MAX && dist[k][b] < i64::MAX {
                        dist[a][b] = dist[a][b].min(dist[a][k] + dist[k][b]);
                    }
                }
            }
        }

        for a in 0..n {
            for b in 0..n {
                for k in 0..n {
                    if dist[a][k] < i64::MAX && dist[k][k] < 0 && dist[k][b] < i64::MAX {
                        dist[a][b] = i64::MIN;
                    }
                }
            }
        }

        for _ in 0..q {
            let (u, v) = (ints.next().unwrap() as usize, ints.next().unwrap() as usize);
            let d = dist[u][v];
            if d == i64::MAX {
                write!(&mut output, "Impossible\n").unwrap();
            } else if d == i64::MIN {
                write!(&mut output, "-Infinity\n").unwrap();
            } else {
                write!(&mut output, "{}\n", dist[u][v]).unwrap();
            }
        }
    }

    print!("{}", output);
}
