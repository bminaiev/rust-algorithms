use std::io;
use std::io::Write;
use std::cmp::min;

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

fn find_best(val: i64, vals: &Vec<i64>) -> i64 {
    assert_ne!(vals.len(), 0);
    let pos = match vals.binary_search(&val) {
        Ok(pos) => pos,
        Err(pos) => pos
    };
    if pos == vals.len() {
        return (val - vals[pos - 1]).abs();
    } else if pos == 0 {
        return (val - vals[0]).abs();
    }
    {
        return min(vals[pos] - val, val - vals[pos - 1]);
    }
}

fn solve(even: &mut Vec<i64>, odd1: &mut Vec<i64>, odd2: &mut Vec<i64>) -> i64 {
    even.sort();
    odd1.sort();
    odd2.sort();
    let mut res = std::i64::MAX;
    for o1 in odd1.iter() {
        res = min(res, find_best(*o1, odd2));
    }
    if !even.is_empty() {
        let mut b1 = std::i64::MAX;
        let mut b2 = std::i64::MAX;
        for o1 in odd1.iter() {
            b1 = min(b1, find_best(*o1, even));
        }
        for o2 in odd2.iter() {
            b2 = min(b2, find_best(*o2, even));
        }
        res = min(res, b1 + b2);
    }
    res
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize() * 2;
    let mut red = vec![];
    let mut green = vec![];
    let mut blue = vec![];
    for _ in 0..n {
        let val = sc.i64();
        let color = sc.string()[0];
        if color == b'R' {
            red.push(val);
        } else if color == b'G' {
            green.push(val);
        } else if color == b'B' {
            blue.push(val);
        } else {
            unreachable!();
        }
    }
    if red.len() % 2 == 0 && green.len() % 2 == 0 && blue.len() % 2 == 0 {
        writeln!(out, "0").unwrap();
        return;
    }
    if red.len() % 2 == 0 {
        let res = solve(&mut red, &mut green, &mut blue);
        writeln!(out, "{}", res).unwrap();
        return;
    }
    if green.len() % 2 == 0 {
        let res = solve(&mut green, &mut red, &mut blue);
        writeln!(out, "{}", res).unwrap();
        return;
    }
    if blue.len() % 2 == 0 {
        let res = solve(&mut blue, &mut green, &mut red);
        writeln!(out, "{}", res).unwrap();
        return;
    }
    unreachable!();
}
