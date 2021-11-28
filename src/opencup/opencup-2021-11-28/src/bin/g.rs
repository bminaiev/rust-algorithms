use std::cmp::min;
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
        Self {
            buffer: vec![],
            input_source: InputSource::Stdin,
        }
    }

    #[allow(dead_code)]
    fn new_file(filename: &str) -> Self {
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
        lines.reverse();
        Self {
            buffer: vec![],
            input_source: InputSource::FromFile(lines),
        }
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
            InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            InputSource::FromFile(lines) => match lines.pop() {
                Some(line) => input = line,
                None => return false,
            },
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

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Elem {
    value: usize,
    a_pos: Option<usize>,
}

#[allow(dead_code)]
pub struct Fenwick {
    values: Vec<usize>,
}

impl Fenwick {
    #[allow(dead_code)]
    fn get_min(&self, mut pos: usize) -> usize {
        let mut res = usize::MAX;
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
    fn update(&mut self, mut pos: usize, change: usize) {
        while pos < self.values.len() {
            self.values[pos] = min(self.values[pos], change);
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new(n: usize) -> Self {
        let values = vec![usize::MAX; n];
        Fenwick { values }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct BalancePair {
    balance: usize,
    value: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let a = sc.vec::<i32>(n);
    let b = sc.vec::<i32>(m);
    let mut all = a.clone();
    all.append(&mut b.clone());
    all.sort();
    let mut a_id: Vec<_> = (0..n).map(|i| all.binary_search(&a[i]).unwrap()).collect();
    a_id.push(n + m);
    let b_id: Vec<_> = (0..m).map(|i| all.binary_search(&b[i]).unwrap()).collect();
    let mut elems = vec![];
    for i in 0..a_id.len() {
        elems.push(Elem {
            value: a_id[i],
            a_pos: Some(i),
        })
    }
    for i in 0..b.len() {
        elems.push(Elem {
            value: b_id[i],
            a_pos: None,
        });
    }
    elems.sort();

    let mut cnt_less_b = 0;

    let mut balance = vec![0; n + 1];

    for elem in elems.iter() {
        if let Some(pos) = elem.a_pos {
            balance[pos] = cnt_less_b + n - pos;
        } else {
            cnt_less_b += 1;
        }
    }

    let mut balance_pairs = vec![];
    for pos in 0..=n {
        balance_pairs.push(BalancePair {
            balance: balance[pos],
            value: a_id[pos],
        });
    }
    balance_pairs.sort();

    let mut fenw = Fenwick::new(balance_pairs.len());

    let mut dp = vec![usize::MAX; n + 1];
    for pos in 0..=n {
        if balance[pos] >= n {
            dp[pos] = pos;
        }
        let fenw_pos = match balance_pairs.binary_search(&BalancePair {
            balance: balance[pos] + 1,
            value: a_id[pos],
        }) {
            Ok(_) => unreachable!(),
            Err(pos) => pos,
        };
        if fenw_pos != 0 {
            let from_prev_fenw = fenw.get_min(fenw_pos - 1);
            if from_prev_fenw != usize::MAX {
                dp[pos] = min(dp[pos], from_prev_fenw + pos - 1 - n);
            }
        }
        if dp[pos] != usize::MAX {
            let pair_pos = balance_pairs
                .binary_search(&BalancePair {
                    balance: balance[pos],
                    value: a_id[pos],
                })
                .unwrap();
            fenw.update(pair_pos, dp[pos] + n - pos);
        }
    }
    let res = if dp[n] == usize::MAX {
        -1
    } else {
        dp[n] as i64
    };
    writeln!(out, "{}", res).unwrap();
}
