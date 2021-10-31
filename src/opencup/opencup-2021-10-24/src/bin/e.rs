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

#[derive(Copy, Clone)]
struct Coin {
    cost: i64,
    weight: i64,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Res {
    min_weight: i64,
    max_weight: i64,
}

const INF_RES: Res = Res { min_weight: i64::MAX / 3, max_weight: i64::MIN / 3 };

fn mul(a: &Vec<Res>, b: &Vec<Res>) -> Vec<Res> {
    let mut r = vec![INF_RES; a.len()];
    for (i, &ai) in a.iter().enumerate() {
        if ai == INF_RES {
            continue;
        }
        for (j, &bj) in b.iter().enumerate() {
            if i + j >= r.len() {
                break;
            }
            if bj == INF_RES {
                continue;
            }
            r[i + j].min_weight = min(r[i + j].min_weight, ai.min_weight + bj.min_weight);
            r[i + j].max_weight = max(r[i + j].max_weight, ai.max_weight + bj.max_weight);
        }
    }
    r
}

fn solve(k: usize, coins: &[Coin], mut need_more: i64) -> Option<Res> {
    let mut dp = vec![INF_RES; k + 1];
    dp[0] = Res { min_weight: 0, max_weight: 0 };

    let mut cur_value_cost = vec![INF_RES; k + 1];


    for (coin_index, coin) in coins.iter().enumerate() {
        if need_more % coin.cost != 0 {
            return None;
        }
        let mut need_here = if coin_index == coins.len() - 1 {
            need_more / coin.cost
        } else {
            let next_cost = coins[coin_index + 1].cost;
            (need_more % next_cost) / coin.cost
        };
        if need_here > k as i64 {
            return None;
        }
        need_more -= coin.cost * need_here;
        let mut need_next = if coin_index == coins.len() - 1 {
            0
        } else {
            coins[coin_index + 1].cost / coin.cost
        };
        if need_next > k as i64 {
            need_next = 0;
        }
        let mut next_value_cost = vec![INF_RES; k + 1];
        next_value_cost[0] = Res { min_weight: 0, max_weight: 0 };

        cur_value_cost[1] = Res { min_weight: min(cur_value_cost[1].min_weight, coin.weight), max_weight: max(cur_value_cost[1].max_weight, coin.weight) };

        for index in 0.. {
            if need_here & (1 << index) != 0 {
                dp = mul(&dp, &cur_value_cost);
                need_here ^= 1 << index;
            }
            if need_next & (1 << index) != 0 {
                next_value_cost = mul(&next_value_cost, &cur_value_cost);
                need_next ^= 1 << index;
            }
            if need_here == 0 && need_next == 0 {
                break;
            }
            cur_value_cost = mul(&cur_value_cost, &cur_value_cost);
        }

        if need_next == 0 {
            next_value_cost[0] = INF_RES;
        }
        cur_value_cost = next_value_cost;
    }

    if dp[k] == INF_RES {
        None
    } else {
        Some(dp[k])
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let k = sc.usize();
    let need_more = sc.i64();
    let mut coins = vec![];
    for _ in 0..n {
        let cost = sc.i64();
        let weight = sc.i64();
        coins.push(Coin { cost, weight });
    }
    match solve(k, &coins, need_more) {
        | None => writeln!(out, "-1").unwrap(),
        | Some(res) => writeln!(out, "{} {}", res.min_weight, res.max_weight).unwrap()
    }
}
