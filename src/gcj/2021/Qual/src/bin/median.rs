use std::io;
use std::io::{Write, BufWriter, StdoutLock};
use std::cmp::{min};

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

fn query(out: &mut BufWriter<StdoutLock>, sc: &mut Scanner, ids: &[usize]) -> usize {
    assert_ne!(ids[0], ids[1]);
    assert_ne!(ids[0], ids[2]);
    assert_ne!(ids[1], ids[2]);
    writeln!(out, "{} {} {}", ids[0], ids[1], ids[2]).unwrap();
    out.flush().unwrap();
    sc.usize()
}

fn solve_one_case(out: &mut BufWriter<StdoutLock>, sc: &mut Scanner, n: usize) {
    let mut perm = vec![1, 2];
    for new_val in 3..=n {
        let mut left = 0;
        let mut right = perm.len();
        while right - left > 0 {
            let len = right - left;
            let len3 = len / 3;
            let mut m1 = left + len3;
            let mut m2 = min(right - len3, right - 1);
            if m1 == m2 {
                if m2 == perm.len() - 1 {
                    m1 -= 1;
                } else {
                    m2 += 1;
                }
            }
            // dbg!(perm, m1, m2, new_val);
            assert_ne!(m1, m2);
            let med = query(out, sc, &[perm[m1], perm[m2], new_val]);
            if med == perm[m1] {
                right = m1;
            } else if med == perm[m2] {
                left = m2 + 1;
            } else {
                assert_eq!(med, new_val);
                left = m1 + 1;
                right = m2;
            }
        }
        perm.insert(left, new_val);
    }
    for &x in &perm {
        write!(out, "{} ", x).unwrap();
    }
    writeln!(out).unwrap();
    out.flush().unwrap();
    assert_eq!(sc.usize(), 1);
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let tc = sc.usize();
    let n = sc.usize();
    let _tot_queries = sc.usize();
    for _ in 0..tc {
        solve_one_case(&mut out, &mut sc, n);
    }
}
