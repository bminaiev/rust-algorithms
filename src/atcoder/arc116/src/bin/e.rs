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

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
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
            std::io::stdin().read_line(&mut input).expect("Failed read");
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


struct DfsResult {
    cost: usize,
    len_to_not_covered: i32,
}

fn join(left: &DfsResult, right: &DfsResult, max_dist: i32) -> DfsResult {
    let cost = left.cost + right.cost;
    if left.len_to_not_covered >= 0 && right.len_to_not_covered >= 0 {
        return DfsResult { cost, len_to_not_covered: max(left.len_to_not_covered, right.len_to_not_covered) };
    } else if left.len_to_not_covered < 0 && right.len_to_not_covered < 0 {
        return DfsResult { cost, len_to_not_covered: max(left.len_to_not_covered, right.len_to_not_covered) };
    } else {
        let covered = -min(left.len_to_not_covered, right.len_to_not_covered);
        if covered + left.len_to_not_covered <= max_dist && covered + right.len_to_not_covered <= max_dist {
            return DfsResult { cost, len_to_not_covered: -covered };
        } else {
            return DfsResult { cost, len_to_not_covered: max(left.len_to_not_covered, right.len_to_not_covered) };
        }
    }
}

fn dfs(v: usize, p: usize, max_dist: i32, g: &Vec<Vec<usize>>) -> DfsResult {
    let mut ret = DfsResult { cost: 0, len_to_not_covered: 0 };
    for &to in g[v].iter() {
        if to == p {
            continue;
        }
        let child_res = dfs(to, v, max_dist, g);
        ret = join(&ret, &child_res, max_dist);
    }
    if ret.len_to_not_covered >= 0 {
        ret.len_to_not_covered += 1;
    } else {
        ret.len_to_not_covered -= 1;
    }
    if ret.len_to_not_covered > max_dist {
        ret.cost += 1;
        ret.len_to_not_covered = -1;
    }
    ret
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let k = sc.usize();

    let mut g = vec![vec![]; n];
    for _ in 1..n {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut l = -1;
    let mut r = n as i32 + 1;
    while r - l > 1 {
        let check_time = (l + r) / 2;
        let res = dfs(0, 0, check_time, &g);
        let cost = if res.len_to_not_covered > 0 {
            res.cost + 1
        } else {
            res.cost
        };
        if cost <= k {
            r = check_time;
        } else {
            l = check_time;
        }
    }
    writeln!(out, "{}", r).unwrap();
}
