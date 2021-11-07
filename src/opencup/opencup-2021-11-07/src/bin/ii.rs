use std::io;
use std::io::Write;
use std::cmp::min;
use std::collections::BTreeMap;

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

#[derive(Clone)]
struct Poly {
    a: Vec<i64>,
}

impl Poly {
    fn derivative(&self) -> Self {
        let mut b = vec![0; self.a.len() - 1];
        for i in 1..self.a.len() {
            b[i - 1] = self.a[i] * (i as i64);
        }
        Self { a: b }
    }
}

struct Node {
    next: BTreeMap<i64, Box<Node>>,
    cost: usize,
}

impl Node {
    fn new() -> Self {
        Self { next: BTreeMap::new(), cost: usize::MAX / 2 }
    }
}

fn add(node: &mut Box<Node>, poly: &[i64], cur_cost: usize) {
    node.cost = min(node.cost, cur_cost + poly.len() * 2);
    if poly.len() == 0 {
        return;
    }
    let last = *poly.last().unwrap();
    if !node.next.contains_key(&last) {
        node.next.insert(last, Box::new(Node::new()));
    }
    let next_node = node.next.get_mut(&last).unwrap();
    add(next_node, &poly[0..poly.len() - 1], cur_cost);
}


fn get(node: &Box<Node>, poly: &[i64]) -> usize {
    let mut res = node.cost;
    if poly.len() == 0 {
        return res;
    }

    let last = *poly.last().unwrap();
    if !node.next.contains_key(&last) {
        return res;
    }
    let next_node = node.next.get(&last).unwrap();
    res = min(res, get(next_node, &poly[0..poly.len() - 1]));
    res
}

struct WithCost {
    cost: usize,
    poly: Poly,
}

fn dist(p1: &Poly, p2: &Poly) -> usize {
    let mut same = 0;
    for i in (0..p1.a.len()).rev() {
        if p1.a[i] != p2.a[i] {
            break;
        }
        same += 1;
    }
    let more = p1.a.len() - same;
    more * 2
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();

    let mut a = vec![];

    let mut read_poly = || -> Poly {
        let k = sc.usize();
        let a = sc.vec::<i64>(k + 1);
        Poly { a }
    };

    let mut min_zero = usize::MAX;

    for _ in 0..n {
        a.push(read_poly());
    }

    for p in a.iter() {
        min_zero = min(min_zero, p.a.len());
    }

    let mut b = vec![];
    for _ in 0..m {
        b.push(read_poly());
    }

    const INF: i64 = 1.2e9 as i64;

    let mut with_cost = vec![];
    for poly in a.iter() {
        let mut cur_poly = poly.clone();
        let mut cur_cost = 0;
        loop {
            while with_cost.len() <= cur_poly.a.len() {
                with_cost.push(Box::new(Node::new()));
            }
            add(&mut with_cost[cur_poly.a.len()], &cur_poly.a, cur_cost);
            if cur_poly.a.len() == 0 {
                break;
            }
            let der = cur_poly.derivative();
            if der.a.len() != 0 && der.a[der.a.len() - 1] > INF {
                break;
            }
            cur_poly = der;
            cur_cost += 1;
        }
    }

    for i in 0..m {
        let mut cur_p = b[i].clone();
        let mut res = cur_p.a.len() + min_zero;
        let mut cur_add_cost = 0;
        loop {
            if with_cost.len() > cur_p.a.len() {
                res = min(res, get(&with_cost[cur_p.a.len()], &cur_p.a) + cur_add_cost);
            }
            if cur_p.a.len() == 0 {
                break;
            }
            cur_p = cur_p.derivative();
            cur_add_cost += 1;
            if cur_p.a.len() != 0 && cur_p.a[cur_p.a.len() - 1] > INF {
                break;
            }
        }
        writeln!(out, "{}", res).unwrap();
    }
}
