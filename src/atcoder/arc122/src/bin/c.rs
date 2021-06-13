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
    fn u64(&mut self) -> u64 {
        self.next::<u64>()
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
struct Random {
    state: usize
}

impl Random {
    fn next(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    #[allow(dead_code)]
    fn next_in_range(&mut self, from: usize, to: usize) -> usize {
        assert!(from < to);
        from + self.next() % (to - from)
    }

    #[allow(dead_code)]
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }

    #[allow(dead_code)]
    fn new(seed: usize) -> Self {
        assert_ne!(seed, 0);
        Self {
            state: seed,
        }
    }

    fn next_u64(&mut self) -> u64 {
        let a = self.next_in_range(0, std::u32::MAX as usize) as u64;
        let b = self.next_in_range(0, std::u32::MAX as usize) as u64;
        (a << 32) | b
    }
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

fn ok(a: u64, b: u64, more_steps: u64) -> Option<Vec<usize>> {
    if a == 1 && b == 1 {
        return Some(vec![]);
    }
    if more_steps == 0 {
        return None;
    }
    if a > b {
        return match ok(a - b, b, more_steps - 1) {
            None => None,
            Some(mut v) => {
                v.push(4);
                Some(v)
            }
        };
    } else {
        return match ok(a, b - a, more_steps - 1) {
            None => None,
            Some(mut v) => {
                v.push(3);
                Some(v)
            }
        };
    }
}

// fn _old_test() {
//     let MAX = 1e18 as u64;
//     for _ in 0..100 {
//         let n = rnd.next_u64() % MAX;
//         let mut found = false;
//         for it in 0..10000 {
//             let a = rnd.next_u64() % n;
//             let b = n - a;
//             if ok(a, b, 120) {
//                 found = true;
//                 writeln!(out, "wow {}!", it).unwrap();
//                 break;
//             }
//         }
//         if !found {
//             writeln!(out, "oops :(").unwrap();
//         }
//     }
// }

fn check(ops: &Vec<usize>, n: u64) {
    let mut x = 0;
    let mut y = 0;
    for op in ops.iter() {
        match op {
            1 => x = x + 1,
            2 => y = y + 1,
            3 => x = x + y,
            4 => y = x + y,
            _ => unreachable!(),
        };
    }
    assert_eq!(x, n);
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();


    let mut rnd = Random::new(123);
    let n = sc.u64();
    if n == 1 {
        writeln!(out, "1").unwrap();
        writeln!(out, "1").unwrap();
        return;
    }
    loop {
        let a = rnd.next_u64() % n;
        let b = n - a;
        match ok(a, b, 127) {
            None => continue,
            Some(mut ops) => {
                ops.reverse();
                ops.push(3);
                ops.insert(0, 1);
                ops.insert(0, 2);
                check(&ops, n);
                writeln!(out, "{}", ops.len()).unwrap();
                for op in ops {
                    writeln!(out, "{}", op).unwrap();
                }
                break;
            }
        }
    }
}
