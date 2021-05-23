use std::io;
use std::io::Write;
use std::collections::BTreeSet;

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

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            match &mut self.input_source {
                | InputSource::Stdin => { std::io::stdin().read_line(&mut input).expect("Failed read"); }
                | InputSource::FromFile(lines) => {
                    let line = lines.pop().unwrap();
                    input = line;
                }
            }

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

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Person {
    sum: i64,
    plus: i64,
    minus: i64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Top {
    value: i64,
    id: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let plus = sc.vec::<i64>(n);
    let minus = sc.vec::<i64>(n);
    let mut persons: Vec<_> = (0..n).map(|i| Person { sum: plus[i] + minus[i], plus: plus[i], minus: minus[i] }).collect();
    persons.sort();
    let mut left = 0;
    let mut right = n;
    let mut sum_add = vec![0; n];
    let mut sum_rem = vec![0; n];
    while right - left > 1 {
        let mid = (left + right) / 2;
        let mut ok = false;
        {
            let mut set = BTreeSet::new();
            let mut cur_sum = 0;
            for pos in 0..n {
                let val = Top { id: pos, value: persons[pos].minus };
                if set.len() < mid {
                    cur_sum += val.value;
                    set.insert(val);
                } else {
                    let last = (*set.iter().next_back().unwrap()).clone();
                    if last.value > val.value {
                        cur_sum += val.value - last.value;
                        set.remove(&last);
                        set.insert(val);
                    }
                }
                if set.len() == mid {
                    sum_rem[pos] = cur_sum;
                } else {
                    sum_rem[pos] = std::i64::MAX;
                }
            }
        }
        {
            let mut set = BTreeSet::new();
            let mut cur_sum = 0;
            for pos in (0..n).rev() {
                let val = Top { id: pos, value: -persons[pos].plus };
                if set.len() < mid {
                    cur_sum += val.value;
                    set.insert(val);
                } else {
                    let last = (*set.iter().next_back().unwrap()).clone();
                    if last.value > val.value {
                        cur_sum += val.value - last.value;
                        set.remove(&last);
                        set.insert(val);
                    }
                }
                if set.len() == mid {
                    sum_add[pos] = cur_sum;
                } else {
                    sum_add[pos] = std::i64::MAX;
                }
            }
        }
        for pos in 0..n {
            if pos + 1 == n {
                break;
            }
            let rem = sum_rem[pos];
            let add = sum_add[pos + 1];
            if rem == std::i64::MAX || add == std::i64::MAX {
                continue;
            }
            if -rem - add >= 0 {
                ok = true;
                break;
            }
        }
        if ok {
            left = mid;
        } else {
            right = mid;
        }
    }
    writeln!(out, "{}", left).unwrap();
}
