use std::io;
use std::io::Write;
use std::cmp::{min, max};

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

struct Dsu {
    p: Vec<usize>,
    min_elem: Vec<usize>,
    max_elem: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        Self { p, min_elem: (0..n).collect(), max_elem: (0..n).collect() }
    }

    fn get(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        } else {
            self.p[v] = self.get(self.p[v]);
            return self.p[v];
        }
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        self.p[x] = y;
        self.min_elem[y] = min(self.min_elem[y], self.min_elem[x]);
        self.max_elem[y] = max(self.max_elem[y], self.max_elem[x]);
    }
}


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();

    let mut dsu = Dsu::new(n);

    const BUBEN: usize = 100;
    let mut do_slow = 100;
    for _ in 0..m {
        let q_type = sc.usize();
        let x = sc.usize() - 1;
        let y = sc.usize() - 1;
        let fr = min(x, y);
        let to = max(x, y);
        if q_type == 1 {
            dsu.unite(fr, to);
            do_slow -= 1;
            let intervals = if fr + BUBEN * 2 < to && do_slow < 0 {
                vec![(fr..fr + BUBEN), to - BUBEN..to]
            } else {
                vec![fr..to]
            };
            for interval in intervals.iter() {
                for pos in interval.start..=interval.end {
                    let id = dsu.get(pos);
                    if dsu.min_elem[id] <= fr || dsu.max_elem[id] >= to {
                        dsu.unite(id, fr);
                    }
                }
            }
        } else {
            assert_eq!(q_type, 2);
            write!(out, "{}", if dsu.get(fr) == dsu.get(to) { 1 } else { 0 }).unwrap();
        }
    }
    writeln!(out).unwrap();
}
