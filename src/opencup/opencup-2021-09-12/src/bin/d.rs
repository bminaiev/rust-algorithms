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


#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Query {
    left_div_sqrt: usize,
    n_minus_right: usize,
    left: usize,
    right: usize,
    id: usize,
}

struct TwoConnectedList {
    next: Vec<usize>,
    prev: Vec<usize>,
    position: Vec<i64>,
    cur_ans: i64,
    prev_cur_ans: i64,
    removed: Vec<usize>,
}

impl TwoConnectedList {
    fn new(n: usize, position: Vec<i64>) -> Self {
        let mut res = Self { next: vec![0; n], prev: vec![0; n], position, cur_ans: 0, prev_cur_ans: 0, removed: vec![] };
        res.init();
        res
    }

    fn get(&self, i: usize, j: usize) -> i64 {
        (self.position[i] - self.position[j]).abs()
    }

    fn init(&mut self) {
        self.cur_ans = 0;
        let n = self.next.len();
        for i in 0..n - 1 {
            self.next[i] = i + 1;
            self.prev[i + 1] = i;
            self.cur_ans += self.get(i, i + 1);
        }
        self.next[n - 1] = n - 1;
        self.prev[0] = 0;
    }

    fn remove(&mut self, value: usize, save: bool) {
        let pr = self.prev[value];
        let ne = self.next[value];
        self.cur_ans -= self.get(value, pr);
        self.cur_ans -= self.get(value, ne);
        if save {
            self.removed.push(value);
        }
        if pr != value && ne != value {
            self.cur_ans += self.get(ne, pr);
            self.next[pr] = ne;
            self.prev[ne] = pr;
        } else {
            if pr != value {
                self.next[pr] = pr;
            }
            if ne != value {
                self.prev[ne] = ne;
            }
        }
    }

    fn save_ans(&mut self) {
        self.prev_cur_ans = self.cur_ans;
    }

    fn calc_ans(&mut self) -> i64 {
        let ans = self.cur_ans;
        for &removed in self.removed.iter().rev() {
            if self.prev[removed] != removed {
                self.next[self.prev[removed]] = removed;
            }
            if self.next[removed] != removed {
                self.prev[self.next[removed]] = removed;
            }
        }
        self.removed.clear();
        self.cur_ans = self.prev_cur_ans;
        ans
    }
}

const BUBEN: usize = 128;

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let q = sc.usize();
    let mut position = vec![0; n];
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = sc.usize() - 1;
        position[p[i]] = i as i64;
    }
    let mut queries = Vec::with_capacity(q);
    for id in 0..q {
        let left = sc.usize() - 1;
        let right = sc.usize() - 1;
        queries.push(Query { left, right, left_div_sqrt: left / BUBEN, n_minus_right: n - right, id });
    }
    queries.sort();
    let mut q_it = 0;
    let mut ans = vec![0; q];
    for start in (0..n).step_by(BUBEN) {
        let mut two_connected_list = TwoConnectedList::new(n, position.clone());
        for i in 0..start {
            two_connected_list.remove(p[i], false);
        }
        let mut r = n - 1;
        while q_it != queries.len() && queries[q_it].left_div_sqrt == start / BUBEN {
            while r != queries[q_it].right {
                two_connected_list.remove(p[r], false);
                r -= 1;
            }
            two_connected_list.save_ans();
            for i in start..queries[q_it].left {
                two_connected_list.remove(p[i], true);
            }
            ans[queries[q_it].id] = two_connected_list.calc_ans();
            q_it += 1;
        }
    }
    for a in ans.iter() {
        writeln!(out, "{}", a).unwrap();
    }
}

