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
    fn usize_opt(&mut self) -> Option<usize> {
        self.next_opt::<usize>()
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

    fn next_opt<T: std::str::FromStr>(&mut self) -> Option<T> {
        for _ in 0..3 {
            if let Some(token) = self.buffer.pop() {
                return Some(token.parse().ok().expect("Failed parse"));
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
        return None;
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

    loop {
        let n = match sc.usize_opt() {
            Some(n) => n,
            None => break,
        };
        let mut g = vec![];
        for _ in 0..n {
            let s = sc.string();
            let mut gg = vec![false; n];
            for i in 0..n {
                if s[i] == b'1' {
                    gg[i] = true;
                }
            }
            g.push(gg);
        }
        let mut res = vec![0, 1];
        for v in 2..n {
            let c = |res: &Vec<usize>, pos: usize| -> bool {
                let p1 = res[pos];
                let p2 = res[(pos + 1) % res.len()];
                return g[p1][p2];
            };
            let cur_sz = res.len();
            let mut all_zeros = true;
            let mut all_ones = true;
            for i in 0..cur_sz {
                if c(&res, i) == false {
                    all_ones = false;
                } else {
                    all_zeros = false;
                }
            }
            if all_zeros {
                res.insert(0, v);
                let last = g[*res.last().unwrap()][v];
                if last {
                    let last_elem = res.pop().unwrap();
                    res.insert(0, last_elem);
                }
            } else if all_ones {
                if g[res[0]][v] {
                    res.insert(0, v);
                } else {
                    res.push(v);
                }
            } else {
                let mut last_one = 0;
                for i in 0..cur_sz {
                    if c(&res, i) && !c(&res, i + 1) {
                        last_one = i;
                        break;
                    }
                }
                assert!(c(&res, last_one) && !c(&res, last_one + 1));
                let p1 = res[last_one];
                let p2 = res[(last_one + 1) % res.len()];
                let p3 = res[(last_one + 2) % res.len()];
                if g[p1][v] {
                    res.insert(last_one + 1, v);
                } else if !g[p3][v] {
                    res.insert(last_one + 2, v);
                } else if g[p2][v] {
                    res.insert(last_one + 2, v);
                } else {
                    res.insert(last_one + 1, v);
                }
            }
            {
                let mut seen_zeros = false;
                for i in 0..res.len() {
                    let cur_c = c(&res, i);
                    if !cur_c {
                        seen_zeros = true;
                    } else {
                        assert!(!seen_zeros);
                    }
                }
            }
        }
        for v in &res {
            write!(out, "{} ", v + 1).unwrap();
        }
        writeln!(out).unwrap();
    }
}
