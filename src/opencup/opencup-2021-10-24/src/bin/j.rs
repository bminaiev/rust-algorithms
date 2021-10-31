use std::io;
use std::io::Write;
use std::collections::HashSet;

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

struct Point {
    pos: i32,
    color: i32,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let mut a = vec![];
    for _ in 0..n {
        let pos = sc.i32();
        let color = sc.i32();
        a.push(Point { pos, color });
    }
    let mut bad = HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            if a[i].color == a[j].color {
                continue;
            }
            let dist = (a[i].pos - a[j].pos).abs();
            for div in 1..=dist {
                if dist % div == 0 {
                    bad.insert(div);
                    bad.insert(dist / div);
                }
                if div * div >= dist {
                    break;
                }
            }
        }
    }
    for size in 2..=n {
        let mut colors = vec![0; size];
        for p in a.iter() {
            let sz = size as i32;
            let index = ((p.pos % sz) + sz) % sz;
            let index = index as usize;
            if colors[index] == 0 {
                colors[index] = p.color;
            }
        }
        let mut all_known = true;
        for &c in colors.iter() {
            if c == 0 {
                all_known = false;
            }
        }
        if !all_known {
            continue;
        }
        for smaller in 1..size {
            if size % smaller == 0 {
                let mut ok = true;
                for id in 0..size {
                    if colors[id] != colors[id % smaller] {
                        ok = false;
                    }
                }
                if ok {
                    bad.insert(size as i32);
                }
            }
        }
    }
    let mut sum = 0i64;
    for &val in bad.iter() {
        sum += val as i64;
    }
    writeln!(out, "{} {}", bad.len(), sum).unwrap();
}
