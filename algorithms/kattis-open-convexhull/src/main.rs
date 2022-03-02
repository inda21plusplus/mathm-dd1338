// https://open.kattis.com/problems/convexhull

use std::fmt::Write;
use std::io::{stdin, Read};

use algorithms::{
    geometry::{Polygon, PolygonMethods},
    v,
};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut ints = input.split(|c| c == '\n' || c == ' ');
    let mut get = || ints.next().unwrap().parse::<i32>().unwrap();

    let mut output = String::new();
    loop {
        let n = get();
        if n == 0 {
            break;
        }
        let polygon = Polygon::<i32, false>::new((0..n).map(|_| v!(get(), get())).collect());
        let convex_hull = polygon.convex_hull();
        writeln!(output, "{}", convex_hull.len()).unwrap();
        for p in convex_hull.points().iter() {
            writeln!(output, "{} {}", p[0], p[1]).unwrap();
        }
    }
    print!("{}", output);
}
