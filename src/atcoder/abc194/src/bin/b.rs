use std::cmp::{max, min};

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

pub fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.next();
    let mut a = vec![0; n];
    let mut b = vec![0; n];
    for i in 0..n {
        a[i] = scanner.next();
        b[i] = scanner.next();
    }
    let mut res = std::i32::MAX;
    for i in 0..n {
        for j in 0..n {
            let mut cost = max(a[i], b[j]);
            if i == j {
                cost += min(a[i], b[j]);
            }
            res = min(res, cost);
        }
    }
    println!("{}", res);
}