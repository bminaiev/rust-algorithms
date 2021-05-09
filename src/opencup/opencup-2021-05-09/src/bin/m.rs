use std::io;
use std::io::{Write};

/**************************************************

    START OF TEMPLATE CODE

 *************************************************/
#[allow(unused_macros)]
macro_rules! dbg {
    ($first_val:expr, $($val:expr),+ $(,)?) => {
        eprint!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($first_val), &$first_val);
        ($(eprint!(", {} = {:?}", stringify!($val), &$val)),+,);
        eprintln!();
    };
    ($first_val:expr) => {
        eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($first_val), &$first_val);
    };
}

enum InputSource {
    Stdin,
    FromFile(Vec<String>),
}

struct Scanner {
    buffer: Vec<String>,
    input_source: InputSource,
}

impl Scanner {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { buffer: vec![], input_source: InputSource::Stdin }
    }

    #[allow(dead_code)]
    fn new_file(filename: &str) -> Self {
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
        lines.reverse();
        Self { buffer: vec![], input_source: InputSource::FromFile(lines) }
    }


    #[allow(dead_code)]
    fn i64(&mut self) -> i64 {
        self.next::<i64>()
    }

    #[allow(dead_code)]
    fn i32(&mut self) -> i32 {
        self.next::<i32>()
    }

    #[allow(dead_code)]
    fn usize(&mut self) -> usize {
        self.next::<usize>()
    }

    #[allow(dead_code)]
    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            match &mut self.input_source {
                | InputSource::Stdin => { std::io::stdin().read_line(&mut input).expect("Failed read"); }
                | InputSource::FromFile(lines) => {
                    let line = lines.pop().unwrap();
                    input = line;
                }
            }

            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    #[allow(dead_code)]
    fn string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/


fn rev_ver(c: u8) -> u8 {
    if c == b'b' {
        return b'p';
    }
    if c == b'p' {
        return b'b';
    }
    if c == b'q' {
        return b'd';
    }
    if c == b'd' {
        return b'q';
    }
    return c;
}


fn rev_hor(c: u8) -> u8 {
    if c == b'b' {
        return b'd';
    }
    if c == b'p' {
        return b'q';
    }
    if c == b'q' {
        return b'p';
    }
    if c == b'd' {
        return b'b';
    }
    return c;
}

const M: usize = std::usize::MAX;

fn print(out: &mut Vec<u8>, fr: usize, to: usize, rev: bool, prev: &[usize], next: &[usize], s: &Vec<u8>) {
    if fr >= to {
        return;
    }
    if rev {
        let c = s[to - 1];
        if c == b')' {
            let opened = prev[to - 1];
            print(out, opened + 1, to - 1, !rev, prev, next, s);
            print(out, fr, opened, rev, prev, next, s);
            return;
        } else if c == b'(' {
            assert!(false);
        } else if c == b'[' || c == b']' {
            print(out, fr, to - 1, rev, prev, next, s);
            return;
        } else {
            out.push(rev_hor(c));
            print(out, fr, to - 1, rev, prev, next, s);
        }
    } else {
        let c = s[fr];
        if c == b'(' {
            let closed = next[fr];
            print(out, fr + 1, closed, !rev, prev, next, s);
            print(out, closed + 1, to, rev, prev, next, s);
            return;
        } else if c == b')' {
            assert!(false);
        } else if c == b'[' || c == b']' {
            print(out, fr + 1, to, rev, prev, next, s);
            return;
        } else {
            out.push(c);
            print(out, fr + 1, to, rev, prev, next, s);
        }
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let mut s = sc.string();
    let n = s.len();
    let mut ver_balance = 0;
    for i in 0..n {
        let c = s[i];
        if c == b'[' || c == b']' {
            ver_balance ^= 1;
        }
        if ver_balance == 1 {
            s[i] = rev_ver(s[i]);
        }
    }
    let mut next = vec![M; n];
    let mut prev = vec![M; n];
    let mut stack = vec![];
    for i in 0..n {
        let c = s[i];
        if c == b'(' {
            stack.push(i);
        } else if c == b')' {
            assert!(stack.len() > 0);
            let last = stack.pop().unwrap();
            next[last] = i;
            prev[i] = last;
        }
    }
    let mut out_vec = vec![];
    print(&mut out_vec, 0, n, false, &prev, &next, &s);
    let s = String::from_utf8(out_vec).unwrap();
    writeln!(out, "{}", s).unwrap();
}
