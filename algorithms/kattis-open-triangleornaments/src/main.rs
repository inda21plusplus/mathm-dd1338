use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut ints = input.split(|c| c == ' ' || c == '\n');
    let mut get = || ints.next().unwrap().parse::<i32>().unwrap();

    let n = get();

    type F = f64;
    let l: F = (0..n)
        .map(|_| {
            let (a, b, c) = (get(), get(), get());

            let a_sq = a * a;
            let b_sq = b * b;
            let c_sq = c * c;

            let cos_a = (b_sq + c_sq - a_sq) as F / (2 * b * c) as F;
            let m_sq = b_sq as F + c_sq as F / 4. - (b * c) as F * cos_a;
            let m = m_sq.sqrt();

            let v: F = ((m_sq as F + (c * c) as F / 4. - b_sq as F) / (m * c as F)).acos();

            (c as F) * v.sin()
        })
        .sum();

    println!("{}", l);
}
