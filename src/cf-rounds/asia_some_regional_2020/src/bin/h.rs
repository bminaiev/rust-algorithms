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

#[allow(dead_code)]
pub struct Fenwick {
    values: Vec<i64>
}

impl Fenwick {
    #[allow(dead_code)]
    fn get_min(&self, mut pos: usize) -> i64 {
        pos = self.values.len() - 1 - pos;
        let mut res = std::i64::MAX;
        loop {
            res = min(res, self.values[pos]);
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    #[allow(dead_code)]
    fn change(&mut self, mut pos: usize, change: i64) {
        pos = self.values.len() - 1 - pos;
        while pos < self.values.len() {
            self.values[pos] = min(self.values[pos], change);
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new(n: usize) -> Self {
        let values = vec![std::i64::MAX; n];
        Fenwick { values }
    }
}


/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

struct Card {
    days: i32,
    rides: usize,
    cost: i64,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let r = sc.i64();
    let mut cards = vec![];
    for _ in 0..n {
        let days = sc.i32();
        let rides = sc.usize();
        let cost = sc.i64();
        cards.push(Card { days, rides, cost });
    }
    cards.push(Card { days: std::i32::MAX / 2, rides: 1, cost: r });
    let mut need = vec![];
    for _ in 0..m {
        let day = sc.i32();
        let cnt = sc.usize();
        for _ in 0..cnt {
            need.push(day);
        }
    }
    need.sort();
    let mut dp = vec![std::i64::MAX; need.len() + 1];
    let mut fenw = Fenwick::new(need.len() + 1);
    dp[0] = 0;
    fenw.change(0, 0);
    let mut iters = vec![0; cards.len()];
    for i in 0..dp.len() {
        let cur_cost = fenw.get_min(i);
        assert_ne!(cur_cost, std::i64::MAX);
        for j in 0..cards.len() {
            while iters[j] != need.len() && need[i] + cards[j].days - 1 >= need[iters[j]] && iters[j] - i < cards[j].rides {
                iters[j] += 1;
            }
            if cur_cost + cards[j].cost < dp[iters[j]] {
                dp[iters[j]] = min(dp[iters[j]], cur_cost + cards[j].cost);
                fenw.change(iters[j], dp[iters[j]]);
            }
        }
    }
    writeln!(out, "{}", dp.last().unwrap()).unwrap();
}
