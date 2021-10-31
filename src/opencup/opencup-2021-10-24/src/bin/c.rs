use std::io;
use std::io::Write;
use std::cmp::min;
use std::collections::{BTreeSet, BTreeMap};

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

struct Dsu {
    p: Vec<usize>,
    min_id: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        let min_id = (0..n).collect();
        Self { p, min_id }
    }

    fn get(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        } else {
            self.p[v] = self.get(self.p[v]);
            return self.p[v];
        }
    }

    fn get_id(&mut self, mut v: usize) -> usize {
        v = self.get(v);
        self.min_id[v]
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        if x == y {
            return;
        }
        self.min_id[y] = min(self.min_id[x], self.min_id[y]);
        self.p[x] = y;
    }
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Edge {
    cost: i64,
    fr: usize,
    to: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Change {
    cost: i64,
    sets: Vec<Vec<usize>>,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Plan {
    changes: Vec<Change>,
    id: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let num_plans = sc.usize();
    let n = sc.usize();

    let mut plans = vec![];

    for plan_id in 0..num_plans {
        let mut edges = vec![];
        for _ in 0..(n - 1) {
            let fr = sc.usize() - 1;
            let to = sc.usize() - 1;
            let cost = sc.i64();
            edges.push(Edge { fr, to, cost });
        }
        edges.sort();
        edges.reverse();
        let mut dsu = Dsu::new(n);
        let mut i = 0;

        let mut changes = vec![];

        while i != edges.len() {
            let mut j = i;
            while j != edges.len() && edges[i].cost == edges[j].cost {
                j += 1;
            }

            let mut sets = vec![];

            let mut min_ids = BTreeSet::new();

            for edge in edges[i..j].iter() {
                min_ids.insert(dsu.get_id(edge.fr));
                min_ids.insert(dsu.get_id(edge.to));
            }

            for edge in edges[i..j].iter() {
                dsu.unite(edge.fr, edge.to);
            }

            let mut by_min_id: BTreeMap<usize, Vec<usize>> = BTreeMap::new();

            for &prev_id in min_ids.iter() {
                let now_min_id = dsu.get_id(prev_id);
                by_min_id.entry(now_min_id).or_default().push(prev_id);
            }

            for (key, val) in by_min_id.into_iter() {
                sets.push(val);
            }

            changes.push(Change { cost: edges[i].cost, sets });

            i = j;
        }

        plans.push(Plan { id: plan_id, changes });
    }
    plans.sort();
    let mut res = vec![0; num_plans];
    let mut i = 0;
    while i != plans.len() {
        let mut j = i;
        while j != plans.len() && plans[i].changes.eq(&plans[j].changes) {
            j += 1;
        }
        let min_plan_id = plans[i].id;
        while i != j {
            res[plans[i].id] = min_plan_id;
            i += 1;
        }
    }
    for plan_id in res.iter() {
        write!(out, "{} ", plan_id + 1).unwrap();
    }
}
