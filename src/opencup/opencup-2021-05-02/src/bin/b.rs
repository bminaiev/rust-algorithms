use std::io;
use std::io::Write;
use std::cmp::min;

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


struct Dsu {
    p: Vec<usize>,
    size: Vec<usize>,
    alive: Vec<bool>,
    n: usize,
}

impl Dsu {
    fn create(n: usize) -> Self {
        let p = vec![0; n];
        let size = vec![1; n];
        let alive = vec![false; n];
        let mut res = Self { p, size, alive, n };
        res.init();
        res
    }

    fn init(&mut self) {
        for i in 0..self.n {
            self.p[i] = i;
            self.size[i] = 1;
            self.alive[i] = false;
        }
    }

    fn get(&mut self, x: usize) -> usize {
        if self.p[x] == x {
            return x;
        }
        self.p[x] = self.get(self.p[x]);
        return self.p[x];
    }

    fn unite(&mut self, mut v: usize, mut u: usize) -> usize {
        v = self.get(v);
        u = self.get(u);
        self.p[v] = u;
        let result = self.size[v] * self.size[u];
        self.size[u] += self.size[v];
        result
    }
}

fn sort(idx: &mut [usize], sort_by: &[usize], tmp_cnt: &mut [usize], tmp_idx: &mut [usize]) {
    for x in tmp_cnt.iter_mut() {
        *x = 0;
    }
    for &x in sort_by.iter() {
        tmp_cnt[x] += 1;
    }
    for i in 1..tmp_cnt.len() {
        tmp_cnt[i] += tmp_cnt[i - 1];
    }
    for &id in idx.iter().rev() {
        let bucket = sort_by[id];
        tmp_cnt[bucket] -= 1;
        tmp_idx[tmp_cnt[bucket]] = id;
    }
    for (pos, &val) in tmp_idx.iter().enumerate() {
        idx[pos] = val;
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let mut a = vec![vec![0; m]; n];
    for time in 0..n * m {
        let x = sc.usize() - 1;
        let y = sc.usize() - 1;
        a[x][y] = time;
    }
    let mut dsu = Dsu::create(m);
    let mut idx = vec![0; m];
    let mut sort_by = vec![0; m];
    let mut tmp_cnt = vec![0; 512];
    let mut tmp_idx = vec![0; m];
    let mut final_res = vec![0; n * m + 1];
    for r1 in 0..n {
        let mut when_die = vec![std::usize::MAX; m];
        for r2 in r1 + 1..=n {
            for c in 0..m {
                when_die[c] = min(when_die[c], a[r2 - 1][c]);
                idx[c] = c;
            }
            {
                for c in 0..m {
                    sort_by[c] = when_die[c] & 511;
                }
                sort(&mut idx, &sort_by, &mut tmp_cnt, &mut tmp_idx);
            }
            {
                for c in 0..m {
                    sort_by[c] = when_die[c] >> 9;
                }
                sort(&mut idx, &sort_by, &mut tmp_cnt, &mut tmp_idx);
            }
            dsu.init();
            for pos in (0..m).rev() {
                let col = idx[pos];
                let when = when_die[col];
                dsu.alive[col] = true;
                let mut add_res = 1;
                if col > 0 && dsu.alive[col - 1] {
                    add_res += dsu.unite(col, col - 1);
                }
                if col + 1 < m && dsu.alive[col + 1] {
                    add_res += dsu.unite(col, col + 1);
                }
                final_res[when] += add_res;
            }
        }
    }
    for pos in (1..final_res.len()).rev() {
        final_res[pos - 1] += final_res[pos];
    }
    for x in final_res.iter().skip(1) {
        writeln!(out, "{}", x).unwrap();
    }
}
