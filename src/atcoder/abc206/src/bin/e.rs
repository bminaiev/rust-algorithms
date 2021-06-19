use std::io;
use std::io::Write;
use std::cmp::max;

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

enum InputSource {
    Stdin,
    FromFile(Vec<String>),
}

struct Scanner {
    buffer: Vec<String>,
    input_source: InputSource,
}


impl Scanner {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { buffer: vec![], input_source: InputSource::Stdin }
    }

    #[allow(dead_code)]
    fn new_file(filename: &str) -> Self {
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
        lines.reverse();
        Self { buffer: vec![], input_source: InputSource::FromFile(lines) }
    }


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

    fn parse_next_line(&mut self) -> bool {
        let mut input = String::new();
        match &mut self.input_source {
            | InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            | InputSource::FromFile(lines) => {
                match lines.pop() {
                    Some(line) => input = line,
                    None => return false,
                }
            }
        }

        self.buffer = input.split_whitespace().rev().map(String::from).collect();
        return true;
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }

            self.parse_next_line();
        }
    }

    #[allow(dead_code)]
    fn has_more_elements(&mut self) -> bool {
        loop {
            if !self.buffer.is_empty() {
                return true;
            }
            if !self.parse_next_line() {
                return false;
            }
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
    let mut sc = Scanner::new();

    const M: usize = 1_000_005;
    let mut some_prime = vec![0; M];
    for x in 2..M {
        if some_prime[x] != 0 {
            continue;
        }
        for y in (x * 2..M).step_by(x) {
            some_prime[y] = x;
        }
    }

    let left = sc.usize();
    let right = sc.usize();

    let mut res = 0;
    for val in 2..M {
        let mut cnt_primes = 0;
        let mut ok = true;
        let mut cur_val = val;
        while cur_val != 1 {
            let p = some_prime[cur_val];
            if p == 0 {
                cnt_primes += 1;
                break;
            } else {
                let mut times = 0;
                while cur_val % p == 0 {
                    cur_val /= p;
                    times += 1;
                }
                if times > 1 {
                    ok = false;
                } else {
                    cnt_primes += 1;
                }
            }
        }
        if ok {
            let max = right / val;
            let min = (left - 1) / val;
            if max > min {
                let add = (max - min) as i64;
                let add = add * (add - 1) / 2;
                if cnt_primes % 2 == 1 {
                    res += add;
                } else {
                    res -= add;
                }
            }
        }
    }
    for x in max(2, left)..=right {
        for _ in (x * 2..=right).step_by(x) {
            res -= 1;
        }
    }
    writeln!(out, "{}", res * 2).unwrap();
}
