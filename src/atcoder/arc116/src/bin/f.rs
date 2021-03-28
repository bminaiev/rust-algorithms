use std::io;
use std::io::Write;
use std::cmp::{min, max};

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


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let k = sc.usize();
    let mut res = 0i64;
    let mut pairs = vec![];
    let mut triples = vec![];
    let mut fours = vec![];

    let mut fours_old = vec![];
    let mut res_old = 0;

    let mut res_new = 0;
    for _ in 0..k {
        let n = sc.usize();
        let a = sc.vec::<i64>(n);
        if n == 1 {
            res += a[0];
        } else {
            if n % 2 == 0 {
                let m1 = n / 2;
                let m2 = m1 - 1;
                let v1 = a[m1];
                let v2 = a[m2];
                if n == 2 {
                    res += min(v1, v2);
                    pairs.push(max(v1, v2) - min(v1, v2));
                } else {
                    let v3 = a[m1 - 2];
                    let v4 = a[m1 + 1];
                    let go_left = min(v2, max(v3, v1));
                    let go_right = min(v1, max(v4, v2));

                    res_old += min(v1, v2);
                    fours_old.push(max(v1, v2) - min(v1, v2));

                    res_new += min(go_left, go_right);
                    fours.push(max(go_left, go_right) - min(go_left, go_right));
                }
            } else {
                let m = n / 2;
                let v_center = a[m];
                let v_left = a[m - 1];
                let v_right = a[m + 1];
                if v_center <= v_left && v_center <= v_right {
                    let mid = min(v_left, v_right);
                    res += v_center;
                    triples.push(mid - v_center);
                } else if v_center >= v_left && v_center >= v_right {
                    let mid = max(v_left, v_right);
                    res += mid;
                    triples.push(v_center - mid);
                } else {
                    res += v_center;
                    triples.push(0);
                }
            }
        }
    }

    let cnt_even = (pairs.len() + fours.len()) % 2;
    if cnt_even % 2 == 1 {
        for x in fours_old {
            pairs.push(x);
        }
    }
    pairs.sort();
    triples.sort();
    let start = (pairs.len() + 1) % 2;
    for pos in (start..pairs.len()).step_by(2) {
        res += pairs[pos];
    }
    fours.sort();
    if cnt_even == 0 {
        res += res_new;
        let start = pairs.len() % 2;
        for pos in (start..fours.len()).step_by(2) {
            res += fours[pos];
        }
    } else {
        res += res_old;
        res += triples.iter().sum::<i64>();
    }
    writeln!(out, "{}", res).unwrap();
}
