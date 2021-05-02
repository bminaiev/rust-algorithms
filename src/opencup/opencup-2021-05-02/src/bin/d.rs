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

fn calc_diff_dist(len: usize, k: i64) -> f64 {
    let len = len as i64;
    let at_least = k / len;
    let need = at_least * len;
    let more = k - need;
    assert!(more < len);
    let for_at_least = 1. / (1. + at_least as f64) * ((len - more) as f64);
    let for_more = 1. / (2. + at_least as f64) * (more as f64);
    return for_at_least + for_more;
}

fn find_y_for_x(x: i64) -> i64 {
    assert_ne!(x, 0);
    let mut left = 0;
    let mut right = x * 2 + 10;
    let x = x as f64;
    let delta = 1. / (x * (x + 1.0));
    while right - left > 1 {
        let mid = (left + right) / 2;
        let mid_f = mid as f64;
        let my_delta = 2.0 / (mid_f * (mid_f + 1.0));
        if my_delta < delta {
            right = mid;
        } else {
            left = mid;
        }
    }
    return right;
}

fn calc_two(same: usize, more: usize, k: i64) -> f64 {
    let same = same as i64;
    let more = more as i64;
    let mut left = 0;
    let mut right = k / more + 10;
    while right - left > 1 {
        let mid = (left + right) / 2;
        let mid2 = find_y_for_x(mid);
        let need_total = mid * more + mid2 * same;
        if need_total > k {
            right = mid;
        } else {
            left = mid;
        }
    }
    let mut res = std::f64::MAX;
    let exp_x = right;
    let exp_y = find_y_for_x(exp_x);
    for dx in -5..5 {
        for dy in -5..5 {
            let x = exp_x + dx;
            let y = exp_y + dy;
            if x < 0 || y < 0 {
                continue;
            }
            if x * more + y * same > k {
                continue;
            }
            let extra = k - (x * more + y * same);
            let x = x as f64;
            let y = y as f64;
            if extra < more {
                let more = more as f64;
                let extra = extra as f64;
                let same = same as f64;
                let cost = extra * (1.0 / (x + 2.0)) + (more - extra) * (1.0 / (x + 1.0)) + 2.0 * same / (1.0 + y);
                // dbg!(x, y, cost);
                if cost < res {
                    res = cost;
                }
            }
            if extra < same {
                let more = more as f64;
                let extra = extra as f64;
                let same = same as f64;
                let cost = extra * (2.0 / (y + 2.0)) + (same - extra) * (2.0 / (y + 1.0)) + 1.0 * more / (1.0 + x);
                dbg!(x, y, cost, "@");
                if cost < res {
                    res = cost;
                }
            }
        }
    }

    res
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let k = sc.i64();
    let mut g = vec![vec![]; n];
    for _ in 0..m {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let mut dist = vec![vec![std::usize::MAX; n]; n];
    for start in 0..n {
        dist[start][start] = 0;
        let mut queue = vec![start];
        let mut q_it = 0;
        while q_it < queue.len() {
            let v = queue[q_it];
            q_it += 1;
            for &to in g[v].iter() {
                if dist[start][to] > dist[start][v] + 1 {
                    dist[start][to] = dist[start][v] + 1;
                    queue.push(to);
                }
            }
        }
    }
    let s1 = sc.usize() - 1;
    let t1 = sc.usize() - 1;
    let s2 = sc.usize() - 1;
    let t2 = sc.usize() - 1;

    let dist1 = dist[s1][t1];
    let dist2 = dist[s2][t2];
    if dist1 == 0 && dist2 == 0 {
        writeln!(out, "0").unwrap();
        return;
    } else if dist1 == 0 || dist2 == 0 {
        let exist_dist = dist1 + dist2;
        let res = calc_diff_dist(exist_dist, k);
        writeln!(out, "{}", res).unwrap();
        return;
    }

    let mut smallest_with_same = vec![std::usize::MAX; n];
    smallest_with_same[0] = dist[s1][t1] + dist[s2][t2];
    const MAX: usize = std::usize::MAX;
    for v1 in 0..n {
        for v2 in 0..n {
            if dist[v1][v2] == MAX {
                continue;
            }
            if dist[s1][v1] == MAX || dist[v2][t1] == MAX {
                continue;
            }
            let same = dist[v1][v2];
            let first_more = dist[s1][v1] + dist[v2][t1];
            if dist[s2][v1] != MAX && dist[v2][t2] != MAX {
                let more = first_more + dist[s2][v1] + dist[v2][t2];
                smallest_with_same[same] = min(smallest_with_same[same], more);
            }
            if dist[s2][v2] != MAX && dist[v1][t2] != MAX {
                let more = first_more + dist[s2][v2] + dist[v1][t2];
                smallest_with_same[same] = min(smallest_with_same[same], more);
            }
        }
    }
    let mut res = calc_diff_dist(smallest_with_same[0], k);
    for (same, &more) in smallest_with_same.iter().enumerate().skip(1) {
        if more == MAX {
            continue;
        }
        // dbg!(same, more);
        let cur_res = calc_two(same, more, k);
        if cur_res < res {
            res = cur_res;
        }
    }
    writeln!(out, "{}", res).unwrap();
}
