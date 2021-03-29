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
struct Pair {
    score: i32,
    first: usize,
    second: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let cnt_in = sc.vec::<i32>(n);
    let mut pairs = vec![];
    for i in 0..n {
        for j in i + 1..n {
            pairs.push(Pair { first: i, second: j, score: (cnt_in[i] - cnt_in[j]).abs() })
        }
    }
    pairs.sort();
    pairs.reverse();
    for pair in pairs.iter() {
        let (f, s) = if cnt_in[pair.first] > cnt_in[pair.second] {
            (pair.first + 1, pair.second + 1)
            // probably possible to get from second to first..
        } else {
            (pair.second + 1, pair.first + 1)
        };
        writeln!(out, "? {} {}", f, s).unwrap();
        out.flush().unwrap();
        if sc.next::<String>() == "Yes" {
            writeln!(out, "! {} {}", f, s).unwrap();
            return;
        }
    }
    writeln!(out, "! 0 0").unwrap();
}
