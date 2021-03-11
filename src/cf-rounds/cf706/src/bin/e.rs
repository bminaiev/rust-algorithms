use std::io;
use std::io::Write;
use std::cmp::{min, max};

fn rnd(seed: &mut i64, base: i64) -> i64 {
    let ret = *seed;
    *seed = (*seed * base + 233) % 1000000007;
    ret
}

pub fn main() {
    let mut sc = Scanner::default();
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let n: usize = sc.next();
    let m: usize = sc.next();
    let mut p = vec![0; m + 1];
    let mut t = vec![0; n];
    let mut a = vec![0; n];
    for i in 1..=m {
        p[i] = sc.next();
        let ki: i64 = sc.next();
        let mut seed = sc.next();
        let base = sc.next();
        for j in p[i - 1] + 1..=p[i] {
            t[j - 1] = rnd(&mut seed, base) % 2;
            a[j - 1] = (rnd(&mut seed, base) % ki) + 1;
        }
    }
    let mut sum_t0 = 0;
    let mut sum_t1 = 0;
    for i in 0..n {
        if t[i] == 0 {
            sum_t0 += a[i];
        } else {
            sum_t1 += a[i];
        }
    }
    let empty_team = if sum_t0 < sum_t1 || (sum_t0 == sum_t1 && t[0] == 0) { 0 } else { 1 };
    let mut ans = vec![0; n];
    if empty_team != t[0] {
        ans[0] += 1;
        a[0] -= 1;
    }
    let mut iter = 0;
    for i in 0..n {
        if t[i] != empty_team {
            continue;
        }
        while a[i] != 0 {
            iter = max(iter, i + 1);
            while t[f(iter, n)] == empty_team || a[f(iter, n)] == 0 {
                iter += 1;
            }
            let used = min(a[f(iter, n)], a[i]);
            a[i] -= used;
            ans[i] += used;
            a[f(iter, n)] -= used;
            ans[f(iter, n)] += used;
        }
    }
    let mut res = 1;
    for i in 0..n {
        let mut t = (i + 1) as i64;
        t = t * t;
        t = t ^ ans[i];
        t += 1;
        t %= MOD as i64;
        res = mul(res, t as i32);
    }
    writeln!(out, "{}", res).unwrap();
}

fn f(iter: usize, n: usize) -> usize {
    if iter >= n { iter - n } else { iter }
}

const MOD: i32 = 1_000_000_007;

fn add(x: i32, y: i32) -> i32 {
    let res = x + y;
    if res >= MOD { res - MOD } else { res }
}

fn mul(x: i32, y: i32) -> i32 {
    ((x as i64) * (y as i64) % (MOD as i64)) as i32
}


#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

// Scanner code is copied from Russell Emerine's solution
// http://codeforces.com/contest/1477/submission/105755265
impl Scanner {
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
    fn next_string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}