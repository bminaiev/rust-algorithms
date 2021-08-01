use std::io;
use std::io::Write;
use std::cmp::{max, min};

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


fn solve(a: &[usize]) -> usize {
    let n = a.len();
    let mut dp = vec![vec![]; n + 1];
    let mut start_from = vec![vec![]; n];
    for i in (0..n).rev() {
        let cur = a[i];
        dp[cur].push(n + 1);
        dp[cur].push(cur);
        for prev in 0..cur {
            for was_len in 0..dp[prev].len() {
                while dp[cur].len() <= was_len + 1 {
                    dp[cur].push(0);
                }
                dp[cur][was_len + 1] = max(dp[cur][was_len + 1], dp[prev][was_len]);
            }
        }
        if i + 1 < n {
            start_from[i] = start_from[i + 1].clone();
            for len in 0..dp[cur].len() {
                while start_from[i].len() <= len {
                    start_from[i].push(0);
                }
                start_from[i][len] = max(start_from[i][len], dp[cur][len]);
            }
        }
    }
    let mut res = 0;
    let mut max_len = vec![n + 1; n + 1];
    max_len[0] = 0;
    for i in 0..n {
        let cur = a[i];
        let mut left = 0;
        let mut right = n + 1;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if max_len[mid] <= cur {
                left = mid;
            } else {
                right = mid;
            }
        }
        max_len[right] = min(max_len[right], cur);
        let mut more_left = 0;
        let mut more_right = n + 1;
        while more_right - more_left > 1 {
            let more_mid = (more_left + more_right) / 2;
            let from = i + more_mid - 1;
            let mut ok = false;
            if from < n {
                if start_from[from].len() > more_mid {
                    if start_from[from][more_mid] > cur {
                        ok = true;
                    }
                }
            }
            if ok {
                more_left = more_mid;
            } else {
                more_right = more_mid;
            }
        }
        res = max(res, right + more_left);
    }
    res
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let mut a = sc.vec::<usize>(n);
        let mut res = solve(&a);
        for i in 0..n {
            a[i] = n + 1 - a[i];
        }
        res = max(res, solve(&a));
        writeln!(out, "{}", res).unwrap();
    }
}
