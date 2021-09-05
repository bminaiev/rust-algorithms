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


#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Obj {
    score: i32,
    positive: bool,
}

#[derive(PartialOrd, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(tp: usize, tn: usize, fp: usize, fn_: usize) -> Self {
        let tp = tp as f64;
        let tn = tn as f64;
        let fp = fp as f64;
        let fn_ = fn_ as f64;
        Self {
            x: fp / (tn + fp),
            y: tp / (tp + fn_),
        }
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let mut objs = vec![];
    for _ in 0..n {
        let typ = sc.string();
        let score = sc.i32();
        objs.push(Obj { score, positive: typ[0] == b'+' });
    }
    objs.sort();
    // all positive
    let mut tp = 0;
    let mut tn = 0;
    let mut fp = 0;
    let mut fn_ = 0;
    for o in objs.iter() {
        if o.positive {
            tp += 1;
        } else {
            fp += 1;
        }
    }
    let mut points = vec![];
    points.push(Point::new(tp, tn, fp, fn_));

    for i in 0..n {
        if objs[i].positive {
            tp -= 1;
            fn_ += 1;
        } else {
            tn += 1;
            fp -= 1;
        }
        if i == n - 1 || objs[i].score != objs[i + 1].score {
            points.push(Point::new(tp, tn, fp, fn_));
        }
    }
    points.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut sorted : Vec<&Point> = vec![];
    for p in points.iter() {
        if sorted.is_empty() || p.y >= sorted.last().unwrap().y {
            sorted.push(p.clone());
        }
    }
    let mut res = 0.0;
    for w in sorted.windows(2) {
        let dx = w[1].x - w[0].x;
        let y = w[0].y;
        res += dx * y;
    }
    writeln!(out, "{}", res).unwrap();
}
