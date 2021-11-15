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

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let mut a = vec![];
        for _ in 0..n {
            a.push(sc.usize() - 1);
        }
        let mut pos = vec![0; n];
        let mut b = vec![];
        for i in 0..n {
            let val = sc.usize() - 1;
            b.push(val);
            pos[val] = i;
        }

        let mut to_check = vec![];
        let mut in_queue = vec![true; n];
        for i in 1..n {
            to_check.push(i - 1);
        }

        let mut res = vec![];

        while let Some(pos_to_check) = to_check.pop() {
            in_queue[pos_to_check] = false;
            let v1 = a[pos_to_check];
            let v2 = a[pos_to_check + 1];
            let pos_of_v1 = pos[v1];
            let pos_of_v2 = pos[v2];
            if pos_of_v1 > pos_of_v2 && v1 > v2 {
                res.push(pos_to_check);
                a.swap(pos_to_check, pos_to_check + 1);
                dbg!(a);
                if pos_to_check > 0 && !in_queue[pos_to_check - 1] {
                    in_queue[pos_to_check - 1] = true;
                    to_check.push(pos_to_check - 1);
                }
                if pos_to_check + 2 < n && !in_queue[pos_to_check + 1] {
                    in_queue[pos_to_check + 1] = true;
                    to_check.push(pos_to_check + 1);
                }
            }
        }

        if a == b {
            writeln!(out, "{}", res.len()).unwrap();
            for pos in res.iter() {
                writeln!(out, "{} {}", pos + 1, pos + 2).unwrap();
            }
        } else {
            writeln!(out, "{}", -1).unwrap();
        }
    }
}
