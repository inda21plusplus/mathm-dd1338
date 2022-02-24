// https://open.kattis.com/problems/hashing

use std::fmt::Write;
use std::io::{stdin, Read};

use algorithms::SegmentTree;

// Stolen and modified from u64::mod
fn mod_pow(mut b: u64, mut e: u32, m: u64) -> u64 {
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

#[derive(Clone, Copy)]
struct Hash {
    val1: u32,
    val2: u32,
    len: u32,
}

const P1B: u64 = 2478183629;
const P1M: u64 = 3900139213;

const P2B: u64 = 2099346611;
const P2M: u64 = 3071506577;

impl Hash {
    fn one(b: u8) -> Self {
        Self {
            val1: b as _,
            val2: b as _,
            len: 1,
        }
    }
    fn unit() -> Self {
        Self {
            val1: 0,
            val2: 0,
            len: 0,
        }
    }
    fn hash(l: Self, r: Self) -> Self {
        let val1 =
            (((l.val1 as u64 * mod_pow(P1B, r.len, P1M)) % P1M + r.val1 as u64) % P1M) as u32;
        let val2 =
            (((l.val2 as u64 * mod_pow(P2B, r.len, P2M)) % P2M + r.val2 as u64) % P2M) as u32;
        Self {
            val1,
            val2,
            len: l.len + r.len,
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    let mut split = input.split(|c| c == ' ' || c == '\n');
    let s = split.next().unwrap();

    let st = SegmentTree::from(s.bytes().map(Hash::one), Hash::hash, Hash::unit);

    let mut get = || split.next().unwrap().parse::<usize>().unwrap();
    let q = get();

    let mut output = String::with_capacity(q * 21);

    for _ in 0..q {
        let (l, r) = (get(), get());

        let Hash { val1, val2, .. } = st.query(l..r);
        let val = val1 as u64 * 2u64.pow(32) + val2 as u64;
        writeln!(output, "{}", val).unwrap();
    }

    print!("{}", output);
}
