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

#[allow(dead_code)]
struct Random {
    state: usize,
}

impl Random {
    fn next(&mut self) -> usize {
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
        from + self.next() % (to - from)
    }

    #[allow(dead_code)]
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }

    #[allow(dead_code)]
    fn new(seed: usize) -> Self {
        assert_ne!(seed, 0);
        Self {
            state: seed,
        }
    }
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/


#[derive(Copy, Clone, Debug, Default)]
struct Point {
    x: f64,
    y: f64,
}

fn calc(mid: Point, angle: f64, r: f64) -> Point {
    let nx = mid.x + r * f64::cos(angle);
    let ny = mid.y + r * f64::sin(angle);
    Point { x: nx, y: ny }
}

impl Point {
    fn dist(&self, p: Point) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn solve(a: &[Point], r: f64) -> f64 {
    if a.len() == 2 {
        let dist = a[0].dist(a[1]);
        return dist - 2.0 * r;
    }


    let n = a.len();

    const BUBEN: usize = 300;

    let start = Instant::now();


    let mut angles = vec![0.0; BUBEN];
    for i in 0..BUBEN {
        angles[i] = std::f64::consts::PI * 2.0 * i as f64 / (BUBEN as f64) + 1.23;
    }
    let mut pts = vec![vec![Point::default(); BUBEN]; n];
    for i in 0..n {
        for j in 0..BUBEN {
            pts[i][j] = calc(a[i], angles[j], r);
        }
    }
    let mut dp = vec![vec![f64::MAX; BUBEN]; n];
    for i in 0..BUBEN {
        dp[0][i] = 0.0;
    }
    let mut prev = vec![vec![0; BUBEN]; n];
    for i in 0..(n - 1) {
        for j in 0..BUBEN {
            for k in 0..BUBEN {
                let d = pts[i][j].dist(pts[i + 1][k]);
                let nd = dp[i][j] + d;
                if dp[i + 1][k] > nd {
                    dp[i + 1][k] = nd;
                    prev[i + 1][k] = j;
                }
            }
        }
    }
    let mut best_id = 0;
    for i in 0..BUBEN {
        if dp[n - 1][i] < dp[n - 1][best_id] {
            best_id = i;
        }
    }

    let mut answers = vec![0.0; n];
    answers[n - 1] = angles[best_id];
    for i in (1..n).rev() {
        let prev_best_id = prev[i][best_id];
        answers[i - 1] = angles[prev_best_id];
        best_id = prev_best_id;
    }

    let mut change = std::f64::consts::PI;
    let mut iter = 0;

    while start.elapsed().as_millis() < 4500 && change > 1e-14 {
        iter += 1;
        change *= 0.99;
        // dbg!(iter, change);
        for i in 0..n {
            let prev = if i == 0 {
                None
            } else {
                Some(calc(a[i - 1], answers[i - 1], r))
            };
            let next = if i + 1 == n {
                None
            } else {
                Some(calc(a[i + 1], answers[i + 1], r))
            };
            let cost = |angle: f64| -> f64 {
                let mut res = 0.0;
                let p = calc(a[i], angle, r);
                if let Some(prev) = prev {
                    res += prev.dist(p);
                }
                if let Some(next) = next {
                    res += next.dist(p);
                }
                res
            };
            {
                let mut cur_ans = answers[i];
                let mut cur_cost = cost(cur_ans);
                for dir in (-1..=1).step_by(2) {
                    let expect = cur_ans + change * (dir as f64);
                    let next_cost = cost(expect);
                    if next_cost < cur_cost {
                        cur_ans = expect;
                        cur_cost = next_cost;
                    }
                }
                answers[i] = cur_ans;
            }
        }
    }

    let mut res = 0.0;
    for i in 0..(n - 1) {
        let p1 = calc(a[i], answers[i], r);
        let p2 = calc(a[i + 1], answers[i + 1], r);
        res += p1.dist(p2);
    }

    res
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();


    let n = sc.usize();
    let r = sc.next::<f64>();
    let mut a = vec![];
    for _ in 0..n {
        let x = sc.next::<f64>();
        let y = sc.next::<f64>();
        a.push(Point { x, y });
    }
    let res = solve(&a, r);
    writeln!(out, "{}", res).unwrap();
}
