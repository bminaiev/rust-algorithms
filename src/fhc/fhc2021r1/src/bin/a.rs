use std::io;
use std::io::Write;
use std::cmp::min;
use std::fs::File;

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
    let mut out = std::io::BufWriter::new(File::create("a.out").unwrap());
    let mut sc = Scanner::new_file("a.in");

    let tc = sc.usize();
    for t in 0..tc {
        let s = sc.string();
        let k = sc.usize();
        const SZ: usize = 26;
        const INF: i32 = std::i32::MAX / 3;
        let mut d = vec![vec![INF; SZ]; SZ];
        for i in 0..SZ {
            d[i][i] = 0;
        }
        for _ in 0..k {
            let edge = sc.string();
            d[(edge[0] - b'A') as usize][(edge[1] - b'A') as usize] = 1;
        }
        for i in 0..SZ {
            for j in 0..SZ {
                for k in 0..SZ {
                    d[j][k] = min(d[j][k], d[j][i] + d[i][k]);
                }
            }
        }
        let mut res = INF;
        for c in 0..SZ {
            let mut cur_cost = 0;
            for c1 in s.iter() {
                let cur = (c1 - b'A') as usize;
                cur_cost += d[cur][c];
                if cur_cost > INF {
                    break;
                }
            }
            res = min(res, cur_cost);
        }
        if res >= INF {
            res = -1;
        }
        writeln!(out, "Case #{}: {}", t + 1, res).unwrap();
    }
}
