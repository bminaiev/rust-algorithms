use std::io;
use std::io::Write;
use std::cmp::max;

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

    fn parse_next_line(&mut self) -> bool {
        let mut input = String::new();
        match &mut self.input_source {
            | InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            | InputSource::FromFile(lines) => {
                match lines.pop() {
                    Some(line) => input = line,
                    None => return false,
                }
            }
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


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();
    let t = sc.usize();
    let mut query = vec![0; t];
    let mut n = 0;
    for i in 0..t {
        query[i] = sc.usize();
        n = max(n, query[i]);
    }
    n += 1;
    let d = (n as f64).sqrt() as usize + 1;
    let mut a = vec![vec![0.0 as f64; d]; 3];
    let mut answers = vec![0; n];
    for total in 1..n {
        for b in 0..d {
            let r = total as i32 - b as i32;
            if r < 0 {
                continue;
            }
            if b == 0 {
                a[total % 3][b] = 1.0;
            } else if b as i32 > r {
                a[total % 3][b] = 0.0;
            } else {
                if r >= 2 {
                    let prob = (b as f64) / (total as f64);
                    a[total % 3][b] = prob * a[(total + 1) % 3][b - 1] + (1.0 - prob) * a[(total + 1) % 3][b];
                    //a[r][b] = prob * a[r - 1][b - 1] + (1.0 - prob) * a[r - 2][b];
                } else {
                    assert!(b == 1 && r == 1);
                    a[total][b] = 0.5;
                }
            }
        }
        let mut best_error = 1.0;
        let mut nb: i32 = -1;
        for b in 1..d {
            if (total as i32) - (b as i32) >= 0 {
                let cur_error = (a[total % 3][b] - 0.5).abs();
                if cur_error < best_error {
                    nb = b as i32;
                    best_error = cur_error;
                }
            }
        }
        answers[total] = nb;
    }
    for qq in query {
        writeln!(out, "{}", answers[qq]).unwrap();
    }
}
