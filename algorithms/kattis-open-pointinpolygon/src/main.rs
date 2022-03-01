// https://kth.kattis.com/problems/pointinpolygon

use std::fmt::Write;
use std::io::{stdin, Read};

use algorithms::geometry::{polygon::Contains, Polygon, Vector};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut ints = input
        .split(|c| c == ' ' || c == '\n')
        .map(|x| x.parse::<i16>().unwrap());
    let mut get = || ints.next().unwrap();

    let mut output = String::new();
    loop {
        let n = get();
        if n == 0 {
            break;
        }
        let polygon = Polygon::new(
            (0..n)
                .map(|_| Vector::<i16, 2>::new(get(), get()))
                .collect(),
        );
        let m = get();
        for _ in 0..m {
            let point = Vector::<i16, 2>::new(get(), get());
            writeln!(
                output,
                "{}",
                match polygon.contains(point) {
                    Contains::Inside => "in",
                    Contains::Outside => "out",
                    Contains::OnBorder => "on",
                }
            )
            .unwrap();
        }
    }
    print!("{}", output);
}
