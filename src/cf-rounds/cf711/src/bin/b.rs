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


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let w = sc.i32();
        let weights: Vec<_> = (0..22).map(|x| (1 << x)).collect();
        let mut cnt = vec![0; weights.len()];
        for _ in 0..n {
            let w = sc.i32();
            let pos = weights.iter().enumerate().find(|(_, &val)| val == w).unwrap().0;
            cnt[pos] += 1;
        }
        let mut res = 0;
        loop {
            let mut here = w;
            for w in (0..weights.len()).rev() {
                while cnt[w] > 0 && here >= weights[w] {
                    here -= weights[w];
                    cnt[w] -= 1;
                }
            }
            if here == w {
                break;
            } else {
                res += 1;
            }
        }
        writeln!(out, "{}", res).unwrap();
    }
}
