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


const MOD: usize = 998244353;

fn add(x: usize, y: usize) -> usize {
    let r = x + y;
    if r >= MOD {
        r - MOD
    } else {
        r
    }
}

fn mul(x: usize, y: usize) -> usize {
    ((x as i64) * (y as i64) % (MOD as i64)) as usize
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let m = sc.usize();
    let mut pw = vec![1; n + 1];
    for i in 1..=n {
        pw[i] = mul(pw[i - 1], m);
    }
    let mut res = 0;
    for h in 1..=m {
        let mut seen_h = 0;
        let mut not_seen_h_yet = 1;
        let bigger = m - h;
        for len in 1..=n {
            let next_seen_h = add(mul(seen_h, 1 + bigger), not_seen_h_yet);
            let next_not_seen_h_yet = mul(not_seen_h_yet, bigger);
            seen_h = next_seen_h;
            not_seen_h_yet = next_not_seen_h_yet;
            if len == n {
                res = add(res, seen_h);
            } else {
                let one_side = mul(h - 1, seen_h);
                res = add(res, mul(add(one_side, one_side), pw[n - len - 1]));
                if len + 2 <= n {
                    let ways = n - len - 1;
                    let two_side = mul(h - 1, one_side);
                    res = add(res, mul(mul(ways, two_side), pw[n - len - 2]));
                }
            }
        }
    }
    writeln!(out, "{}", res).unwrap();
}
