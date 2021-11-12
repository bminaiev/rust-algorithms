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


fn get_day_id(s: String) -> usize {
    let tmp = vec!["Monday".to_string(), "Tuesday".to_string(), "Wednesday".to_string(), "Thursday".to_string(), "Friday".to_string(), "Saturday".to_string(), "Sunday".to_string()];
    for i in 0..7 {
        if tmp[i] == s {
            return i;
        }
    }
    assert!(false);
    return 0;
}
fn get_day_name(x: usize) -> String {
    let tmp = vec!["Monday".to_string(), "Tuesday".to_string(), "Wednesday".to_string(), "Thursday".to_string(), "Friday".to_string(), "Saturday".to_string(), "Sunday".to_string()];
    return tmp[x].clone();
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();
    let t = sc.i64();
    for _ in 0..t {
        let n = sc.usize();
        let m = sc.usize();
        let mut init_state = vec![0; m];
        for i in 0..m {
            let s = sc.next::<String>();
            init_state[i] = get_day_id(s);
        }
        let mut rev:Vec<i32> = vec![0; 7];
        for i in 1..7 {
            for j in 1..7 {
                if (i * j) % 7 == 1 {
                    rev[i] = j as i32;
                }
            }
        }


        let mut a:Vec<Vec<i32>> = vec![vec![0; m]; n];

        for i in 0..n {
            let k = sc.usize();
            for _ in 0..k {
                let planet_id = sc.usize();
                a[i][planet_id - 1] = 1;
            }
        }
        let mut found = false;
        for i in 0..m {
            let mut pos:i32 = -1;
            for j in i..n {
                if a[j][i] != 0 {
                    pos = j as i32;
                    break;
                }
            }
            if pos == -1 {
                init_state[i] = (init_state[i] + 1) % 7;
                found = true;
                write!(out, "YES ").unwrap();
                for i in 0..m {
                    write!(out, "{} ", get_day_name(init_state[i])).unwrap();
                }
                writeln!(out, "").unwrap();
                break;
            }
            else {
                a.swap(i, pos as usize);
                // dbg!(a[i][i], i, pos as usize);
                assert!(1 <= a[i][i] && a[i][i] < 7);
                let dv = rev[a[i][i] as usize];
                for j in 0..m {
                    a[i][j] = (a[i][j] * dv) % 7;
                }
                for j in 0..n {
                    if j == i {
                        continue;
                    }
                    let mult = a[j][i];
                    for k in 0..m {
                        a[j][k] = (a[j][k] - mult * a[i][k] % 7 + 7) % 7;
                    }
                    assert!(a[j][i] == 0);
                }
            }
        }
        if !found {
            writeln!(out, "NO").unwrap();
        }
    }
}