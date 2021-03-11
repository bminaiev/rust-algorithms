use std::io;
use std::io::Write;

pub fn main() {
    let mut sc = Scanner::default();
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let tc: usize = sc.next();
    for _ in 0..tc {
        let n: usize = sc.next();
        let mut ys = Vec::with_capacity(n);
        let mut xs = Vec::with_capacity(n);
        for _ in 0..(2 * n) {
            let x: i32 = sc.next();
            let y: i32 = sc.next();
            if x == 0 {
                ys.push(y.abs());
            } else {
                xs.push(x.abs());
            }
        }
        xs.sort();
        ys.sort();
        let mut res = 0f64;
        for i in 0..n {
            let x = xs[i] as f64;
            let y = ys[i] as f64;
            res += (x * x + y * y).sqrt();
        }
        writeln!(out, "{}", res).unwrap();
    }
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