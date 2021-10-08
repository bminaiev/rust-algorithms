use std::{io, fs, thread};
use std::io::{Write, BufWriter};
use std::fs::File;
use std::time::SystemTime;
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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct ModifiedFile {
    modified: SystemTime,
    path: String,
}

fn get_last_modified_file() -> String {
    let mut all_files = vec![];

    for entry in fs::read_dir("/home/borys/Downloads").unwrap() {
        let entry = entry.unwrap();
        let path = String::from(entry.path().to_str().unwrap());

        let metadata = fs::metadata(&path).unwrap();
        let modified = metadata.modified().unwrap();

        all_files.push(ModifiedFile { path, modified });
    }

    all_files.sort();

    let last = all_files.last().unwrap();
    println!("Last file is {}", last.path);
    last.path.clone()
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

struct State {
    g: Vec<Vec<usize>>,
    freq: Vec<usize>,
}


const MAX: usize = 20;

struct MutState {
    by_freq: Vec<Vec<usize>>,
    h: Vec<usize>,
    up: Vec<Vec<usize>>,
}

impl MutState {
    fn up(&self, mut v: usize, lvl: usize) -> usize {
        for i in 0..MAX {
            if ((1 << i) & lvl) != 0 {
                v = self.up[i][v];
            }
        }
        v
    }

    fn lca(&self, mut x: usize, mut y: usize) -> usize {
        let h1 = self.h[x];
        let h2 = self.h[y];
        let need_h = min(h1, h2);
        x = self.up(x, h1 - need_h);
        y = self.up(y, h2 - need_h);
        if x == y {
            return x;
        }
        for lvl in (0..MAX).rev() {
            if self.up[lvl][x] != self.up[lvl][y] {
                x = self.up[lvl][x];
                y = self.up[lvl][y];
            }
        }
        assert_eq!(self.up[0][x], self.up[0][y]);
        return self.up[0][x];
    }
}

fn dfs(v: usize, state: &State, p: usize, mut_state: &mut MutState) {
    mut_state.by_freq[state.freq[v] - 1].push(v);
    mut_state.h[v] = mut_state.h[p] + 1;
    mut_state.up[0][v] = p;
    for lvl in 1..MAX {
        mut_state.up[lvl][v] = mut_state.up[lvl - 1][mut_state.up[lvl - 1][v]];
    }

    for &to in state.g[v].iter() {
        if to == p {
            continue;
        }
        dfs(to, state, v, mut_state);
    }
}

struct DfsAns {
    cur_delta: i32,
    res: i32,
}

fn dfs_res(v: usize, state: &State, p: usize, delta: &Vec<i32>) -> DfsAns {
    let mut res = 0;
    let mut cur_delta = delta[v];
    for &to in state.g[v].iter() {
        if to == p {
            continue;
        }
        let child_ans = dfs_res(to, state, v, delta);
        cur_delta += child_ans.cur_delta;
        if child_ans.cur_delta == 0 {
            res += 1;
        }
        res += child_ans.res;
    }
    DfsAns { res, cur_delta }
}


fn solve_one_test(sc: &mut Scanner, out: &mut BufWriter<File>, test_n: usize) {
    let n = sc.usize();
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let freq = sc.vec::<usize>(n);
    let by_freq = vec![vec![]; n];
    let state = State { g, freq };
    let mut mut_state = MutState { by_freq, h: vec![0; n], up: vec![vec![0; n]; MAX] };
    let mut delta = vec![0; n];
    dfs(0, &state, 0, &mut mut_state);
    for freq in 0..n {
        for w in mut_state.by_freq[freq].windows(2) {
            let lca = mut_state.lca(w[0], w[1]);
            delta[lca] -= 2;
            delta[w[0]] += 1;
            delta[w[1]] += 1;
        }
    }
    let res = dfs_res(0, &state, 0, &delta);
    assert_eq!(res.cur_delta, 0);
    writeln!(out, "Case #{}: {}", test_n, res.res).unwrap();
}

pub fn main() {
    let input_file = get_last_modified_file();
    const OUTPUT_FILE: &str = "out/b.out";
    let mut out = std::io::BufWriter::new(File::create(OUTPUT_FILE).unwrap());
    let mut sc = Scanner::new_file(&input_file);

    let tc = sc.usize();
    for test_n in 1..=tc {
        dbg!("started", test_n);
        solve_one_test(&mut sc, &mut out, test_n);
    }

    let source_code_file = concat!(env!("CARGO_MANIFEST_DIR"), "/", file!());
    dbg!(source_code_file);
    const OUTPUT_DIR: &str = "/home/borys/fb-output";

    fs::create_dir_all(OUTPUT_DIR).unwrap();
    fs::copy(source_code_file, String::from(OUTPUT_DIR) + "/solution.rs").unwrap();
    out.flush().unwrap();
    fs::copy(OUTPUT_FILE, String::from(OUTPUT_DIR) + "/answer.txt").unwrap();
}
