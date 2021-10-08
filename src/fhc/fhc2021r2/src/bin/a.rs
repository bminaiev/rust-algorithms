use std::{io, fs};
use std::io::{Write, BufWriter};
use std::fs::File;
use std::time::SystemTime;
use std::collections::BTreeMap;

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
    let rounds = sc.usize();
    let n = sc.usize();
    let mut can_change: BTreeMap<i32, i32> = BTreeMap::new();
    for _ in 0..n {
        let style = sc.i32();
        *can_change.entry(style).or_default() += 1;
    }
    let mut no_change: BTreeMap<i32, i32> = BTreeMap::new();
    let mut res = 0;
    for _ in 0..rounds {
        let mut next_can_change = BTreeMap::new();
        let mut next_no_change = BTreeMap::new();

        let mut required_new = 0;
        for _ in 0..n {
            let need_style = sc.i32();
            if *no_change.entry(need_style).or_default() > 0 {
                *no_change.entry(need_style).or_default() -= 1;
                *next_no_change.entry(need_style).or_default() += 1;
            } else if *can_change.entry(need_style).or_default() > 0 {
                *can_change.entry(need_style).or_default() -= 1;
                *next_can_change.entry(need_style).or_default() += 1;
            } else {
                required_new += 1;
                *next_no_change.entry(need_style).or_default() += 1;
            }
        }

        for entry in can_change.iter() {
            required_new -= entry.1;
        }

        if required_new > 0 {
            res += required_new;
        }

        can_change = next_can_change;
        no_change = next_no_change;
    }
    writeln!(out, "Case #{}: {}", test_n, res).unwrap();
}

pub fn main() {
    let input_file = get_last_modified_file();
    const OUTPUT_FILE: &str = "out/a.out";
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
