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

    let n = sc.usize();
    let m = sc.usize();
    let k = sc.usize();
    if k % 2 == 1 {
        for _ in 0..n {
            for _ in 0..m {
                write!(out, "-1 ").unwrap();
            }
            writeln!(out).unwrap();
        }
        return;
    }
    let k = k / 2;
    let mut to_right = vec![];
    for _ in 0..n {
        to_right.push(sc.vec::<i32>(m - 1));
    }
    let mut to_down = vec![];
    for _ in 0..(n - 1) {
        to_down.push(sc.vec::<i32>(m));
    }
    let mut dp = vec![vec![vec![std::i32::MAX; m]; n]; k + 1];
    for i in 0..n {
        for j in 0..m {
            dp[0][i][j] = 0;
        }
    }
    for lvl in 0..k {
        for i in 0..n {
            for j in 0..m {
                if i + 1 < n {
                    dp[lvl + 1][i][j] = min(dp[lvl + 1][i][j], dp[lvl][i + 1][j] + to_down[i][j]);
                }
                if i > 0 {
                    dp[lvl + 1][i][j] = min(dp[lvl + 1][i][j], dp[lvl][i - 1][j] + to_down[i - 1][j]);
                }
                if j + 1 < m {
                    dp[lvl + 1][i][j] = min(dp[lvl + 1][i][j], dp[lvl][i][j + 1] + to_right[i][j]);
                }
                if j > 0 {
                    dp[lvl + 1][i][j] = min(dp[lvl + 1][i][j], dp[lvl][i][j - 1] + to_right[i][j - 1]);
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(out, "{} ", 2 * dp[k][i][j]).unwrap();
        }
        writeln!(out).unwrap();
    }
}
