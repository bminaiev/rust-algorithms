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


struct Point {
    x: i64,
    y: i64,
}

fn vect_mul(a: &Point, b: &Point, c: &Point) -> i32 {
    let res = (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y);
    if res > 0 { 1 } else if res < 0 { -1 } else { 0 }
}

#[derive(Clone)]
struct BitSet(Vec<u64>);

impl BitSet {
    const BITS: usize = 6;
    const M: usize = 1 << Self::BITS;

    fn create(n: usize) -> Self {
        Self(vec![0; (n + Self::M - 1) / Self::M])
    }

    fn set(&mut self, pos: usize) {
        self.0[pos >> Self::BITS] |= 1 << (pos & (Self::M - 1));
    }

    fn xor(&mut self, pos: usize) {
        self.0[pos >> Self::BITS] ^= 1 << (pos & (Self::M - 1));
    }

    fn clear(&mut self) {
        for x in self.0.iter_mut() {
            *x = 0;
        }
    }

    fn or_eq(&mut self, other: &Self) {
        for (x, y) in self.0.iter_mut().zip(other.0.iter()) {
            *x |= *y;
        }
    }

    fn and_eq(&mut self, other: &Self) {
        for (x, y) in self.0.iter_mut().zip(other.0.iter()) {
            *x &= *y;
        }
    }

    fn count(&self) -> usize {
        let mut res = 0;
        for x in self.0.iter() {
            res += x.count_ones() as usize;
        }
        return res;
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let k = sc.usize();
    let mut a = vec![];
    for _ in 0..k {
        let x = sc.i64();
        let y = sc.i64();
        a.push(Point { x, y });
    }
    let n = sc.usize();
    let mut b = vec![];
    let mut alive = BitSet::create(n);
    for i in 0..n {
        let x = sc.i64();
        let y = sc.i64();
        let is_alive = sc.i32();
        b.push(Point { x, y });
        if is_alive == 1 {
            alive.set(i);
        }
    }
    let mut to_right = vec![vec![BitSet::create(n); k]; k];
    for i in 0..k {
        for j in i + 1..k {
            for t in 0..n {
                if vect_mul(&a[i], &a[j], &b[t]) < 0 {
                    to_right[i][j].set(t);
                } else {
                    to_right[j][i].set(t);
                }
            }
        }
    }
    let q = sc.usize();
    let mut p = [0usize; 5];
    let mut tmp_bs = BitSet::create(n);
    for _ in 0..q {
        let type_ = sc.usize();
        match type_ {
            1 => {
                let who = sc.usize() - 1;
                alive.xor(who);
            }
            2 => {
                for x in p.iter_mut() {
                    *x = sc.usize() - 1;
                }
                tmp_bs.clear();
                tmp_bs.or_eq(&alive);
                tmp_bs.and_eq(&to_right[p[0]][p[2]]);
                tmp_bs.and_eq(&to_right[p[1]][p[3]]);
                tmp_bs.and_eq(&to_right[p[2]][p[4]]);
                tmp_bs.and_eq(&to_right[p[3]][p[0]]);
                tmp_bs.and_eq(&to_right[p[4]][p[1]]);
                writeln!(out, "{}", tmp_bs.count()).unwrap();
            }
            _ => unreachable!(),
        }
    }
}
