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

fn first_pos(a: &[u32], bit: i32) -> usize {
    let mut pos_of_bit_set = 0;
    while pos_of_bit_set != a.len() && (a[pos_of_bit_set] >> bit) & 1 == 0 {
        pos_of_bit_set += 1;
    }
    return pos_of_bit_set;
}

fn find_closest(left: &[u32], right: &[u32], bit: i32) -> u32 {
    if left.is_empty() || right.is_empty() {
        return std::u32::MAX;
    }
    if bit == -1 {
        return left[0] ^ right[0];
    }
    let p1 = first_pos(left, bit);
    let p2 = first_pos(right, bit);
    let mut res = std::u32::MAX;
    if p1 > 0 && p2 > 0 {
        res = min(res, find_closest(&left[..p1], &right[..p2], bit - 1));
    }
    if p1 < left.len() && p2 < right.len() {
        res = min(res, find_closest(&left[p1..], &right[p2..], bit - 1));
    }
    if res != std::u32::MAX {
        return res;
    }
    res = min(res, find_closest(&left[..p1], &right[p2..], bit - 1));
    res = min(res, find_closest(&left[p1..], &right[..p2], bit - 1));
    return res;
}

fn solve(a: &[u32], bit: i32) -> u32 {
    if bit == -1 {
        return 0;
    }
    let pos_of_bit_set = first_pos(a, bit);
    if pos_of_bit_set == a.len() || pos_of_bit_set == 0 {
        return solve(a, bit - 1);
    }
    let left = pos_of_bit_set;
    let right = a.len() - left;
    if left % 2 == 0 && right % 2 == 0 {
        return max(solve(&a[..pos_of_bit_set], bit - 1), solve(&a[pos_of_bit_set..], bit - 1));
    } else {
        return find_closest(&a[..pos_of_bit_set], &a[pos_of_bit_set..], bit - 1);
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize() * 2;
    let mut a = sc.vec::<u32>(n);
    a.sort();
    writeln!(out, "{}", solve(&a, 30)).unwrap();
}
