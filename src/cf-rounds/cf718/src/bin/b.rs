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


#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Road {
    len: i64,
    i: usize,
    j: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let m = sc.usize();
        let mut bs = vec![];
        let mut roads = vec![];
        for i in 0..n {
            bs.push(sc.vec::<i64>(m));
            for j in 0..m {
                roads.push(Road { i, j, len: bs[i][j] });
            }
        }
        roads.sort();
        let UNUSED: usize = m;
        let mut who = vec![vec![UNUSED; m]; n];
        for it in 0..m {
            let road = &roads[it];
            who[road.i][road.j] = it;
        }
        for i in 0..n {
            let mut assigned = vec![false; m];
            for j in 0..m {
                if who[i][j] != UNUSED {
                    assigned[who[i][j]] = true;
                }
            }
            let mut it = 0;
            for j in 0..m {
                if assigned[j] {
                    continue;
                }
                while who[i][it] != UNUSED {
                    it += 1;
                }
                who[i][it] = j;
            }
        }
        for i in 0..n {
            let mut res = vec![0; m];
            for j in 0..m {
                res[who[i][j]] = bs[i][j];
            }
            for j in 0..m {
                write!(out, "{} ", res[j]).unwrap();
            }
            writeln!(out).unwrap();
        }
    }
}
