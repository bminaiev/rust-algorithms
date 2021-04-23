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

    let n = sc.usize() * 2;
    let mut c = vec![vec![0; n + 1]; n + 1];
    c[0][0] = 1u64;
    for i in 1..c.len() {
        c[i][0] = 1;
        for j in 1..c[i].len() {
            c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
        }
    }
    let k = sc.usize();
    // dp[segs][len][total_sum]
    let mut dp = vec![vec![vec![0u64; k + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for cnt in 1..=n {
        let tot_sum = cnt * (cnt - 1) / 2;
        if tot_sum > k {
            break;
        }
        dp[0][cnt][tot_sum] = 1;
    }
    for total_sum in 0..=k {
        for len in 1..=n {
            for segs in 1..=n {
                for next_segs in 0..=n {
                    let used_len = segs * 2 + next_segs;
                    let points = segs + next_segs;
                    if used_len > len {
                        break;
                    }
                    let mut ways = c[next_segs + segs - 1][segs - 1];
                    let cur_sum = points * (points - 1) / 2;
                    if cur_sum > total_sum {
                        break;
                    }
                    let there_sum = total_sum - cur_sum;
                    ways *= dp[next_segs][len - used_len][there_sum];
                    dp[segs][len][total_sum] += ways;
                }
            }
        }
    }
    let mut res = 0u64;
    for segs in 1..=n {
        let points = segs + 1;
        let my_sum = points * (points - 1) / 2;
        if my_sum > k {
            break;
        }
        for segs_up in 0..=segs {
            let segs_down = segs - segs_up;
            for sum_up in 0..=(k - my_sum) {
                let sum_down = k - sum_up - my_sum;
                for len_up in 0..=n {
                    let len_down = n - len_up;
                    let mut ways = c[segs][segs_up];
                    ways *= dp[segs_up][len_up][sum_up];
                    ways *= dp[segs_down][len_down][sum_down];
                    if ways != 0 {
                        dbg!(segs_up, segs_down, sum_up, sum_down, ways);
                    }
                    res += ways;
                }
            }
        }
    }
    writeln!(out, "{}", res).unwrap();
}
