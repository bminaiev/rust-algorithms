use std::io;
use std::io::Write;
use std::cmp::{min, max};
use std::collections::BTreeSet;
use std::mem::swap;

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
    elems: Vec<BTreeSet<usize>>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        Self {
            p,
            elems: (0..n).map(|id| {
                let mut set = BTreeSet::new();
                set.insert(id);
                set
            }).collect(),
        }
    }

    fn get(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        } else {
            self.p[v] = self.get(self.p[v]);
            return self.p[v];
        }
    }

    fn append(&mut self, from: usize, to: usize) {
        // oh, this beautiful world of borrow checking...
        // let's prove that [from] and [to] are not equal...
        if to > from {
            let pair = self.elems.split_at_mut(from + 1);
            pair.1[to - from - 1].append(&mut pair.0[pair.0.len() - 1]);
        } else {
            let pair = self.elems.split_at_mut(to + 1);
            pair.0[pair.0.len() - 1].append(&mut pair.1[from - to - 1]);
        }
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return;
        }
        if self.elems[x].len() > self.elems[y].len() {
            let tmp = x;
            x = y;
            y = tmp;
        }
        self.p[x] = y;
        self.append(x, y);
    }
}


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();

    let mut alive = BTreeSet::new();

    let mut dsu = Dsu::new(n);

    for _ in 0..m {
        let q_type = sc.usize();
        let x = sc.usize() - 1;
        let y = sc.usize() - 1;
        let fr = min(x, y);
        let to = max(x, y);
        alive.insert(fr);
        alive.insert(to);

        if q_type == 1 {
            dsu.unite(fr, to);
            let mut iter = fr + 1;
            let mut to_join = vec![];
            loop {
                let next = *alive.range(iter..).next().unwrap();

                if next >= to {
                    break;
                }

                let id = dsu.get(next);
                if *dsu.elems[id].iter().next().unwrap() <= fr || *dsu.elems[id].iter().next_back().unwrap() >= to {
                    to_join.push(next);
                }

                let max_right_edge = *dsu.elems[id].range(..to).next_back().unwrap_or(&0);
                assert!(max_right_edge <= to);
                assert!(max_right_edge >= next);

                iter = max_right_edge + 1;
            }
            for v in to_join.into_iter() {
                dsu.unite(v, fr);
            }
        } else {
            assert_eq!(q_type, 2);
            write!(out, "{}", if dsu.get(fr) == dsu.get(to) { 1 } else { 0 }).unwrap();
        }
    }
    writeln!(out).unwrap();
}
