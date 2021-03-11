use std::io;
use std::io::Write;

pub fn main() {
    let mut sc = Scanner::default();
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let tc: usize = sc.next();
    for _ in 0..tc {
        let n: usize = sc.next();
        let m: usize = sc.next();
        let mut field = vec![Vec::new(); n];
        for i in 0..n {
            field[i] = sc.next_string();
        }
        let start_pos = if m % 3 == 1 {
            0
        } else {
            1
        };
        for y in (start_pos..m).step_by(3) {
            for x in 0..n {
                field[x][y] = b'X';
            }
        }
        for y in (start_pos..m).step_by(3) {
            if y + 3 >= m {
                break;
            }
            let mut changed = false;
            for x in 0..n {
                if field[x][y + 1] == b'X' {
                    field[x][y + 2] = b'X';
                    changed = true;
                }
                if field[x][y + 2] == b'X' {
                    field[x][y + 1] = b'X';
                    changed = true;
                }
                if changed {
                    break;
                }
            }
            if !changed {
                field[0][y + 1] = b'X';
                field[0][y + 2] = b'X';
            }
        }
        for x in 0..n {
            let str = String::from_utf8(field[x].clone()).unwrap();
            write!(out, "{}", str).unwrap();
            writeln!(out).unwrap();
        }
        // writeln!(out).unwrap();
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