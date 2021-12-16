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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Column {
    v1: i32,
    v2: i32,
    pos: usize,
}

impl Column {
    fn same(&self, rhs: &Self) -> bool {
        self.v1 == rhs.v1 && self.v2 == rhs.v2
    }
}

fn build(r1: &[i32], r2: &[i32]) -> Vec<Column> {
    let n = r1.len();
    let mut res = Vec::with_capacity(n);
    for pos in 0..n {
        let (v1, v2) = if pos % 2 == 0 {
            (r1[pos], r2[pos])
        } else {
            (r2[pos], r1[pos])
        };
        res.push(Column { v1, v2, pos });
    }
    res.sort();
    res
}

#[allow(dead_code)]
pub struct Fenwick {
    values: Vec<i64>,
}

impl Fenwick {
    #[allow(dead_code)]
    fn get_sum(&self, mut pos: usize) -> i64 {
        let mut res = 0i64;
        loop {
            res += self.values[pos] as i64;
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, mut pos: usize, change: i64) {
        while pos < self.values.len() {
            self.values[pos] += change;
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new(n: usize) -> Self {
        let values = vec![0; n];
        Fenwick { values }
    }
}

fn cost(from: &[Column], to: &[Column]) -> i64 {
    let mut res = 0i64;
    let n = from.len();
    let mut perm = vec![0; n];
    for i in 0..n {
        if !from[i].same(&to[i]) {
            return -1;
        }
        perm[from[i].pos] = to[i].pos;
    }
    let mut fenw = Fenwick::new(n);
    for pos in 0..n {
        let p = perm[pos];
        res += (pos as i64) - fenw.get_sum(p);
        fenw.add(p, 1);
    }
    res
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();

        let from = {
            let row1 = sc.vec::<i32>(n);
            let row2 = sc.vec::<i32>(n);
            build(&row1, &row2)
        };

        let to = {
            let row1 = sc.vec::<i32>(n);
            let row2 = sc.vec::<i32>(n);
            build(&row1, &row2)
        };

        let res = cost(&from, &to);
        writeln!(out, "{}", res).unwrap();
    }
}
