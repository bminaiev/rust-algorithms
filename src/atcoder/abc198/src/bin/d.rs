use std::io;
use std::io::{Write, BufWriter, StdoutLock};
use std::collections::BTreeMap;

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

fn convert_num(map: &BTreeMap<u8, i64>, s: &[u8]) -> Option<i64> {
    if map[&s[0]] == 0 {
        return None;
    }
    let mut res = 0i64;
    for x in s.iter() {
        res = res * 10 + (map[x] as i64);
    }
    Some(res)
}

fn find_sol(map: &mut BTreeMap<u8, i64>, all_digits: &Vec<u8>, pos: usize, used: &mut Vec<bool>,
            s1: &[u8], s2: &[u8], s3: &[u8], out: &mut BufWriter<StdoutLock>) -> bool {
    if pos == all_digits.len() {
        if let Some(s1) = convert_num(map, s1) {
            if let Some(s2) = convert_num(map, s2) {
                if let Some(s3) = convert_num(map, s3) {
                    if s1 + s2 == s3 {
                        writeln!(out, "{}\n{}\n{}", s1, s2, s3).unwrap();
                        return true;
                    }
                }
            }
        }
        return false;
    } else {
        let digit = all_digits[pos];
        if map.contains_key(&digit) {
            return find_sol(map, all_digits, pos + 1, used, s1, s2, s3, out);
        } else {
            for x in 0..used.len() {
                if used[x] {
                    continue;
                }
                used[x] = true;
                map.insert(all_digits[pos], x as i64);
                if find_sol(map, all_digits, pos + 1, used, s1, s2, s3, out) {
                    return true;
                }
                map.remove(&all_digits[pos]);
                used[x] = false;
            }
        }
    }
    return false;
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let s1 = sc.string();
    let s2 = sc.string();
    let s3 = sc.string();

    let mut all_digits = vec![];
    all_digits.append(&mut s1.clone());
    all_digits.append(&mut s2.clone());
    all_digits.append(&mut s3.clone());

    if !find_sol(&mut BTreeMap::new(), &all_digits, 0, &mut vec![false; 10], &s1, &s2, &s3, &mut out) {
        writeln!(out, "UNSOLVABLE").unwrap();
    }
}
