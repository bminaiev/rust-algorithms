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

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

mod modulo {
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mod(pub(crate) i32, i32);

    impl Mod {
        fn rev_rec(a: i32, m: i32) -> i32 {
            if a == 1 {
                return a;
            }
            return ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64) as i32;
        }

        #[allow(dead_code)]
        fn inv(self) -> Mod {
            Mod(Self::rev_rec(self.0, self.1), self.1)
        }

        pub(crate) fn pow(self, p: i64) -> Self {
            if p == 0 {
                Mod::new(1, self.1)
            } else if p == 1 {
                self
            } else {
                let p2 = self.pow(p / 2);
                let mut res = p2 * p2;
                if p % 2 == 1 {
                    res = res * self;
                }
                res
            }
        }

        #[allow(dead_code)]
        pub fn new(mut x: i32, modulo: i32) -> Self {
            if x < 0 {
                x += modulo;
            } else if x >= modulo {
                x -= modulo;
            }
            assert!(0 <= x && x < modulo);
            Self(x, modulo)
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
            } else if self.0 >= self.1 - MAX as i32 {
                write!(f, "-{}", self.1 - self.0)
            } else {
                for denum in 1..MAX {
                    for num in 1..MAX {
                        if Mod(num as i32, self.1) / Mod(denum as i32, self.1) == *self {
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
            if res >= self.1 {
                Mod(res - self.1, self.1)
            } else {
                Mod(res, self.1)
            }
        }
    }

    impl std::ops::AddAssign for Mod {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
            if self.0 >= self.1 {
                self.0 -= self.1;
            }
        }
    }

    impl std::ops::Sub for Mod {
        type Output = Mod;

        fn sub(self, rhs: Self) -> Self::Output {
            let res = self.0 - rhs.0;
            if res < 0 {
                Mod(res + self.1, self.1)
            } else {
                Mod(res, self.1)
            }
        }
    }

    impl std::ops::SubAssign for Mod {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
            if self.0 < 0 {
                self.0 += self.1;
            }
        }
    }

    impl std::ops::Mul for Mod {
        type Output = Mod;

        fn mul(self, rhs: Self) -> Self::Output {
            let res = (self.0 as i64) * (rhs.0 as i64) % (self.1 as i64);
            Mod(res as i32, self.1)
        }
    }

    impl std::ops::MulAssign for Mod {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = ((self.0 as i64) * (rhs.0 as i64) % (self.1 as i64)) as i32;
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

#[derive(Copy, Clone, Debug)]
struct Rat {
    x: i32,
    y: i32,
}

const MAIN_MODULO: i32 = 998244353;

fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let mut x = sc.i32();
        let mut y = sc.i32();
        // divide by gcd
        let g = gcd(x, y);
        x /= g;
        y /= g;
        a.push(Rat { x, y });
    }
    let max_y = (a.iter().map(|elem| elem.y).max().unwrap() + 1) as usize;
    let mut by_y = vec![vec![]; max_y];
    for rat in a.iter() {
        // dbg!(x, y);
        by_y[rat.y as usize].push(rat);
    }
    let mut res = Mod::new(1, MAIN_MODULO);
    for rat in a.iter() {
        res *= Mod::new(rat.y, MAIN_MODULO).pow((n - 1) as i64);
    }
    let mut is_prime = vec![true; max_y];
    for x in 2..max_y {
        for y in (x * 2..max_y).step_by(x) {
            is_prime[y] = false;
        }
    }
    let mut all_cnt = vec![0; max_y];
    let mut changed = vec![];
    for p in 2..max_y {
        if !is_prime[p] {
            continue;
        }
        let mut subtract_p = 0;
        let mut p_in_power = 1;
        for power in 1.. {
            if max_y / p < p_in_power {
                break;
            }
            p_in_power *= p;
            if p_in_power >= max_y {
                break;
            }
            {
                let mut cur_cnt = 0;

                let mut for_vanya = vec![];
                for z in (p_in_power..max_y).step_by(p_in_power) {
                    for rat in by_y[z].iter() {
                        // first
                        subtract_p += cur_cnt;
                        cur_cnt += 1;
                        if (rat.y / p_in_power as i32) % (p as i32) != 0 {
                            let mut my_y = rat.y;
                            while my_y % p as i32 == 0 {
                                my_y /= p as i32;
                            }
                            let frac = Mod::new(rat.x % (p_in_power as i32), p_in_power as i32);
                            let frac = frac / Mod::new(my_y % (p_in_power as i32), p_in_power as i32);
                            let frac = frac.0 as usize;
                            for_vanya.push(frac);
                        }
                    }
                }
                for p2 in 1..=power {
                    for &frac in for_vanya.iter() {
                        let frac = frac % p.pow(p2);
                        // second
                        subtract_p += all_cnt[frac] as i64;
                        all_cnt[frac] += 1;
                        changed.push(frac);
                    }


                    for c in changed.iter() {
                        all_cnt[*c] = 0;
                    }
                    changed.clear();
                }
            }
        }

        res /= Mod::new(p as i32, MAIN_MODULO).pow(subtract_p);
    }
    writeln!(out, "{}", res).unwrap();
}
