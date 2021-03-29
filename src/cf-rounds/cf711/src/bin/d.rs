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
    let mut ops = vec![std::usize::MAX; m + 1];
    ops[0] = 0;
    let mut res = vec![std::usize::MAX; m + 1];
    for iter in 0..n {
        let op_type = sc.usize();
        let x = (sc.i64() as f64) / 100000.0;
        let y = sc.usize();
        if op_type == 1 {
            for cur in 0..m {
                if ops[cur] != std::usize::MAX {
                    let next = (cur as f64 + x).ceil() as usize;
                    if next < ops.len() {
                        ops[next] = min(ops[next], ops[cur] + 1);
                    }
                }
            }
        } else {
            for cur in 0..m {
                if ops[cur] != std::usize::MAX {
                    let next = (cur as f64 * x).ceil() as u64;
                    if next < ops.len() as u64 {
                        let next = next as usize;
                        ops[next] = min(ops[next], ops[cur] + 1);
                    }
                }
            }
        }
        for cur in 0..=m {
            if ops[cur] <= y {
                ops[cur] = 0;
                if res[cur] == std::usize::MAX {
                    res[cur] = iter;
                }
            } else {
                ops[cur] = std::usize::MAX;
            }
        }
    }
    for x in 1..=m {
        if res[x] == std::usize::MAX {
            write!(out, "-1 ").unwrap();
        } else {
            write!(out, "{} ", res[x] + 1).unwrap();
        }
    }
}
