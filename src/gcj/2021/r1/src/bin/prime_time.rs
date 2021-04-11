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

    for t in 0..tc {
        let distinct_primes = sc.usize();
        let mut primes = vec![0; distinct_primes];
        let mut num_primes = vec![0; distinct_primes];
        for i in 0..distinct_primes {
            primes[i] = sc.i64();
            num_primes[i] = sc.i64();
        }
        let total_sum = primes.iter().zip(num_primes.iter()).map(|(p, cnt)| p * cnt).sum::<i64>();
        const MAGIC: i64 = 20_000;
        let mut found = false;
        for expected_sum in (total_sum - MAGIC..total_sum).rev() {
            if expected_sum <= 0 {
                break;
            }
            let mut now_sum = expected_sum;
            let mut real_sum = 0;
            let mut ok = true;
            for i in 0..distinct_primes {
                let mut cur_pw = 0;
                while now_sum % primes[i] == 0 {
                    now_sum /= primes[i];
                    cur_pw += 1;
                    real_sum += primes[i];
                }
                if cur_pw > num_primes[i] {
                    ok = false;
                    break;
                }
            }
            if ok && real_sum + expected_sum == total_sum && now_sum == 1 {
                writeln!(out, "Case #{}: {}", t + 1, expected_sum).unwrap();
                found = true;
                break;
            }
        }
        if !found {
            writeln!(out, "Case #{}: 0", t + 1).unwrap();
        }
    }
}
