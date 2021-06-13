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

fn gen_number(v: &[usize]) -> i64 {
    let mut res = 0;
    for x in v.iter() {
        res = res * 10 + (*x as i64);
    }
    res
}

fn greedy(new_cnt: &mut Vec<usize>, big_number: &mut Vec<usize>, small_number: &mut Vec<usize>, big_len: usize) {
    assert!(big_number.len() >= small_number.len());
    if big_number.len() == small_number.len() {
        for (x,y) in big_number.iter().zip(small_number.iter()) {
            assert!(*x >= *y);
            if x > y {
                break;
            }
        }
    }
    for v in 0..10 {
        while new_cnt[v] > 0 && big_number.len() < big_len {
            new_cnt[v] -= 1;
            big_number.push(v);
        }
    }
    for v in (0..10).rev() {
        while new_cnt[v] > 0 {
            new_cnt[v] -= 1;
            small_number.push(v);
        }
    }
}

fn solve(cnt: &[usize]) -> i64 {
    let mut tot_cnt = 0;
    for x in cnt.iter() {
        tot_cnt += *x;
    }
    if tot_cnt % 2 == 0 {
        let mut res = std::i64::MAX;
        let len = tot_cnt / 2;
        for small_digit in 1..10 {
            for big_digit in small_digit..10 {
                if cnt[small_digit] == 0 {
                    continue;
                }
                if cnt[big_digit] == 0 || (cnt[big_digit] == 1 && big_digit == small_digit) {
                    continue;
                }
                let mut new_cnt: Vec<_> = cnt.iter().cloned().collect();
                new_cnt[small_digit] -= 1;
                new_cnt[big_digit] -= 1;
                let mut big_number = vec![];
                let mut small_number = vec![];
                big_number.push(big_digit);
                small_number.push(small_digit);
                if small_digit < big_digit {
                    greedy(&mut new_cnt, &mut big_number, &mut small_number, len);
                    assert_eq!(small_number.len(), len);
                    res = min(res, gen_number(&big_number) - gen_number(&small_number));
                } else {
                    for v in new_cnt.iter_mut() {
                        while *v > 1 {
                            *v -= 2;
                            big_number.push(0);
                            small_number.push(0);
                        }
                    }
                    if big_number.len() == len {
                        assert_eq!(small_number.len(), len);
                        res = min(res, gen_number(&big_number) - gen_number(&small_number));
                    }
                    for small2 in 0..10 {
                        for big2 in small2 + 1..10 {
                            if new_cnt[small2] == 0 || new_cnt[big2] == 0 {
                                continue;
                            }
                            let mut new_cnt2: Vec<_> = new_cnt.iter().cloned().collect();
                            new_cnt2[small2] -= 1;
                            new_cnt2[big2] -= 1;
                            let mut big_number = big_number.clone();
                            let mut small_number = small_number.clone();
                            big_number.push(big2);
                            small_number.push(small2);
                            greedy(&mut new_cnt2, &mut big_number, &mut small_number, len);
                            assert_eq!(small_number.len(), len);
                            res = min(res, gen_number(&big_number) - gen_number(&small_number));
                        }
                    }
                }
            }
        }
        return res;
    } else {
        let small_len = tot_cnt / 2;
        let mut big_number = vec![];
        let mut new_cnt: Vec<_> = cnt.iter().cloned().collect();
        for v in 1..10 {
            if new_cnt[v] > 0 {
                new_cnt[v] -= 1;
                big_number.push(v);
                break;
            }
        }
        let mut small_number = vec![];
        greedy(&mut new_cnt, &mut big_number, &mut small_number, small_len + 1);
        assert_eq!(small_number.len(), small_len);
        return gen_number(&big_number) - gen_number(&small_number);
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for t in 0..tc {
        let s = sc.string();
        let mut cnt = vec![0; 10];
        for c in s.iter() {
            let digit = c - b'0';
            cnt[digit as usize] += 1;
        }
        let res = solve(&cnt);
        writeln!(out, "Case #{}: {}", t + 1, res).unwrap();
    }
}
