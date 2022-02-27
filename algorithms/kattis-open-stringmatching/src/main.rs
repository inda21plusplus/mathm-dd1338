// https://open.kattis.com/problems/stringmatching

use std::{
    fmt::Write,
    io::{stdin, Read},
    iter::{self, successors},
};

// Stolen and modified from u64::mod
fn mod_pow(mut b: u64, mut e: usize, m: u64) -> u64 {
    if e == 0 {
        return 1;
    }
    let mut acc = 1;

    while e > 1 {
        if (e & 1) == 1 {
            acc = (acc * b) % m;
        }
        e /= 2;
        b = (b * b) % m;
    }

    (acc * b) % m
}

const P1B: u64 = 2478183629;
const P1M: u64 = 3900139213;

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut output = String::new();
    for (pat, text) in iter::from_fn(|| {
        let (a, b) = (lines.next()?, lines.next()?);
        Some((a, b))
    }) {
        if pat.len() > text.len() {
            writeln!(output).ok();
            continue;
        }
        let pat = pat.as_bytes();
        let text = text.as_bytes();
        let pat_hash = pat
            .iter()
            .rev()
            .zip(successors(Some(1), |a| Some((a * P1B) % P1M)))
            .map(|(&c, b)| c as u64 * b)
            .fold(0, |a, c| (a + c) % P1M);
        let mut h = text[..pat.len()]
            .iter()
            .rev()
            .zip(successors(Some(1), |a| Some((a * P1B) % P1M)))
            .map(|(&c, b)| c as u64 * b)
            .fold(0, |a, c| (a + c) % P1M);
        for (first, last) in (0..).zip(pat.len()..text.len()) {
            if h == pat_hash {
                write!(output, "{} ", first).ok();
            }
            let to_sub = (text[first] as u64 * mod_pow(P1B, pat.len() - 1, P1M)) % P1M;
            if to_sub > h {
                h += P1M;
                h -= to_sub;
                h %= P1M;
            } else {
                h -= to_sub;
            }
            h = (h as u64 * P1B) % P1M;
            h = (h as u64 + text[last] as u64) % P1M;
        }
        if h == pat_hash {
            write!(output, "{} ", text.len() - pat.len()).ok();
        }
        writeln!(output).ok();
    }
    print!("{}", output);
}
