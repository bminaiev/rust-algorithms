use std::io;
use std::io::Write;

/**************************************************

   START OF TEMPLATE CODE

*************************************************/
#[allow(unused_macros)]
macro_rules! dbg {
    ($first_val:expr, $($val:expr),+ $(,)?) => {
        eprint!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($first_val), &$first_val);
        ($(eprint!(", {} = {:?}", stringify!($val), &$val)),+,);
        eprintln!();
    };
    ($first_val:expr) => {
        eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($first_val), &$first_val);
    };
}

enum InputSource {
    Stdin,
    FromFile(Vec<String>),
}

struct Scanner {
    buffer: Vec<String>,
    input_source: InputSource,
}

impl Scanner {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            buffer: vec![],
            input_source: InputSource::Stdin,
        }
    }

    #[allow(dead_code)]
    fn new_file(filename: &str) -> Self {
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
        lines.reverse();
        Self {
            buffer: vec![],
            input_source: InputSource::FromFile(lines),
        }
    }

    #[allow(dead_code)]
    fn i64(&mut self) -> i64 {
        self.next::<i64>()
    }

    #[allow(dead_code)]
    fn i32(&mut self) -> i32 {
        self.next::<i32>()
    }

    #[allow(dead_code)]
    fn usize(&mut self) -> usize {
        self.next::<usize>()
    }

    #[allow(dead_code)]
    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

    fn parse_next_line(&mut self) -> bool {
        let mut input = String::new();
        match &mut self.input_source {
            InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            InputSource::FromFile(lines) => match lines.pop() {
                Some(line) => input = line,
                None => return false,
            },
        }

        self.buffer = input.split_whitespace().rev().map(String::from).collect();
        return true;
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }

            self.parse_next_line();
        }
    }

    #[allow(dead_code)]
    fn has_more_elements(&mut self) -> bool {
        loop {
            if !self.buffer.is_empty() {
                return true;
            }
            if !self.parse_next_line() {
                return false;
            }
        }
    }

    #[allow(dead_code)]
    fn string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}

/**************************************************

   END OF TEMPLATE CODE

*************************************************/

fn mod_exp(mut x: u128, mut d: u128, n: u128) -> u128 {
    let mut ret = 1;
    while d != 0 {
        if d % 2 == 1 {
            ret *= x;
            if ret >= n {
                ret %= n;
            }
        }
        d /= 2;
        x *= x;
        if x >= n {
            x %= n;
        }
    }
    ret
}

/// Test if `n` is prime, using the deterministic version of the
/// Miller-Rabin test.
///
/// Doing a lot of primality tests with numbers strictly below some
/// upper bound will be faster using the `is_prime` method of a
/// `Primes` instance.
pub fn is_prime_miller_rabin(n: u64) -> bool {
    const HINT: &'static [u64] = &[2];

    // we have a strict upper bound, so we can just use the witness
    // table of Pomerance, Selfridge & Wagstaff and Jeaschke to be as
    // efficient as possible, without having to fall back to
    // randomness.
    const WITNESSES: &'static [(u64, &'static [u64])] = &[
        (2_046, HINT),
        (1_373_652, &[2, 3]),
        (9_080_190, &[31, 73]),
        (25_326_000, &[2, 3, 5]),
        (4_759_123_140, &[2, 7, 61]),
        (1_112_004_669_632, &[2, 13, 23, 1662803]),
        (2_152_302_898_746, &[2, 3, 5, 7, 11]),
        (3_474_749_660_382, &[2, 3, 5, 7, 11, 13]),
        (341_550_071_728_320, &[2, 3, 5, 7, 11, 13, 17]),
        (0xFFFF_FFFF_FFFF_FFFF, &[2, 3, 5, 7, 11, 13, 17, 19, 23]),
    ];

    if n % 2 == 0 {
        return n == 2;
    }
    if n == 1 {
        return false;
    }

    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1
    }

    let witnesses = WITNESSES
        .iter()
        .find(|&&(hi, _)| hi >= n)
        .map(|&(_, wtnss)| wtnss)
        .unwrap();
    'next_witness: for &a in witnesses.iter() {
        let mut power = mod_exp(a as u128, d as u128, n as u128);
        if power == 1 {
            continue 'next_witness;
        }

        for _r in 0..s {
            if power == (n - 1) as u128 {
                continue 'next_witness;
            }
            power *= power;
            if power >= n as u128 {
                power %= n as u128;
            }
        }
        return false;
    }

    true
}

fn solve(left: u64, right: u64) -> u64 {
    let mut max_pow_of_two = 0;
    while (1u64 << max_pow_of_two) <= right {
        max_pow_of_two += 1;
    }
    let mut basis = vec![0; max_pow_of_two];
    let mut seen_basis = 0;
    for p in (left..=right).rev() {
        dbg!(seen_basis, max_pow_of_two);
        if seen_basis == max_pow_of_two {
            break;
        }
        if !is_prime_miller_rabin(p) {
            continue;
        }
        dbg!("prime!!", p);
        let mut changed_p = p;
        for iter in (0..max_pow_of_two).rev() {
            if (1u64 << iter) & changed_p == 0 {
                continue;
            }
            if basis[iter] == 0 {
                basis[iter] = changed_p;
                seen_basis += 1;
                break;
            }
            changed_p ^= basis[iter];
        }
    }
    1u64 << seen_basis
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let left = sc.next::<u64>();
        let right = sc.next::<u64>();
        writeln!(out, "{}", solve(left, right)).unwrap();
    }
}
