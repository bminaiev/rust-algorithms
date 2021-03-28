use std::io;
use std::io::{Write, BufWriter, StdoutLock};
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

fn solve(n: usize, c: usize, out: &mut BufWriter<StdoutLock>) {
    if c < n - 1 || c > (n + 2) * (n - 1) / 2 {
        writeln!(out, " IMPOSSIBLE").unwrap();
        return;
    }
    let mut perm: Vec<_> = (0..n).collect();
    let mut more = c - (n - 1);
    let mut res = vec![n; n];
    for i in 0..n {
        let pos = min(more, n - i - 1);
        more -= pos;
        res[perm[pos + i]] = i + 1;
        let mut left_it = i;
        let mut right_it = i + pos;
        while left_it < right_it {
            perm.swap(left_it, right_it);
            left_it += 1;
            right_it -= 1;
        }
    }
    assert_eq!(more, 0);
    for &x in res.iter() {
        write!(out, " {}", x).unwrap();
    }
    writeln!(out).unwrap();
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let tc = sc.usize();
    for t in 0..tc {
        let n = sc.usize();
        let c = sc.usize();
        write!(out, "Case #{}:", t + 1).unwrap();
        solve(n, c, &mut out);
    }
}
