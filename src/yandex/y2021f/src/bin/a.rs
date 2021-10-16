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
        Self { buffer: vec![], input_source: InputSource::Stdin }
    }

    #[allow(dead_code)]
    fn new_file(filename: &str) -> Self {
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
        lines.reverse();
        Self { buffer: vec![], input_source: InputSource::FromFile(lines) }
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
            | InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            | InputSource::FromFile(lines) => {
                match lines.pop() {
                    Some(line) => input = line,
                    None => return false,
                }
            }
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

mod modulo {
    const MODULO: i32 = 1_000_000_123;

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Mod(pub(crate) i32);

    impl Mod {
        #[allow(unused)]
        pub const ZERO: Self = Self(0);

        #[allow(unused)]
        pub const ONE: Self = Self(1);

        fn rev_rec(a: i32, m: i32) -> i32 {
            if a == 1 {
                return a;
            }
            return ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64) as i32;
        }

        #[allow(dead_code)]
        fn inv(self) -> Mod {
            Mod(Self::rev_rec(self.0, MODULO))
        }

        #[allow(dead_code)]
        pub fn new(mut x: i32) -> Self {
            if x < 0 {
                x += MODULO;
            } else if x >= MODULO {
                x -= MODULO;
            }
            assert!(0 <= x && x < MODULO);
            Self(x)
        }
    }

    impl std::fmt::Display for Mod {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::fmt::Debug for Mod {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            const MAX: usize = 100;
            if self.0 <= MAX as i32 {
                write!(f, "{}", self.0)
            } else if self.0 >= MODULO - MAX as i32 {
                write!(f, "-{}", MODULO - self.0)
            } else {
                for denum in 1..MAX {
                    for num in 1..MAX {
                        if Mod(num as i32) / Mod(denum as i32) == *self {
                            return write!(f, "{}/{}", num, denum);
                        }
                    }
                }
                write!(f, "(?? {} ??)", self.0)
            }
        }
    }

    impl std::ops::Add for Mod {
        type Output = Mod;

        fn add(self, rhs: Self) -> Self::Output {
            let res = self.0 + rhs.0;
            if res >= MODULO {
                Mod(res - MODULO)
            } else {
                Mod(res)
            }
        }
    }

    impl std::ops::AddAssign for Mod {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
            if self.0 >= MODULO {
                self.0 -= MODULO;
            }
        }
    }

    impl std::ops::Sub for Mod {
        type Output = Mod;

        fn sub(self, rhs: Self) -> Self::Output {
            let res = self.0 - rhs.0;
            if res < 0 {
                Mod(res + MODULO)
            } else {
                Mod(res)
            }
        }
    }

    impl std::ops::SubAssign for Mod {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
            if self.0 < 0 {
                self.0 += MODULO;
            }
        }
    }

    impl std::ops::Mul for Mod {
        type Output = Mod;

        fn mul(self, rhs: Self) -> Self::Output {
            let res = (self.0 as i64) * (rhs.0 as i64) % (MODULO as i64);
            Mod(res as i32)
        }
    }

    impl std::ops::MulAssign for Mod {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = ((self.0 as i64) * (rhs.0 as i64) % (MODULO as i64)) as i32;
        }
    }

    impl std::ops::Div for Mod {
        type Output = Mod;

        fn div(self, rhs: Self) -> Self::Output {
            let rhs_inv = rhs.inv();
            self * rhs_inv
        }
    }

    impl std::ops::DivAssign for Mod {
        fn div_assign(&mut self, rhs: Self) {
            *self *= rhs.inv();
        }
    }
}

use modulo::*;
use std::collections::{HashSet, HashMap};
use std::cmp::min;
use std::hash::{Hasher, BuildHasherDefault, BuildHasher};

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/


const MUL: Mod = Mod { 0: 1234577 };
const MAX_LEN: usize = 21;

struct State {
    expect_len : usize,
    existing_hashes : Vec<HashSet<Mod, BuildHasherDefault<IntHasher>>>,
    added : usize,
    k : i32,
}

fn dfs(a: &mut Vec<i32>, state : &State) -> usize {
    return if a.len() == state.expect_len {
        let mut cur_hash = Mod::ZERO;
        for l1 in 0..min(a.len(), MAX_LEN) {
            cur_hash = cur_hash * MUL + Mod::new(a[a.len() - 1 - l1]);
            if l1 + 1 >= state.added && !state.existing_hashes[l1].contains(&cur_hash) {
                return l1 + 1;
            }
        }
        usize::MAX
    } else {
        let mut global_res = usize::MAX;
        for v1 in (1..=state.k).rev() {
            a.push(v1);
            let res = dfs(a, state);
            if res <= state.added {
                a.pop();
                return res;
            }
            global_res = min(global_res, res);
            a.pop();
        }
        global_res
    };
}

#[derive(Default)]
#[allow(dead_code)]
struct IntHasher {
    value: u64,
}


#[allow(dead_code)]
impl Hasher for IntHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes.iter() {
            self.value = self.value.wrapping_mul(934_475_254_358_498_081);
            self.value = self.value.wrapping_add(b as u64);
        }
    }
}

#[allow(dead_code)]
type IntBuildHasher = BuildHasherDefault<IntHasher>;


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();


    let n = sc.usize();
    let k = sc.i32();
    let mut a = sc.vec::<i32>(n);

    if k == 1 {
        writeln!(out, "{} {}", n, 0).unwrap();
        return;
    }


    let mut smallest_len = MAX_LEN;


    let mut min_add = 0;
    let mut cur_add = 0;


    let existing_hashes = vec![HashSet::with_capacity_and_hasher(n / 4, IntBuildHasher::default()); MAX_LEN];

    let mut state = State {
        expect_len : 0,
        existing_hashes ,
        added : 0,
        k ,
    };

    for start in 0..n {
        if start == n - 1 {
            let alen = a.len();
            state.expect_len = alen;
            state.added = 0;
            let res = dfs(&mut a, &state);
            if res < smallest_len {
                smallest_len = res;
                min_add = cur_add;
            }
        }
        let mut cur_hash = Mod::ZERO;
        for len in 0..smallest_len {
            if len > start {
                break;
            }
            cur_hash = cur_hash * MUL + Mod::new(a[start - len]);
            state.existing_hashes[len].insert(cur_hash);
        }
    }

    cur_add += 1;
    while cur_add < smallest_len {
        let alen = a.len();
        state.expect_len = alen + cur_add;
        state.added = cur_add;
        let res = dfs(&mut a, &state);
        if res < smallest_len {
            smallest_len = res;
            min_add = cur_add;
        }

        cur_add += 1;
    }
    writeln!(out, "{} {}", smallest_len, min_add).unwrap();
}
