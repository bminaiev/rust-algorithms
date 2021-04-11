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

struct BigNum {
    digits: Vec<i32>
}

impl BigNum {
    fn new(value: i64) -> Self {
        let mut digits = vec![];
        let mut cur_value = value;
        while cur_value != 0 {
            digits.push((cur_value % 10) as i32);
            cur_value /= 10;
        }
        Self { digits }
    }

    fn add_one(&self) -> Self {
        let mut more = 1;
        let mut iter = 0;
        let mut digits = vec![];
        while iter < self.digits.len() || more != 0 {
            let cur_sum = self.digits.get(iter).unwrap_or(&0) + more;
            digits.push(cur_sum % 10);
            more = cur_sum / 10;
            iter += 1;
        }
        Self { digits }
    }

    fn add_zeros(&self, zeros: usize) -> Self {
        let mut digits = vec![0; zeros];
        digits.append(&mut self.digits.clone());
        Self { digits }
    }

    fn to_string(&self) -> String {
        let mut res = self.digits.iter().map(|&x| x as u8 + b'0').collect::<Vec<u8>>();
        res.reverse();
        String::from_utf8(res).unwrap()
    }
}

struct Result {
    next_at_least: BigNum,
    add_to_result: usize,
}

fn solve(at_least: &BigNum, value: &BigNum) -> Result {
    let at_least_str = at_least.to_string();
    let value_str = value.to_string();
    if value_str.len() > at_least_str.len() {
        return Result { next_at_least: value.add_one(), add_to_result: 0 };
    }
    if at_least_str < value_str {
        let len_diff = at_least_str.len() - value_str.len();
        return Result { next_at_least: (value.add_zeros(len_diff).add_one()), add_to_result: len_diff };
    }
    let at_least_pref = at_least_str.split_at(value_str.len()).0;
    if at_least_pref == value_str {
        let len_diff = at_least_str.len() - value_str.len();
        return Result { next_at_least: at_least.add_one(), add_to_result: len_diff };
    }
    assert!(at_least_pref > value_str.as_str());
    let len_diff = at_least_str.len() - value_str.len() + 1;
    return Result { next_at_least: (value.add_zeros(len_diff).add_one()), add_to_result: len_diff };
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let tc = sc.usize();

    for t in 0..tc {
        let n = sc.usize();
        let a = sc.vec::<i64>(n);
        let mut at_least = BigNum::new(1);
        let mut result = 0;
        for &val in &a {
            let res = solve(&at_least, &BigNum::new(val));
            result += res.add_to_result;
            at_least = res.next_at_least;
        }
        writeln!(out, "Case #{}: {}", t + 1, result).unwrap();
    }
}
