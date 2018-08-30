use std::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::BTreeMap;

const R_MIN: u32 = 1;
const R_MAX: u32 = 16383;
const N_MIN: u32 = 192;
const N_MAX: u32 = 524287;
const K_MIN: u32 = 25;
const K_MAX: u32 = 50;
const F_REF: u32 = 40_000_000;

struct Freq(f64);
impl PartialEq for Freq {
    fn eq(&self, other: &Freq) -> bool {
        self.0 == other.0
    }
}
impl Eq for Freq {}
impl PartialOrd for Freq {
    fn partial_cmp(&self, other: &Freq) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Freq {
    fn cmp(&self, other: &Freq) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn strip_n(n: u32) -> u32 {
    min(N_MAX, max(N_MIN, n))
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn are_coprime(a: u32, b: u32) -> bool {
    gcd(a,b) == 1
}

fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn count() -> u32 {
    let mut cnt = 0;
    for r in R_MIN..=R_MAX {
        let n0 = strip_n(r * K_MIN);
        let n1 = strip_n(r * K_MAX);
        //cnt += n1 - n0 + 1;
        for n in n0..=n1 {
            if are_coprime(n, r) {
                cnt += 1;
            }
        } // 1511106787
    }
    cnt
}

fn best_r_n(freq: f64) -> (u32, u32) {
    assert!(freq >= 1e9 && freq <= 2e9);
    let k = freq / { F_REF as f64 };
    let mut best_r: u32 = 0;
    let mut best_n: u32 = 0;
    let mut best_delta = 1000.0;
    for r in R_MIN..=R_MAX {
        let r_f64 = r as f64;
        let n = (r_f64 * k).round();
        let delta = abs(k - n / r_f64);
        if delta < best_delta {
            best_delta = delta;
            best_r = r;
            best_n = n as u32;
        }
    }
    (best_r, best_n)
}

fn solutions_near(freq: f64) {
    assert!(freq >= 1e9 && freq <= 2e9);
    let mut dfreq = 1.0;
    let mut solutions = BTreeMap::new();
    loop {
        let freq0 = freq - dfreq;
        let freq1 = freq + dfreq;
        let k0 = freq0 / { F_REF as f64 };
        let k1 = freq1 / { F_REF as f64 };
        for r in R_MIN..=R_MAX {
            let r_f64 = r as f64;
            let n0 = max(N_MIN, (r_f64 * k0).ceil() as u32);
            let n1 = min(N_MAX, (r_f64 * k1).floor() as u32);
            for n in n0..=n1 {
                let r_freq = { F_REF as f64 } * { n as f64 } / { r as f64 };
                if !solutions.contains_key(&Freq(r_freq)) {
                    solutions.insert(Freq(r_freq), (r, n));
                }
            }
        }
        if solutions.len() < 10 {
            dfreq *= 2.0;
            solutions.clear();
        } else {
            break;
        }
    }
    println!("\ncnt: {}", solutions.len());

    for (Freq(r_freq), (r, n)) in solutions {
        let delta = r_freq - freq;
        println!("{}: (delta={})  r={}  n={}", r_freq, delta, r, n);
    }
}

fn print_best_r_n(freq: f64) {
    let (r, n) = best_r_n(freq);
    let r_freq = { F_REF as f64 } * { n as f64 } / { r as f64 };
    let delta = r_freq - freq;
    println!(
        "{} -> r={}  n={} -> {} (delta={})",
        freq, r, n, r_freq, delta
    );
}

fn main() {
    //println!("count: {}", count());

    print_best_r_n(1.21477e9);
    print_best_r_n(1.214771e9);
    print_best_r_n(1.2147712e9);
    print_best_r_n(1.54215e9);

    solutions_near(1.54215e9);
    //solutions_near(1e9);
    //solutions_near(1.5e9);
    solutions_near(1.654321e9);
    println!("");
    print_best_r_n(1e9);
    print_best_r_n(2e9);
    print_best_r_n(1_000_001_000.0);
    print_best_r_n(1_000_002_000.0);
    print_best_r_n(1_000_003_000.0);
    print_best_r_n(1_000_004_000.0);
    print_best_r_n(1_999_999_000.0);
    print_best_r_n(1_999_998_000.0);
    print_best_r_n(1_999_997_000.0);
    print_best_r_n(1_999_996_000.0);
    //solutions_near(1e9);
}
