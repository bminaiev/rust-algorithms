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
    let mut sum = 0i64;
    let mut sum_sq = 0i64;
    let mut res = 0i64;
    let mut cnt = 0i64;
    for _ in 0..n {
        let val: i64 = scanner.next();
        res += cnt * val * val + sum_sq - 2 * val * sum;
        cnt += 1;
        sum += val;
        sum_sq += val * val;
    }
    println!("{}", res);
}