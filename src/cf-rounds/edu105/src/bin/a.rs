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

fn is_correct_bracket_seq(seq: &Vec<u8>) -> bool {
    let mut balance = 0;
    for &c in seq.iter() {
        if c == b'(' {
            balance += 1;
        } else {
            balance -= 1;
            if balance < 0 {
                return false;
            }
        }
    }
    balance == 0
}

fn is_ok(str: &Vec<u8>, a: u8, b: u8, c: u8) -> bool {
    let replaced = str.iter().map(|ch| match ch {
        b'A' => a,
        b'B' => b,
        b'C' => c,
        _ => unreachable!("!!!")
    }).collect();
    is_correct_bracket_seq(&replaced)
}

pub fn main() {
    let mut scanner = Scanner::default();
    let tc: usize = scanner.next();
    for _ in 0..tc {
        let str = scanner.next_string();
        let brackets = [b'(', b')'];
        let mut found = false;
        'outer: for &a in brackets.iter() {
            for &b in brackets.iter() {
                for &c in brackets.iter() {
                    if is_ok(&str, a, b, c) {
                        println!("YES");
                        found = true;
                        break 'outer;
                    }
                }
            }
        }
        if !found {
            println!("NO");
        }
    }
}