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
    let mut res = 0f64;
    for size in 1..n {
        let cnt_other_vertices = n - size;
        let prob_other = (cnt_other_vertices as f64) / (n as f64);
        res += 1. / prob_other;
    }
    println!("{}", res);
}