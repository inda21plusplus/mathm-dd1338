// https://open.kattis.com/problems/blowingcandles

use std::io::{stdin, Read};

use algorithms::{
    geometry::{Polygon, PolygonMethods, Vector},
    v,
};

fn dist(a: Vector<i32, 2>, b: Vector<i32, 2>, p: Vector<i32, 2>) -> f64 {
    let n = (b - a).cross(p - a).abs() as f64;
    let d = ((b - a).len_sq() as f64).sqrt();
    n / d
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut split = input.split(|c| c == ' ' || c == '\n');
    let mut get = || split.next().unwrap().parse::<i32>().unwrap();
    let (n, r) = (get(), get());

    let ps = Polygon::new((0..n).map(|_| v![get(), get()]).collect());

    let hull = ps.convex_hull();

    let mut j = 2isize;
    let mut min_w = r as f64 * 2.;
    for i in 0..hull.len() as isize {
        while dist(hull[i], hull[i + 1], hull[j + 1]) > dist(hull[i], hull[i + 1], hull[j]) {
            j = (j + 1) % hull.len() as isize;
        }
        let w = dist(hull[i], hull[i + 1], hull[j]);
        min_w = min_w.min(w);
    }

    println!("{}", min_w);
}
