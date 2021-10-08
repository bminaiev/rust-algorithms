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

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn vect_mul(p1: Point, p2: Point, p3: Point) -> f64 {
    (p2.x - p1.x) * (p3.y - p1.y) - (p2.y - p1.y) * (p3.x - p1.x)
}

const EPS: f64 = 1e-9;

fn intersect(p1: Point, p2: Point, p3: Point, p4: Point) -> Point {
    let d = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
    let xx = (p1.x * p2.y - p1.y * p2.x) * (p3.x - p4.x) - (p1.x - p2.x) * (p3.x * p4.y - p3.y * p4.x);
    let yy = (p1.x * p2.y - p1.y * p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x * p4.y - p3.y * p4.x);
    Point { x: xx / d, y: yy / d }
}

fn cut(mut fig: Vec<Point>, a: Point, b: Point) -> Vec<Point> {
    if fig.is_empty() {
        return fig;
    }
    fig.push(fig[0]);
    let mut res = vec![];
    for w in fig.windows(2) {
        let cur_sign = vect_mul(a, b, w[0]);
        if cur_sign >= -EPS {
            res.push(w[0]);
        }
        let next_sign = vect_mul(a, b, w[1]);
        if (cur_sign < -EPS && next_sign > EPS) || (cur_sign > EPS && next_sign < -EPS) {
            res.push(intersect(w[0], w[1], a, b));
        }
    }
    res
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let max_x = sc.i64() as f64;
    let max_y = sc.i64() as f64;
    let n = sc.usize();
    let pts: Vec<_> = (0..n).map(|_| {
        let x = sc.i64() as f64;
        let y = sc.i64() as f64;
        Point { x, y }
    }).collect();
    let ordered: Vec<_> = (0..n).map(|_| {
        let id = sc.usize() - 1;
        pts[id]
    }).collect();
    let mut border = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: max_x, y: 0.0 },
        Point { x: max_x, y: max_y },
        Point { x: 0.0, y: max_y },
    ];
    for i in 0..n {
        for j in i + 1..n {
            border = cut(border, ordered[j], ordered[i]);
        }
    }
    let mut res = 0.0;
    for i in 0..border.len() {
        let p1 = border[i];
        let p2 = border[(i + 1) % border.len()];
        res += vect_mul(Point { x: 0.0, y: 0.0 }, p1, p2);
    }
    writeln!(out, "{}", res.abs() / 2.0).unwrap();
}
