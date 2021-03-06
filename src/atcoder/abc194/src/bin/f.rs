#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

// Scanner code is copied from Russell Emerine's solution
// http://codeforces.com/contest/1477/submission/105755265
impl Scanner {
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
    fn next_string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}

const MOD: i32 = 1_000_000_007;

fn add(x: i32, y: i32) -> i32 {
    let res = x + y;
    if res >= MOD {
        res - MOD
    } else {
        res
    }
}

fn mul(x: i32, y: i32) -> i32 {
    ((x as i64) * (y as i64) % (MOD as i64)) as i32
}


const D: usize = 16;

fn solve(digits: &Vec<u8>, k: usize) -> i32 {
    let n = digits.len();


    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for len in 1..=n {
        for more in 0..=k {
            let used = k - more;
            if more > 0 {
                dp[len][more] = add(dp[len][more], mul(dp[len - 1][more - 1], (D - used) as i32));
            }
            dp[len][more] = add(dp[len][more], mul(dp[len - 1][more], used as i32));
        }
    }


    let mut res = 0;
    for len in 1..n {
        let mut ways = (D - 1) as i32;
        ways = mul(ways, dp[len - 1][k - 1]);
        res = add(res, ways);
    }
    // eprintln!("for k = {}, cur res = {}", k, res);
    let mut seen_digits = vec![false; D];
    let mut seen_uniq_digits = 0;
    for same_prefix in 0..n {
        let max_digit = digits[same_prefix] as usize;
        for d in 0..max_digit {
            if same_prefix == 0 && d == 0 {
                continue;
            }
            let mut total_uniq_digits = seen_uniq_digits;
            if !seen_digits[d] {
                total_uniq_digits += 1;
            }
            if total_uniq_digits <= k {
                let ways = dp[n - same_prefix - 1][k - total_uniq_digits];
                res = add(res, ways);
            }
        }
        if !seen_digits[max_digit] {
            seen_digits[max_digit] = true;
            seen_uniq_digits += 1;
        }
        if seen_uniq_digits > k {
            break;
        }
        // eprintln!("same_prefix = {}, res = {}", same_prefix, res);
    }
    if seen_uniq_digits == k {
        res = add(res, 1);
    }
    res
}

pub fn main() {
    let mut scanner = Scanner::default();
    let s = scanner.next_string();
    let k: usize = scanner.next();
    let mut digits = vec![0; s.len()];
    for i in 0..digits.len() {
        let c = s[i];
        digits[i] = if c >= b'A' && c <= b'F' {
            c - b'A' + 10
        } else {
            c - b'0'
        }
    }
    let res = solve(&digits, k);
    println!("{}", res);
}