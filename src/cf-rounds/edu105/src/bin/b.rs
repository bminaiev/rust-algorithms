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
    let tc = scanner.next::<usize>();
    for _ in 0..tc {
        let n: i32 = scanner.next();
        let u: i32 = scanner.next();
        let r: i32 = scanner.next();
        let d: i32 = scanner.next();
        let l: i32 = scanner.next();
        let mut ok = false;
        for lu in 0..2 {
            for ld in 0..2 {
                for ru in 0..2 {
                    for rd in 0..2 {
                        if !(0..=n - 2).contains(&(u - lu - ru)) {
                            continue;
                        }
                        if !(0..=n - 2).contains(&(d - ld - rd)) {
                            continue;
                        }
                        if !(0..=n - 2).contains(&(l - lu - ld)) {
                            continue;
                        }
                        if !(0..=n - 2).contains(&(r - ru - rd)) {
                            continue;
                        }
                        ok = true;
                    }
                }
            }
        }
        if ok {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}