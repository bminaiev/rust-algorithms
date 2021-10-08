use std::io;
use std::io::Write;
use std::cmp::{max, min};

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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn rotate_left(&self, around: Point) -> Self {
        let dx = self.x - around.x;
        let dy = self.y - around.y;
        Point { x: around.x + dy, y: around.y - dx }
    }

    fn rotate_right_as_box(&self, around: Point) -> Self {
        let dx = self.x - around.x;
        let dy = self.y - around.y;
        Point { x: around.x - 1 - dy, y: around.y + dx }
    }

    const ZERO: Self = Point { x: 0, y: 0 };
}

const MAX: usize = 63;

#[derive(Debug, Copy, Clone)]
struct Box {
    bottom_left: Point,
    top_right: Point,
}

impl Box {
    fn new(diag: Point) -> Self {
        Self {
            bottom_left: Point { x: min(diag.x, 0), y: min(diag.y, 0) },
            top_right: Point { x: max(diag.x, 0), y: max(diag.y, 0) },
        }
    }

    fn union(boxes: &[Box]) -> Self {
        let min_x = boxes.iter().map(|b| b.bottom_left.x).min().unwrap();
        let min_y = boxes.iter().map(|b| b.bottom_left.y).min().unwrap();
        let max_x = boxes.iter().map(|b| b.top_right.x).max().unwrap();
        let max_y = boxes.iter().map(|b| b.top_right.y).max().unwrap();

        Self {
            bottom_left: Point { x: min_x, y: min_y },
            top_right: Point { x: max_x, y: max_y },
        }
    }

    fn rotate(&self, around: Point) -> Self {
        let top_left = self.bottom_left.rotate_left(around);
        let bottom_right = self.top_right.rotate_left(around);
        Self {
            bottom_left: Point { x: top_left.x, y: bottom_right.y },
            top_right: Point { x: bottom_right.x, y: top_left.y },
        }
    }

    fn contains(&self, p: Point) -> bool {
        return p.x >= self.bottom_left.x && p.x < self.top_right.x && p.y >= self.bottom_left.y && p.y < self.top_right.y;
    }
}


#[derive(Debug)]
struct CurvePart {
    last: Point,
    bounding_box: Box,
    len: i64,
}

impl CurvePart {
    fn new(first: Point) -> Self {
        Self { last: first, bounding_box: Box::new(first), len: 1 }
    }

    fn gen(curves: &[CurvePart]) -> Self {
        let around = curves.last().unwrap().last;
        let boxes: Vec<_> = curves.iter().map(|c| c.bounding_box).collect();
        let bounding_box = Box::union(&boxes);
        let len = curves.iter().map(|c| c.len).sum();
        Self { last: Point::ZERO.rotate_left(around), bounding_box: bounding_box.rotate(around), len }
    }
}

fn find(p: Point, curves: &[CurvePart]) -> Option<i64> {
    if curves.len() == 1 {
        return if curves[0].bounding_box.contains(p) {
            Some(0)
        } else {
            None
        };
    }
    let last_curve = curves.last().unwrap();
    if last_curve.bounding_box.contains(p) {
        let around = curves[curves.len() - 2].last;
        if let Some(pos) = find(p.rotate_right_as_box(around), &curves[..curves.len() - 1]) {
            return Some(last_curve.len * 2 - 1 - pos);
        }
    }
    return find(p, &curves[..curves.len() - 1]);
}

#[derive(Debug)]
struct Res {
    curve_id: usize,
    pos: i64,
}

fn solve(p: Point, curves: &[Vec<CurvePart>]) -> Res {
    for (curve_id, curves) in curves.iter().enumerate() {
        if let Some(pos) = find(p, curves) {
            return Res { curve_id, pos };
        }
    }
    unreachable!();
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();


    let init_dirs = vec![
        Point { x: 1, y: 1 },
        Point { x: -1, y: 1 },
        Point { x: -1, y: -1 },
        Point { x: 1, y: -1 },
    ];

    let curves: Vec<_> = init_dirs.into_iter().map(|dir| -> Vec<CurvePart> {
        let first = CurvePart::new(dir);
        let mut curves = vec![first];
        while curves.len() < MAX {
            curves.push(CurvePart::gen(&curves));
        }
        curves
    }
    ).collect();

    let tc = sc.usize();
    for _ in 0..tc {
        let x = sc.i64();
        let y = sc.i64();

        let res = solve(Point { x, y }, &curves);
        writeln!(out, "{} {}", res.curve_id + 1, res.pos + 1).unwrap();
    }
}
