use std::io;
use std::io::Write;
use std::collections::HashMap;

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
    const MODULO: i32 = 998244353;

    #[derive(Copy, Clone, Eq, PartialEq, Default)]
    pub struct Mod(i32);

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

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/


#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct State {
    a: usize,
    b: usize,
    c: usize,
    ab: usize,
    bc: usize,
    ca: usize,
    more_starts: usize,
    expect: Expect,
}

impl State {
    fn can_finish(&self) -> bool {
        if self.expect.a < self.b + self.c + self.bc {
            return false;
        }
        if self.expect.b < self.a + self.c + self.ca {
            return false;
        }
        if self.expect.c < self.b + self.a + self.ab {
            return false;
        }
        let more_a = self.expect.a - (self.b + self.c + self.bc);
        let more_b = self.expect.b - (self.a + self.c + self.ca);
        let more_c = self.expect.c - (self.b + self.a + self.ab);
        assert_eq!(more_a, more_b);
        assert_eq!(more_b, more_c);

        true
    }

    fn go(&self, c: u8) -> Option<Self> {
        // let need_at_least = (self.a + self.b + self.c) * 2 + self.ab + self.bc + self.ca;
        // let prefer_closing = need_at_least >= more;
        if let Some(next_expect) = self.expect.go(c) {
            let mut res = self.clone();
            res.expect = next_expect;
            let prefer_closing = res.more_starts == 0;
            if c == b'A' {
                if self.c > 0 {
                    res.c -= 1;
                    res.ca += 1;
                } else if self.bc > 0 && prefer_closing {
                    res.bc -= 1;
                } else {
                    res.a += 1;
                    if res.more_starts == 0 {
                        return None;
                    }
                    res.more_starts -= 1;
                }
            } else if c == b'C' {
                if self.b > 0 {
                    res.b -= 1;
                    res.bc += 1;
                } else if self.ab > 0 && prefer_closing {
                    res.ab -= 1;
                } else {
                    res.c += 1;
                    if res.more_starts == 0 {
                        return None;
                    }
                    res.more_starts -= 1;
                }
            } else {
                assert_eq!(c, b'B');
                if self.a > 0 {
                    res.a -= 1;
                    res.ab += 1;
                } else if self.ca > 0 && prefer_closing {
                    res.ca -= 1;
                } else {
                    res.b += 1;
                    if res.more_starts == 0 {
                        return None;
                    }
                    res.more_starts -= 1;
                }
            }
            Some(res)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Expect {
    a: usize,
    b: usize,
    c: usize,
}

impl Expect {
    fn go(&self, c: u8) -> Option<Self> {
        let mut res = self.clone();
        if c == b'A' {
            if res.a == 0 {
                return None;
            }
            res.a -= 1;
        } else if c == b'B' {
            if res.b == 0 {
                return None;
            }
            res.b -= 1;
        } else {
            assert_eq!(c, b'C');
            if res.c == 0 {
                return None;
            }
            res.c -= 1;
        }
        return Some(res);
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();


    let n = sc.usize();
    let s = sc.string();
    assert_eq!(n * 3, s.len());

    let start = State { a: 0, b: 0, c: 0, ab: 0, bc: 0, ca: 0, expect: Expect { a: n, b: n, c: n }, more_starts : n };

    let mut cnt = HashMap::new();
    *cnt.entry(start).or_default() = Mod::ONE;


    let all_c = vec![b'A', b'B', b'C'];

    for (pos, &c) in s.iter().enumerate() {
        let mut new_cnt = HashMap::new();

        for (prev_state, &old_cnt) in cnt.iter() {
            for &next_c in all_c.iter() {
                if next_c == c || c == b'?' {
                    if let Some(next_state) = prev_state.go(next_c) {
                        *new_cnt.entry(next_state).or_default() += old_cnt;
                    }
                }
            }
        }

        cnt = new_cnt;
    }

    let end = State { a: 0, b: 0, c: 0, ab: 0, bc: 0, ca: 0, expect: Expect { a: 0, b: 0, c: 0 }, more_starts : 0 };

    let res = cnt.entry(end).or_default();
    writeln!(out, "{}", res).unwrap();
}
