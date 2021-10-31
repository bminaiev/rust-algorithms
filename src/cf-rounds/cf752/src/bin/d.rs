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


fn gcd(x: usize, y: usize) -> usize {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn calc(l: usize, r: usize) -> i64 {
    let mut res = 0;
    for i in l..=r {
        for j in i + 1..=r {
            if gcd(i, j) >= l {
                res += 1;
            }
        }
    }
    res
}

fn phi(mut n: usize) -> usize {
    let mut result = n;
    for i in 2..=n {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    const K: usize = 23;
    const N: usize = 300;

    let mut all_phi = vec![0; N];
    let mut pref_sum_phi = vec![0; N];

    let mut all_divs = vec![vec![]; N];
    for x in 1..N {
        all_phi[x] = phi(x) as i64;
        if x != 1 {
            pref_sum_phi[x] = pref_sum_phi[x - 1] + (all_phi[x]);
        }

        for y in 1..=x {
            if y * y > x {
                break;
            }
            if x % y == 0 {
                all_divs[x].push(y);
                if y * y < x {
                    all_divs[x].push(x / y);
                }
            }
        }
    }

    let mut dp = vec![vec![0; N]; K];

    // let mut cache = vec![vec![None; N]; N];

    for n in 1..N {
        let nn = n as i64;
        dp[1][n] = nn * (nn - 1) / 2;
    }

    for k in 2..K {
        let mut prev = k - 1;

        let mut cur_last_seg_cost = 0i64;

        for n in k..N {
            for &g in all_divs[n].iter() {
                if g <= prev || g == n {
                    continue;
                }
                let mult = n / g;
                cur_last_seg_cost += all_phi[mult];
            }


            loop {
                let our_cost = dp[k - 1][prev] + cur_last_seg_cost;
                if our_cost == 0 || prev + 1 == n {
                    break;
                }
                let max_mult = n / (prev + 1);
                let sub = pref_sum_phi[max_mult];
                let next_cost = dp[k - 1][prev + 1] + cur_last_seg_cost - sub;
                if next_cost > our_cost {
                    break;
                }
                cur_last_seg_cost -= sub;
                prev += 1;
            }

            assert!(cur_last_seg_cost >= 0);

            assert_eq!(cur_last_seg_cost, calc(prev + 1, n));

            dp[k][n] = dp[k - 1][prev] + cur_last_seg_cost;


            let mut best_index = 0;
            for check_prev in 1..n {
                let cur_res = dp[k - 1][check_prev] + calc(check_prev + 1, n);
                if cur_res < dp[k][n] {
                    dbg!(cur_res, n, k, prev, check_prev);

                    for check_prev2 in 0..n {
                        let zz = dp[k - 1][check_prev2] + calc(check_prev2 + 1, n);
                        dbg!(check_prev2, zz);
                    }

                    assert!(false);
                    best_index = check_prev;
                }
                if cur_res < 10000 {
                    // write!(out, "{} ", cur_res).unwrap();
                }
            }
        }
    }
    let get_ans = |n: usize, k: usize| -> i64 {
        if k >= dp.len() {
            n as i64
        } else {
            dp[k][n] + n as i64
        }
    };


    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let k = sc.usize();
        writeln!(out, "{}", get_ans(n, k)).unwrap();
    }
}
