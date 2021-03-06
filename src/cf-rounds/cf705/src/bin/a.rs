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
    let mut sc = Scanner::default();
    let tc: usize = sc.next();
    for _ in 0..tc {
        let n: usize = sc.next();
        let k: usize = sc.next();
        let mut res = vec![0; 0];
        for x in (k + 1) / 2..=n {
            if x != k {
                res.push(x);
            }
        }
        println!("{}", res.len());
        for x in &res {
            print!("{} ", x);
        }
        println!();
    }
}