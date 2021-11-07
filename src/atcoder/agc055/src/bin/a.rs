use std::io;
use std::io::Write;
use std::collections::HashMap;

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

#[allow(dead_code)]
struct Random {
    state: usize,
}

impl Random {
    fn next(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    #[allow(dead_code)]
    fn next_in_range(&mut self, from: usize, to: usize) -> usize {
        assert!(from < to);
        from + self.next() % (to - from)
    }

    #[allow(dead_code)]
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }

    #[allow(dead_code)]
    fn new(seed: usize) -> Self {
        assert_ne!(seed, 0);
        Self {
            state: seed,
        }
    }
}

fn solve(s: &[u8]) -> Vec<usize> {
    let n = s.len() / 3;

    let mut res = vec![0; n * 3];

    // cnt[char][part]
    let mut cnt = vec![vec![0; 3]; 3];

    for i in 0..n {
        for part in 0..3 {
            let c = (s[part * n + i] - b'A') as usize;
            cnt[c][part] += 1;
        }
    }
    for ab in 0..=cnt[0][0] {
        if ab > cnt[1][1] {
            continue;
        }
        let cb = cnt[1][1] - ab;
        if cb > cnt[2][0] {
            continue;
        }
        let ca = cnt[2][0] - cb;
        if ca > cnt[0][1] {
            continue;
        }
        let ba = cnt[0][1] - ca;
        if ba > cnt[1][0] {
            continue;
        }
        let bc = cnt[1][0] - ba;
        if ab > cnt[0][0] {
            continue;
        }
        let ac = cnt[0][0] - ab;
        if ab + ba != cnt[2][2] {
            continue;
        }
        if ac + ca != cnt[1][2] {
            continue;
        }
        if bc + cb != cnt[0][2] {
            continue;
        }
        let mut ab_ids = vec![];
        for id in 0..ab {
            ab_ids.push(0);
        }
        let mut ac_ids = vec![];
        for id in ab..ab + ac {
            ac_ids.push(1);
        }
        let mut ba_ids = vec![];
        for id in ab + ac..ab + ac + ba {
            ba_ids.push(2);
        }
        let mut bc_ids = vec![];
        for id in ab + ac + ba..ab + ac + ba + bc {
            bc_ids.push(3);
        }
        let start = ab + ac + ba + bc;
        let mut ca_ids = vec![];
        for id in start..start + ca {
            ca_ids.push(4);
        }
        let mut cb_ids = vec![];
        for id in start + ca..start + ca + cb {
            cb_ids.push(5);
        }

        for part in 0..3 {
            let mut can_use_ids = vec![vec![]; 3];
            if part == 0 {
                can_use_ids[0].append(&mut ab_ids.clone());
                can_use_ids[0].append(&mut ac_ids.clone());
                can_use_ids[1].append(&mut ba_ids.clone());
                can_use_ids[1].append(&mut bc_ids.clone());
                can_use_ids[2].append(&mut ca_ids.clone());
                can_use_ids[2].append(&mut cb_ids.clone());
            } else if part == 1 {
                can_use_ids[0].append(&mut ba_ids.clone());
                can_use_ids[0].append(&mut ca_ids.clone());
                can_use_ids[1].append(&mut ab_ids.clone());
                can_use_ids[1].append(&mut cb_ids.clone());
                can_use_ids[2].append(&mut ac_ids.clone());
                can_use_ids[2].append(&mut bc_ids.clone());
            } else {
                can_use_ids[0].append(&mut bc_ids.clone());
                can_use_ids[0].append(&mut cb_ids.clone());
                can_use_ids[1].append(&mut ac_ids.clone());
                can_use_ids[1].append(&mut ca_ids.clone());
                can_use_ids[2].append(&mut ab_ids.clone());
                can_use_ids[2].append(&mut ba_ids.clone());
            }
            for i in 0..n {
                let c = (s[part * n + i] - b'A') as usize;
                let used = can_use_ids[c].pop().unwrap();
                res[part * n + i] = used + 1;
            }
        }

        return res;
    }
    dbg!(s);
    unreachable!();
}

fn stress() {
    for test in 1.. {
        dbg!(test);
        let mut rnd = Random::new(test);

        let n = 1 + rnd.next_in_range(0, 10000);
        let mut s = vec![];
        let mut cnt = vec![n; 3];
        dbg!(cnt);
        while s.len() != n * 3 {
            let id = rnd.next_in_range(0, 3);
            if cnt[id] > 0 {
                cnt[id] -= 1;
            } else {
                continue;
            }
            s.push(b'A' + id as u8);
        }

        solve(&s);
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    // stress();

    let n = sc.usize();
    let s = sc.string();
    assert_eq!(n * 3, s.len());
    let res = solve(&s);


    for &x in res.iter() {
        assert!(x <= 6);
        write!(out, "{}", x).unwrap();
    }
    writeln!(out).unwrap();
}
