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

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
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

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    #[allow(dead_code)]
    fn string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}

mod modulo {
    const MODULO: i32 = 998_244_353;

    #[derive(Copy, Clone, Eq, PartialEq)]
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

use modulo::Mod;
use std::cmp::min;

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

fn join(need1: &Vec<Mod>, has1: &Vec<Mod>, need2: &Vec<Mod>, has2: &Vec<Mod>, r: usize) -> (Vec<Mod>, Vec<Mod>) {
    let mut res_need_bad = vec![Mod::ZERO; 0];
    let mut res_has_bad = vec![Mod::ZERO; 0];
    for i in 0..need1.len() {
        for j in 0..need2.len() {
            let i = r + 1 - i;
            let j = r + 1 - j;
            let xx = r + 1 - min(i, j);
            while res_need_bad.len() <= xx {
                res_need_bad.push(Mod::ZERO);
            }
            res_need_bad[xx] += need1[xx] * need2[r + 1 - j];
        }
    }
    for i in 0..has1.len() {
        for j in 0..has2.len() {
            let tt = min(i, j);
            while res_has_bad.len() <= tt {
                res_has_bad.push(Mod::ZERO);
            }
            res_has_bad[min(i, j)] += has1[i] * has2[j];
        }
    }
    for i in 0..need1.len() {
        let i = r + 1 - i;
        for j in 0..has2.len() {
            if i < j {
                let xx = r + 1 - i;
                while res_need_bad.len() <= xx {
                    res_need_bad.push(Mod::ZERO);
                }
                res_need_bad[xx] += need1[r + 1 - i] * has2[j];
            } else {
                while res_has_bad.len() <= j {
                    res_has_bad.push(Mod::ZERO);
                }
                res_has_bad[j] += need1[r + 1 - i] * has2[j];
            }
        }
    }
    for i in 0..need2.len() {
        let i = r + 1 - i;
        for j in 0..has1.len() {
            if i < j {
                let xx = r + 1 - i;
                while (res_need_bad.len() <= xx) {
                    res_need_bad.push(Mod::ZERO);
                }
                res_need_bad[xx] += need2[i] * has1[j];
            } else {
                while res_has_bad.len() <= j {
                    res_has_bad.push(Mod::ZERO);
                }
                res_has_bad[j] += need2[r + 1 - i] * has1[j];
            }
        }
    }
    (res_need_bad, res_has_bad)
}

// return {need_bad_on_dist <= X, has_bad_on_dist X}
fn dfs(g: &[Vec<usize>], r: usize, v: usize, p: usize) -> (Vec<Mod>, Vec<Mod>) {
    let mut res_need_bad = vec![Mod::ZERO; 2];
    res_need_bad[r + 1 - (r)] = Mod::ONE;
    let mut res_has_bad = vec![Mod::ZERO; 1];
    res_has_bad[0] = Mod::ONE;
    for &to in &g[v] {
        if to == p {
            continue;
        }
        let (mut child_need, mut child_has) = dfs(g, r, to, v);
        for d in 1..child_need.len() {
            child_need[d - 1] = child_need[d];
        }
        child_need[r + 1 - r] = Mod::ZERO;
        child_has.push(Mod::ZERO);
        for d in (0..child_has.len() - 1).rev() {
            let tmp = child_has[d];
            child_has[d + 1] += tmp;
            child_has[d] = Mod::ZERO;
        }
        child_has[0] = Mod::ZERO;

        let (x, y) = join(&res_need_bad, &res_has_bad, &child_need, &child_has, r);
        res_need_bad = x;
        res_has_bad = y;
    }
    (res_need_bad, res_has_bad)
}

fn solve(g: &[Vec<usize>], r: usize) -> Mod {
    let (_, has) = dfs(g, r, 0, 0);
    let mut res = Mod::ZERO;
    for &ways in &has {
        res += ways;
    }
    res
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let mut g = vec![vec![]; n];
    for _ in 1..n {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut solved: Vec<_> = (0..=n).map(|r| solve(&g, r)).collect();
    solved[0] = Mod::ONE;
    let mut res = Mod::new(0);
    let mut total_ways = Mod::new(1);
    for _ in 0..n {
        total_ways = total_ways + total_ways;
    }
    solved.push(total_ways);
    for ans in (1..=n).rev() {
        let here = solved[ans + 1] - solved[ans];
        res += here * (Mod::new(ans as i32));
    }
    res -= Mod::ONE;
    res /= total_ways;
    writeln!(out, "{}", res).unwrap();
}
