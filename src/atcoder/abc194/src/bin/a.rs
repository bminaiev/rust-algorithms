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
    let a : usize = scanner.next();
    let b : usize = scanner.next();
    if a + b >= 15 && b >= 8 {
        println!("1");
    } else if a + b >= 10 && b >= 3 {
        println!("2");
    } else if a + b >= 3 {
        println!("3");
    } else {
        println!("4");
    }
}