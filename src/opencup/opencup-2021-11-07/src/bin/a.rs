use std::io;
use std::io::Write;
use std::cmp::max;

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

fn solve2(a: i64, b: i64, r: i64, buben: i64, buben2 : i64) -> i64 {
    let mut d = ((a * a + b * b) as f64).sqrt();
    if d == 0.0 {
        d = 1.0;
    }
    let approx_a = (((a * r) as f64) / d) as i64;
    let approx_b = (((b * r) as f64) / d) as i64;
    let mut res = 0;
    for da in -buben..buben {
        let my_a = approx_a + da;
        let mut approx_b = ((r * r  - my_a * my_a) as f64);
        if approx_b < 0.0 {
            continue;
        }
        let mut approx_b = approx_b.sqrt() as i64;
        if b < 0 {
            approx_b *= -1i64;
        }
        for db in -buben2..=buben2 {
            let my_b = approx_b + db;
            if my_a * my_a + my_b * my_b <= r * r {
                let force = a * my_a + b * my_b;
                res = max(res, force);
            }
        }
    }
    res
}

fn solve(a: i64, b: i64, r: i64, buben: i64) -> i64 {
    let mut d = ((a * a + b * b) as f64).sqrt();
    if d == 0.0 {
        d = 1.0;
    }
    let approx_a = (((a * r) as f64) / d) as i64;
    let approx_b = (((b * r) as f64) / d) as i64;
    let mut res = 0;
    for da in -buben..buben {
        for db in -buben..buben {
            let my_a = approx_a + da;
            let my_b = approx_b + db;
            if my_a * my_a + my_b * my_b <= r * r {
                let force = a * my_a + b * my_b;
                res = max(res, force);
            }
        }
    }
    res
}

fn stress() {
    const MAX: usize = 1_000_000_000;
    for t in 5.. {
        let mut rnd = Random::new(t);
        dbg!(t);

        let a = (rnd.next_in_range(0, MAX) as i64) - (MAX / 2) as i64;
        let b = (rnd.next_in_range(0, MAX) as i64) - (MAX / 2) as i64;
        let r = (rnd.next_in_range(0, MAX) as i64) + 1;

        let s1 = solve2(a, b, r, 10000, 5);
        let s2 = solve2(a, b, r, 100000, 1);
        if s1 != s2 {
            dbg!(a, b, r, s1, s2);
            assert_eq!(s1, s2);
        }
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    // stress();

    let tc = sc.usize();
    for _ in 0..tc {
        let a = sc.i64();
        let b = sc.i64();
        let r = sc.i64();
        const BUBEN: i64 = 50000;
        let res = solve2(a, b, r, BUBEN, 1);
        writeln!(out, "{}", res).unwrap();
    }
}
