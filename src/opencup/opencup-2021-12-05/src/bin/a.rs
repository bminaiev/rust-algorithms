use std::cmp::min;
use std::io;
use std::io::Write;
use std::time::Instant;

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
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn d2(&self, other: &Point) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

struct Dsu {
    p: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        Self { p }
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
    }
}

fn solve(pts: &[Point], k: usize) -> Vec<usize> {
    let n = pts.len();
    let mut ids: Vec<_> = (0..k).collect();
    let mut ids_mins = vec![i64::MAX; k];
    for i in 0..k {
        for j in 0..k {
            if i == j {
                continue;
            }
            ids_mins[i] = min(ids_mins[i], pts[i].d2(&pts[j]));
        }
    }

    let mut dsu = Dsu::new(n);
    for i in k..n {
        ids.push(i);
        ids_mins.push(i64::MAX);
        for j in 0..k {
            let d2 = pts[ids[j]].d2(&pts[ids[k]]);
            ids_mins[j] = min(ids_mins[j], d2);
            ids_mins[k] = min(ids_mins[k], d2);
        }
        let mut min_pos = 0;
        for j in 1..=k {
            if ids_mins[j] < ids_mins[min_pos] {
                min_pos = j;
            }
        }
        let mut min_pos2 = min_pos;
        let mut cur_d2 = i64::MAX;
        for (j, &ids_j) in ids.iter().enumerate() {
            if j == min_pos {
                continue;
            }
            let d2 = pts[ids_j].d2(&pts[ids[min_pos]]);
            if d2 < cur_d2 {
                cur_d2 = d2;
                min_pos2 = j;
            }
        }
        assert_ne!(min_pos, min_pos2);

        dsu.unite(ids[min_pos], ids[min_pos2]);

        ids.swap_remove(min_pos2);
        ids_mins.swap_remove(min_pos2);

        assert_ne!(min_pos, k);

        ids_mins[min_pos] = i64::MAX;
        for j in 0..k {
            if j == min_pos {
                continue;
            }
            ids_mins[min_pos] = min(ids_mins[min_pos], pts[ids[min_pos]].d2(&pts[ids[j]]));
        }

        assert_eq!(ids.len(), k);
        assert_eq!(ids_mins.len(), k);
    }
    let mut res = Vec::with_capacity(n);
    let mut by_dsu_id = vec![0; n];
    let mut cnt = 0;
    for i in 0..n {
        if dsu.get(i) == i {
            cnt += 1;
            by_dsu_id[i] = cnt;
        }
    }
    for i in 0..n {
        let val = by_dsu_id[dsu.get(i)];
        assert_ne!(val, 0);
        assert!(val <= k);
        res.push(val);
    }
    res
}

#[allow(dead_code)]
struct Random {
    state: u64,
}

impl Random {
    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    #[allow(dead_code)]
    fn next_in_range(&mut self, from: usize, to: usize) -> usize {
        assert!(from < to);
        (from as u64 + self.next() % ((to - from) as u64)) as usize
    }

    #[allow(dead_code)]
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }

    #[allow(dead_code)]
    fn new(seed: u64) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }
}

fn stress() -> bool {
    let n = 2_000_000;
    let mut rnd = Random::new(787788);
    for _ in 0..5 {
        const MAX_C: usize = 1_000_000_000;
        let pts: Vec<_> = (0..n)
            .map(|_| Point {
                x: rnd.next_in_range(0, MAX_C) as i64,
                y: rnd.next_in_range(0, MAX_C) as i64,
            })
            .collect();
        const K: usize = 20;
        solve(&pts, K);
    }
    return true;
}

pub fn main() {
    // let start = Instant::now();
    // if stress() {
    //     dbg!(start.elapsed().as_millis());
    //     return;
    // }
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let k = sc.usize();
        let pts: Vec<_> = (0..n)
            .map(|_| {
                let x = sc.i64();
                let y = sc.i64();
                Point { x, y }
            })
            .collect();
        let res = solve(&pts, k);
        for r in res.iter() {
            write!(out, "{} ", r).unwrap();
        }
        writeln!(out).unwrap();
    }
}
