use std::{io, fs};
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

const USED: u8 = b'X';

fn solve_go_up(a: &Vec<Vec<u8>>, need_row: usize) -> usize {
    let n = a.len();
    let m = a[0].len();
    let mut pref_sum = vec![vec![0; m]; n + 1];
    for i in 0..n {
        for j in 0..m {
            pref_sum[i + 1][j] = pref_sum[i][j] + (if a[i][j] == USED { 1 } else { 0 })
        }
    }
    let mut res = m as usize;
    for cnt_up in 0..=n - need_row {
        let mut this_way_res = cnt_up;
        for col in 0..m {
            let expect_row = need_row + cnt_up;
            if expect_row < n && a[expect_row][col] == USED {
                this_way_res += 1;
            } else {
                if pref_sum[min(expect_row + 1, n)][col] > need_row {
                    this_way_res += 1;
                }
            }
        }
        res = min(res, this_way_res);
    }
    res
}

fn solve_one_test(sc: &mut Scanner, out: &mut BufWriter<File>, test_n: usize) {
    let n = sc.usize();
    let m = sc.usize();
    let need_row = sc.usize() - 1;
    let mut a = vec![];
    for _ in 0..n {
        a.push(sc.string());
    }
    let up = solve_go_up(&a, need_row);
    a.reverse();
    let down = solve_go_up(&a, n - 1 - need_row);
    let res = min(down, up);
    writeln!(out, "Case #{}: {}", test_n, res).unwrap();
}

pub fn main() {
    let input_file = get_last_modified_file();
    const OUTPUT_FILE: &str = "out/c.out";
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
