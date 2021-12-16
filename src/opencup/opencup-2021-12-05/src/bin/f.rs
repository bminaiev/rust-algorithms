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

#[derive(Copy, Clone)]
struct Elem {
    len: usize,
    sum_even: i32,
    sum_odd: i32,
    can_join: bool,
}

fn cost(elems: &[Elem], b: i32) -> i32 {
    let mut res = 0;
    let mut cur_len = 0;
    for e in elems.iter() {
        if e.can_join {
            if cur_len % 2 == 0 {
                res += e.sum_even;
            } else {
                res += e.sum_odd;
            }
            cur_len += e.len;
        } else {
            let cnt = (e.sum_even + b - 1) / b;
            let full = (cnt - 1) * b;
            let more = e.sum_even - full;
            let mut even = (cnt) / 2 * b;
            let mut odd = full - even;
            if cnt % 2 == 0 {
                odd += more;
            } else {
                even += more;
            }
            if cur_len % 2 == 0 {
                res += even;
            } else {
                res += odd;
            }
            cur_len += cnt as usize;
        }
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
        let a = sc.vec::<i32>(n);
        let mut elems: Vec<_> = a
            .iter()
            .map(|&val| Elem {
                len: 1,
                sum_even: val,
                sum_odd: 0,
                can_join: false,
            })
            .collect();
        let max_a = *a.iter().max().unwrap();
        for b in 1..=max_a {
            let mut next_elems: Vec<Elem> = Vec::with_capacity(elems.len());
            for &e in elems.iter() {
                if !next_elems.is_empty()
                    && next_elems.last().unwrap().can_join
                    && (e.can_join || e.sum_even <= b)
                {
                    let mut prev = next_elems.pop().unwrap();
                    if prev.len % 2 == 0 {
                        prev.sum_even += e.sum_even;
                        prev.sum_odd += e.sum_odd;
                    } else {
                        prev.sum_even += e.sum_odd;
                        prev.sum_odd += e.sum_even;
                    }
                    prev.len += e.len;
                    next_elems.push(prev);
                } else {
                    if e.sum_even <= b {
                        next_elems.push(Elem {
                            sum_even: e.sum_even,
                            sum_odd: e.sum_odd,
                            len: e.len,
                            can_join: true,
                        })
                    } else {
                        next_elems.push(e);
                    }
                }
            }
            elems = next_elems;
            writeln!(out, "{}", cost(&elems, b)).unwrap();
        }
    }
}
