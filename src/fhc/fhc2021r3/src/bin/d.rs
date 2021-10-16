use std::{io, fs};
use std::io::{Write, BufWriter};
use std::fs::File;
use std::time::SystemTime;
use std::cmp::max;

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

struct Dsu {
    p: Vec<usize>,
    robot: Vec<bool>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        let robot = vec![false; n];
        Self { p, robot }
    }

    fn get(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        } else {
            self.p[v] = self.get(self.p[v]);
            return self.p[v];
        }
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        self.p[x] = y;
        self.robot[y] |= self.robot[x];
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Height {
    h: i32,
    id: usize,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Event {
    h: i32,
    id: usize,
}

fn solve_one_test(sc: &mut Scanner, out: &mut BufWriter<File>, test_n: usize) {
    let rows = sc.usize();
    let cols = sc.usize();
    let mut ids = vec![vec![0; cols]; rows];
    let mut n = 0;
    let mut rr = vec![];
    let mut cc = vec![];
    for r in 0..rows {
        for c in 0..cols {
            ids[r][c] = n;
            rr.push(r);
            cc.push(c);
            n += 1;
        }
    }
    let mut h_by_id = vec![0; n];
    let mut heights = vec![];
    for r in 0..rows {
        for c in 0..cols {
            h_by_id[ids[r][c]] = sc.i32();
            heights.push(Height { id: ids[r][c], h: h_by_id[ids[r][c]] })
        }
    }
    heights.sort();
    heights.reverse();

    let mut max_can_do = 0;
    let mut need_robots = 0;

    let mut events = vec![];
    for r in 0..rows {
        for c in 0..cols {
            events.push(Event { h: sc.i32(), id: ids[r][c] });
        }
    }

    events.sort();

    let mut dsu = Dsu::new(n);

    let mut alive = vec![false; n];

    let mut iter = 0;
    for e in events.iter().rev() {
        while iter != heights.len() && heights[iter].h > e.h {
            let id = heights[iter].id;
            alive[id] = true;
            let r = rr[id];
            let c = cc[id];
            for shift in SHIFTS.iter() {
                let nr = (r as i32) + shift.dr;
                let nc = (c as i32) + shift.dc;
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    let nid = ids[nr][nc];

                    if alive[nid] {
                        dsu.unite(nid, id);
                    }
                }
            }

            iter += 1;
        }

        let need_id = e.id;
        if !alive[need_id] {
            continue;
        }
        max_can_do += 1;

        let dsu_id = dsu.get(need_id);
        if !dsu.robot[dsu_id] {
            need_robots += 1;
            dsu.robot[dsu_id] = true;
        }
    }


    writeln!(out, "Case #{}: {} {}", test_n, max_can_do, need_robots).unwrap();
}

struct Shift {
    dr: i32,
    dc: i32,
}

const SHIFTS: [Shift; 4] = [Shift { dr: 1, dc: 0 }, Shift { dr: -1, dc: 0 }, Shift { dr: 0, dc: 1 }, Shift { dr: 0, dc: -1 }, ];


pub fn main() {
    let input_file = get_last_modified_file();
    const OUTPUT_FILE: &str = "out/d.out";
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
