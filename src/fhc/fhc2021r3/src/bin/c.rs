use std::{io, fs};
use std::io::{Write, BufWriter};
use std::fs::File;
use std::time::SystemTime;
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


fn solve_one_test(sc: &mut Scanner, out: &mut BufWriter<File>, test_n: usize) {
    let n = sc.usize();
    let k = sc.usize();
    let mut g = vec![vec![]; n];
    for v in 1..n {
        let p = sc.usize() - 1;
        g[p].push(v);
    }
    const SMALL: i64 = -1_000_000_000;
    let mut dp = vec![vec![SMALL; k + 2]; n];
    let mut full_size = vec![1; n];
    for v in (0..n).rev() {
        for &to in g[v].iter() {
            full_size[v] += full_size[to];
        }
    }
    for v in (0..n).rev() {
        dp[v][0] = 0;
        dp[v][1] = 0;
        for cnt in 0..=min(k + 1, g[v].len() + 1) {
            dp[v][cnt] = 0;
        }
        if g[v].len() >= k {
            let mut sum_best_dp = 0;
            for &to in g[v].iter() {
                sum_best_dp += max(dp[to][1], dp[to][k + 1]);
            }
            for &to in g[v].iter() {
                if full_size[to] >= 2 {
                    let cur_best_sum = sum_best_dp - max(dp[to][1], dp[to][k + 1]);
                    dp[v][k + 1] = max(dp[v][k + 1], cur_best_sum + 1);
                }
            }
        }
        for &to in g[v].iter() {
            let subtree_size = k + 1;
            let total_tree = min(k + 1, subtree_size + 1);
            dp[v][total_tree] = max(dp[v][total_tree], dp[to][subtree_size] + 1);
        }
        let mut total_sum = vec![SMALL; k + 2];
        total_sum[1] = 0;
        for &to in g[v].iter() {
            for cur_sum in (0..k + 2).rev() {
                let dp_cur_sum = total_sum[cur_sum];
                for subtree in 0..k + 2 {
                    let next = min(k + 1, cur_sum + subtree);
                    total_sum[next] = max(total_sum[next], dp_cur_sum + dp[to][subtree])
                }
            }
        }
        for sz in 0..k + 2 {
            dp[v][sz] = max(dp[v][sz], total_sum[sz]);
        }
    }
    let mut res = 0;
    for for_sz in dp[0].iter() {
        res = max(res, *for_sz);
    }
    writeln!(out, "Case #{}: {}", test_n, res).unwrap();
}

pub fn main() {
    let input_file = get_last_modified_file();
     const OUTPUT_FILE: &str = "out/c.out";
    let mut out = std::io::BufWriter::new(File::create(OUTPUT_FILE).unwrap());
    let mut sc = Scanner::new_file(&input_file);

    let tc = sc.usize();
    for test_n in 1..=tc {
        dbg!("started", test_n, tc);
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
