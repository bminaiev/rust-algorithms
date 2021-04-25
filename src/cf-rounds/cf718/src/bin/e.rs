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

fn solve(a: &[i64]) -> u64 {
    let n = a.len();
    if n == 1 {
        return 1;
    }
    let mut pref_sum = vec![0; n + 1];
    let mut pref_sum_2 = vec![0; n + 1];
    for i in 0..n {
        pref_sum[i + 1] = pref_sum[i] + a[i];
        pref_sum_2[i + 1] = a[i];
        if i > 0 {
            pref_sum_2[i + 1] += pref_sum_2[i - 1];
        }
    }
    let mut res = 1; // no black at all
    for start_blue in 0..=1 {
        for end_black in 0..=1 {
            let from = start_blue;
            let to = n - end_black;
            // [from, to)
            let mut start_balance = 0;
            if end_black == 1 {
                start_balance -= a[n - 1];
            }
            if start_blue == 1 {
                start_balance += a[0];
            }
            let total_len = to - from;
            if total_len == 0 {
                if start_balance > 0 {
                    res += 1;
                }
            } else {
                let mut now_balance = start_balance;
                let max_black_len = if end_black == 1 {
                    total_len + 1
                } else {
                    total_len
                };
                if start_blue == 1 && end_black == 1 {
                    for first_blue_len in 1..=total_len {
                        let cur_pos = from + first_blue_len - 1;
                        let check_balance = now_balance + pref_sum[cur_pos + 1] - pref_sum[from] - (pref_sum[to] - pref_sum[cur_pos + 1]);
                        if check_balance > 0 {
                            res += 1;
                        }
                    }
                }
                for black_len in 1..max_black_len {
                    let cur_pos = from + black_len - 1;
                    now_balance -= a[cur_pos];
                    let balance_if_only_blue = now_balance + pref_sum[to] - pref_sum[cur_pos + 1];
                    if balance_if_only_blue <= 0 {
                        break;
                    }
                    // max_jumps * 2 < to - cur_pos
                    // overflow?
                    if cur_pos + 2 >= to - 1 {
                        res += 1;
                        continue;
                    }
                    let max_jumps = (to - cur_pos - 2) / 2;
                    assert!(cur_pos + max_jumps * 2 < to - 1);
                    assert!(cur_pos + (max_jumps + 1) * 2 >= to - 1);
                    let mut left = 0;
                    let mut right = max_jumps + 1;
                    while right - left > 1 {
                        let mid = (left + right) / 2;
                        let last_jump_pos = cur_pos + mid * 2;
                        let additional_sum = pref_sum_2[last_jump_pos + 1] - pref_sum_2[cur_pos + 1];
                        let checked_balance = balance_if_only_blue - additional_sum * 2;
                        if checked_balance > 0 {
                            left = mid;
                        } else {
                            right = mid;
                        }
                    }
                    res += right as u64;
                }
            }
        }
    }
    return res;
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let a = sc.vec::<i64>(n);
        let res = solve(&a);
        writeln!(out, "{}", (res % 998244353)).unwrap();
    }
}
