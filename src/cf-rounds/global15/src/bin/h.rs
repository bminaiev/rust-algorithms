use std::io;
use std::io::{Write, BufWriter, StdoutLock};
use std::cmp::{min, max};

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
struct Point {
    x: usize,
    y: usize,
}

const MAX_X: usize = 200;

fn ask(x_val: usize, x_mod: usize, y_val: usize, y_mod: usize, sc: &mut Scanner, out: &mut BufWriter<StdoutLock>) -> usize {
    let mut pts = vec![];
    for x in 1..=MAX_X {
        for y in 1..=MAX_X {
            if x % x_mod == x_val && y % y_mod == y_val {
                pts.push(Point { x, y });
            }
        }
    }
    if pts.len() == 0 {
        return 0;
    }
    writeln!(out, "? {}", pts.len()).unwrap();
    for p in pts.iter() {
        write!(out, "{} {} ", p.x, p.y).unwrap();
    }
    writeln!(out).unwrap();
    out.flush().unwrap();
    sc.usize()
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();
    let full = ask(0, 1, 0, 1, &mut sc, &mut out);
    if full % 3 == 0 {
        let part = ask(0, 2, 0, 1, &mut sc, &mut out);
        assert_ne!(part, full);
        let cnt1 = if part * 2 < full { full - part * 2 } else { part * 2 - full };
        assert_eq!(full % cnt1, 0);
        let cnt2 = full / cnt1;
        writeln!(out, "! {}", 2 * (cnt1 + cnt2 - 2)).unwrap();
    } else {
        let part1 = ask(0, 3, 0, 1, &mut sc, &mut out);
        let part2 = ask(1, 3, 0, 1, &mut sc, &mut out);
        if part1 + part2 == full {
            assert_eq!(part1, part2);
            writeln!(out, "! {}", 2 * (part1)).unwrap();
        } else {
            let part3 = full - part1 - part2;
            if part1 == 0 || part2 == 0 {
                writeln!(out, "! {}", 2 * (part3)).unwrap();
            } else {
                let min_p = min(min(part1, part2), part3);
                let max_p = max(max(part1, part2), part3);
                assert_ne!(min_p, max_p);
                let cnt1 = max_p - min_p;
                assert_eq!(full % cnt1, 0);
                let cnt2 = full / cnt1;
                writeln!(out, "! {}", 2 * (cnt1 + cnt2 - 2)).unwrap();
            }
        }
    }
}
