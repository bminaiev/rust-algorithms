use std::cmp::max;
use std::io;
use std::io::Write;

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
        Self {
            buffer: vec![],
            input_source: InputSource::Stdin,
        }
    }

    #[allow(dead_code)]
    fn new_file(filename: &str) -> Self {
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
        lines.reverse();
        Self {
            buffer: vec![],
            input_source: InputSource::FromFile(lines),
        }
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

    fn parse_next_line(&mut self) -> bool {
        let mut input = String::new();
        match &mut self.input_source {
            InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            InputSource::FromFile(lines) => match lines.pop() {
                Some(line) => input = line,
                None => return false,
            },
        }

        self.buffer = input.split_whitespace().rev().map(String::from).collect();
        return true;
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }

            self.parse_next_line();
        }
    }

    #[allow(dead_code)]
    fn has_more_elements(&mut self) -> bool {
        loop {
            if !self.buffer.is_empty() {
                return true;
            }
            if !self.parse_next_line() {
                return false;
            }
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

fn split(range: &[i32], bit1: usize) -> (&[i32], &[i32]) {
    assert!(bit1 <= 21);
    let mut pos = 0;
    while pos != range.len() && ((1 << (bit1 - 1)) & range[pos]) == 0 {
        pos += 1;
    }
    (&range[..pos], &range[pos..])
}

fn solve(lhs: &[i32], rhs: &[i32], bit1: usize, same: bool, k: i32) -> usize {
    if bit1 == 0 && same {
        return lhs.len();
    }
    if lhs.len() == 0 || rhs.len() == 0 || bit1 == 0 {
        return lhs.len() + rhs.len();
    }
    let (lhs0, lhs1) = split(lhs, bit1);
    let (rhs0, rhs1) = split(rhs, bit1);
    if ((1 << (bit1 - 1)) & k) == 0 {
        let if_split = max(
            solve(lhs0, rhs0, bit1 - 1, same, k),
            solve(lhs1, rhs1, bit1 - 1, same, k),
        );
        let full_subtree = if same { 0 } else { max(lhs.len(), rhs.len()) };
        max(full_subtree, if_split)
    } else {
        let left = solve(lhs0, rhs1, bit1 - 1, false, k);
        if same {
            return left;
        } else {
            let right = solve(lhs1, rhs0, bit1 - 1, false, k);
            return left + right;
        }
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let k = sc.i32();
        let mut a = sc.vec::<i32>(n);
        a.sort();
        writeln!(out, "{}", solve(&a, &a, 21, true, k)).unwrap();
    }
}
