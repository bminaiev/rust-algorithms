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
struct Number {
    value: usize,
    pos: usize,
}

const MAX: usize = 5_000_001;

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();
    let n = sc.usize();
    let mut a = (0..n).map(|i| Number { value: sc.usize(), pos: i + 1 }).collect::<Vec<_>>();
    a.sort();
    let mut first_same = None;
    for i in 0..(n - 1) {
        if a[i].value == a[i + 1].value {
            first_same = Some(i);
            break;
        }
    }

    if let Some(first_same) = first_same {
        for i in 0..(n - 1) {
            if a[i].value == a[i + 1].value && i > first_same + 1 {
                let x = a[first_same].pos;
                let y = a[i].pos;
                let z = a[first_same + 1].pos;
                let w = a[i + 1].pos;
                writeln!(out, "YES").unwrap();
                writeln!(out, "{} {} {} {}", x, y, z, w).unwrap();
                return;
            }
        }
    }

    let mut seen = vec![None; MAX];
    for i in 0..n {
        if i > 0 && a[i].value == a[i - 1].value {
            continue;
        }
        for j in (i + 1)..n {
            if j > i + 1 && a[j].value == a[j - 1].value {
                continue;
            }
            let sum = a[i].value + a[j].value;
            if let Some((x, y)) = seen[sum] {
                writeln!(out, "YES").unwrap();
                writeln!(out, "{} {} {} {}", x, y, a[i].pos, a[j].pos).unwrap();
                return;
            } else {
                seen[sum] = Some((a[i].pos, a[j].pos));
            }
        }
    }

    writeln!(out, "NO").unwrap();
}
