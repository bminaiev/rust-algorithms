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

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            match &mut self.input_source {
                | InputSource::Stdin => { std::io::stdin().read_line(&mut input).expect("Failed read"); }
                | InputSource::FromFile(lines) => {
                    let line = lines.pop().unwrap();
                    input = line;
                }
            }

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

#[derive(Default, Clone)]
struct Solver {
    min: u32,
    max: u32,
    exist_center: bool,
    exist_min_1: bool,
    exist_max_1: bool,
}

impl Solver {
    fn clear(&mut self) {
        self.min = std::u32::MAX;
        self.max = 0;
        self.exist_center = false;
        self.exist_max_1 = false;
        self.exist_min_1 = false;
    }

    fn add_pos(&mut self, new_pos: u32) {
        if new_pos == self.min + 1 {
            self.exist_min_1 = true;
        }
        if new_pos + 1 == self.max {
            self.exist_max_1 = true;
        }
        if new_pos > self.min + 1 && new_pos + 1 < self.max {
            self.exist_center = true;
        }
    }

    fn get_ans(&self, row_len: u32) -> u32 {
        if self.exist_center {
            return self.max - self.min + 1 + 3 * (row_len - 1);
        }
        0
    }

    fn add(&mut self, line: u32, row_len: u32) -> u32 {
        if self.max == 0 {
            self.min = line;
            self.max = line;
            return 0;
        }
        let t1 = if self.exist_min_1 { self.min + 1 } else { line };
        let t2 = if self.exist_center { self.min + 2 } else { line };
        let t3 = if self.exist_max_1 { self.max - 1 } else { line };
        let t4 = self.min;
        let t5 = self.max;
        self.min = min(self.min, line);
        self.max = max(self.max, line);
        self.exist_min_1 = false;
        self.exist_max_1 = false;
        self.exist_center = false;
        self.add_pos(t1);
        self.add_pos(t2);
        self.add_pos(t3);
        self.add_pos(t4);
        self.add_pos(t5);
        self.add_pos(line);
        self.get_ans(row_len)
    }
}

enum RowsLen {
    Short(Vec<u8>),
    Long(Vec<u32>),
}

impl RowsLen {
    fn create(n: usize, m: usize) -> Self {
        if m < 255 {
            Self::Short(vec![0; n])
        } else {
            Self::Long(vec![0; n])
        }
    }

    fn set_zero(&mut self, pos: usize) {
        match self {
            Self::Short(a) => a[pos] = 0,
            Self::Long(a) => a[pos] = 0,
        }
    }

    fn add_one(&mut self, pos: usize) {
        match self {
            Self::Short(a) => a[pos] += 1,
            Self::Long(a) => a[pos] += 1,
        }
    }

    fn get(&self, pos: usize) -> u32 {
        match self {
            Self::Short(a) => a[pos] as u32,
            Self::Long(a) => a[pos],
        }
    }
}

struct Field {
    flatten: Vec<u8>,
    m: usize,
}

impl Field {
    fn create(n: usize, m: usize) -> Self {
        let flatten = vec![0; n * m];
        Self { flatten, m }
    }

    fn pos(&self, r: usize, c: usize) -> usize {
        r * self.m + c
    }

    fn set(&mut self, row: usize, bytes: &[u8]) {
        for (col, &ch) in bytes.iter().enumerate().take(self.m) {
            let p = self.pos(row, col);
            assert!(p < self.flatten.len());
            self.flatten[p] = ch;
        }
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        self.flatten[self.pos(row, col)]
    }
}

pub fn main() {
    let mut sc = Scanner::new();
    let n = sc.usize();
    let m = sc.usize();
    let mut a = Field::create(n, m);
    let mut b = vec![vec![0]; 1_000_000];
    for x in 0..b.len() {
        for y in 0..b[x].len() {
            b[x][y] = 123u8;
        }
    }
    for row in 0..n {
        let mut input = String::with_capacity(m);
        std::io::stdin().read_line(&mut input).expect("Failed read");
        a.set(row, &input.into_bytes());
    }
    let mut res = 0;
    let mut rows_sorted: Vec<u32> = (0..n as u32).collect();
    let mut rows_len = RowsLen::create(n, m);
    let mut solvers = vec![Solver::default(); (n + 6) / 6];
    let mut idx = vec![0u32; n];
    for c in (0..m).rev() {
        for r in 0..n {
            if a.get(r, c) == b'#' {
                rows_len.set_zero(r);
            } else {
                rows_len.add_one(r);
            }
        }
        let mut n_sz = 0;
        idx.clear();
        for i in 0..n {
            let r = rows_sorted[i];
            if rows_len.get(r as usize) == 0 {
                idx.push(r);
            } else {
                rows_sorted[n_sz] = r;
                n_sz += 1;
            }
        }
        for &r in idx.iter() {
            rows_sorted[n_sz] = r;
            n_sz += 1;
        }
        assert_eq!(n_sz, n);
        idx.resize(n, 0);
        const M: u32 = std::u32::MAX;
        {
            let mut solver_id = 0;
            let mut i = 0;
            while i != n {
                if rows_len.get(i) == 0 {
                    idx[i] = M;
                    i += 1;
                    continue;
                }
                let mut j = i;
                while j != n && rows_len.get(j) != 0 {
                    j += 1;
                }
                let len = j - i;
                if len < 5 {
                    while i < j {
                        idx[i] = M;
                        i += 1;
                    }
                } else {
                    while i < j {
                        idx[i] = solver_id;
                        i += 1;
                    }
                    solvers[solver_id as usize].clear();
                    solver_id += 1;
                }
            }
            assert!(solver_id <= solvers.len() as u32);
        }
        // dbg!("column", c);
        for &r in rows_sorted.iter() {
            let idx = idx[r as usize];
            if rows_len.get(r as usize) < 2 {
                break;
            }
            if idx == M {
                continue;
            }
            let cur_ans = solvers[idx as usize].add((r + 1) as u32, rows_len.get(r as usize));
            // dbg!(r, rows_len[r], cur_ans, idx);
            res = max(res, cur_ans);
        }
    }
    if res == 0 {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
