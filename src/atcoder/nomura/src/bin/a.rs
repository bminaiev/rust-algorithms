use std::io;
use std::io::Write;
use std::cmp::{min, max};
use std::collections::BTreeSet;

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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    id: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let mut a: Vec<_> = (0..n).map(|id| {
        let x = sc.i64();
        let y = sc.i64();
        Point { x, y, id }
    }).collect();

    let mut pts = BTreeSet::new();
    const M: usize = 3;
    {
        a.sort_by(|p1, p2| p1.x.cmp(&p2.x));
        for it in 0..M {
            pts.insert(a[it]);
            pts.insert(a[a.len() - it - 1]);
        }
    }
    {
        a.sort_by(|p1, p2| p1.y.cmp(&p2.y));
        for it in 0..M {
            pts.insert(a[it]);
            pts.insert(a[a.len() - it - 1]);
        }
    }
    let mut dists = vec![];
    for p1 in pts.iter() {
        for p2 in pts.iter() {
            if p1.id <= p2.id {
                continue;
            }
            let d = max((p1.x - p2.x).abs(), (p1.y - p2.y).abs());
            dists.push(d);
        }
    }
    dists.sort();
    writeln!(out, "{}", dists[dists.len() - 2]).unwrap();
}
