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
struct Op {
    from: usize,
    to: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let l = sc.usize();
    let mut more_time = sc.i64();
    let mut ops = vec![];
    let mut by_from = vec![vec![]; n];
    for index in 0..l {
        let to = sc.usize() - 1;
        let from = sc.usize() - 1;
        ops.push(Op { from, to });
        by_from[from].push(index);
    }
    let mut state = sc.vec::<i32>(n);
    let mut good = BTreeSet::new();
    for i in 0..l {
        good.insert(i);
    }
    let mut cur_pos = 0;
    loop {
        let mut next_pos = *good.range(cur_pos..).next().unwrap_or(&l);
        more_time -= (next_pos - cur_pos) as i64;
        if next_pos == l {
            next_pos = *good.range(0..).next().unwrap_or(&l);
            more_time -= next_pos as i64;
            if next_pos == l {
                break;
            }
        }
        if more_time <= 0 {
            break;
        }
        cur_pos = next_pos;
        assert_ne!(cur_pos, l);
        good.remove(&cur_pos);
        let op = ops[cur_pos];
        let potential = state[op.to] | state[op.from];
        if potential != state[op.to] {
            state[op.to] = potential;
            for &check in by_from[op.to].iter() {
                good.insert(check);
            }
        }
    }
    for x in state.iter() {
        write!(out, "{} ", x).unwrap();
    }
}
