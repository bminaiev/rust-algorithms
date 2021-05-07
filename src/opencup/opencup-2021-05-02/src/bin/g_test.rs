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

#[derive(Copy, Clone)]
struct Query {
    id: usize,
    r: usize,
}

struct Solver {
    a: Vec<i64>,
    b: Vec<i64>,
    start: usize,
}

impl Solver {
    fn create(n: usize) -> Self {
        let a = vec![0; n];
        let b = vec![0; n];
        Self { a, b, start: n }
    }

    fn xor(&mut self, from: usize, to: usize) {
        for x in self.a[from..to].iter_mut() {
            *x ^= 1;
        }
        self.start = from;
    }

    fn plus(&mut self) {
        for (a_it, b_it) in self.a[self.start..].iter().zip(self.b[self.start..].iter_mut()) {
            *b_it += *a_it
        }
    }

    fn get(&self, from: usize, to: usize) -> i64 {
        self.b[from..to].iter().sum()
    }
}

#[allow(dead_code)]
struct Random {
    state: usize
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

const MAX: usize = 100_000;

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let start = Instant::now();
    let n = MAX;
    let mut rnd = Random::new(787788);
    let a: Vec<_> = (0..MAX).map(|_| rnd.next_in_range(0, n)).collect();
    let m = MAX;
    let mut queries = vec![vec![]; n];
    for id in 0..m {
        let l = rnd.next_in_range(0, n);
        let r = rnd.next_in_range(0, n);
        let (l, r) = if l < r {
            (l, r)
        } else {
            (r, l)
        };
        queries[l].push(Query { id, r });
    }
    let mut last_time = vec![n; n + 1];
    let mut solver = Solver::create(n);
    let mut results = vec![0; m];
    for left in (0..n).rev() {
        solver.xor(left, last_time[a[left]]);
        solver.plus();
        for query in queries[left].iter() {
            results[query.id] = solver.get(left, query.r);
        }
        last_time[a[left]] = left;
    }
    // for &x in results.iter() {
    //     writeln!(out, "{}", x).unwrap();
    // }
    let elapsed = start.elapsed().as_millis();
    dbg!(elapsed);
}
