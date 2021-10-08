use std::io;
use std::io::Write;
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


pub fn main() {
    let mut path = vec![0, 1, 2, 0];
    let mut n = 3;
    for _ in 0.. {
        let new_n = n + 2;
        dbg!(n, new_n);
        let mut new_path = vec![n, n + 1, path[0]];
        let max_chain_len = new_n - 3;
        let mut iter = 0;
        let mut want_id = 0;
        let mut last_seen = vec![0, 1];
        let mut used_n = vec![vec![false; n]; 2];
        used_n[1][path[0]] = true;
        while iter != path.len() {
            let next_pos = new_path.len();
            let prev_pos = last_seen[want_id];
            let dist = next_pos - prev_pos;
            let want_n = if want_id == 0  {n } else {n + 1};
            if dist > max_chain_len && !used_n[want_id][path[iter]] && !used_n[want_id][path[iter + 1]] {
                last_seen[want_id] = new_path.len();
                new_path.push(want_n);
                used_n[want_id][path[iter]] = true;
                used_n[want_id][path[iter + 1]] = true;
                want_id = 1 - want_id;
            }
            new_path.push(path[iter]);
            iter += 1;
        }

        path = new_path;
        n = new_n;
        assert_eq!(path.len(), n * (n - 1) / 2 + 1);
    }
}

pub fn main11() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for test in 0..tc {
        let n = sc.usize();
        let k = sc.usize();
        let lengths = sc.vec::<usize>(k);
        let mut iter = vec![0; n];
        let mut g = vec![vec![1; n]; n];
        for i in 0..n {
            g[i][i] = 0;
        }
        let mut stack = vec![];
        let mut res = vec![];
        stack.push(0);
        while stack.len() > 0 {
            let v = *stack.last().unwrap();
            while iter[v] != n && g[v][iter[v]] == 0 {
                iter[v] += 1;
            }
            if iter[v] == n {
                res.push(v);
                stack.pop();
            } else {
                stack.push(iter[v]);
                g[v][iter[v]] -= 1;
                g[iter[v]][v] -= 1;
            }
        }
        let mut tested = vec![vec![0; n]; n];
        for w in res.windows(2) {
            let x = min(w[0], w[1]);
            let y = max(w[0], w[1]);
            tested[x][y] += 1;
        }
        for i in 0..n {
            for j in i + 1..n {
                assert_eq!(tested[i][j], 1);
            }
        }
        writeln!(out, "Case #{}:", test + 1).unwrap();
        let mut cur_iter = 0;
        for &need_len in lengths.iter() {
            for i in 0..=need_len {
                write!(out, "{} ", res[cur_iter + i] + 1).unwrap();
            }
            writeln!(out).unwrap();
            cur_iter += need_len;
        }
    }
}
