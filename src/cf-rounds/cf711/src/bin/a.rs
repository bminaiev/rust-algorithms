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


fn gcd(x: i64, y: i64) -> i64 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn sum_digits(n: i64) -> i64 {
    if n < 10 {
        n
    } else {
        n % 10 + sum_digits(n / 10)
    }
}

fn ok_number(n: i64) -> bool {
    gcd(n, sum_digits(n)) > 1
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let tc = sc.usize();
    for _ in 0..tc {
        let mut n = sc.i64();
        while !ok_number(n) {
            n += 1;
        }
        writeln!(out, "{}", n).unwrap();
    }
}
