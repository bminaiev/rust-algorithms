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

#[allow(dead_code)]
pub struct Fenwick {
    values: Vec<i32>
}

impl Fenwick {
    #[allow(dead_code)]
    fn get_sum(&self, mut pos: usize) -> i32 {
        let mut res = 0i32;
        loop {
            res += self.values[pos] as i32;
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    #[allow(dead_code)]
    fn add(&mut self, mut pos: usize, change: i32) {
        while pos < self.values.len() {
            self.values[pos] += change;
            pos |= pos + 1;
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new(n: usize) -> Self {
        let values = vec![0; n];
        Fenwick { values }
    }
}


/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

const BUBEN: usize = 50;

struct Block {
    b: Vec<usize>,
    steps: Vec<usize>,
}

impl Block {
    fn recalc_steps(&mut self, fenw: &mut Fenwick) {
        for (idx, &val) in self.b.iter().enumerate() {
            let mut l = -1;
            let mut r = fenw.values.len() as i32;
            while r - l > 1 {
                let mid = (l + r) >> 1;
                let add = fenw.get_sum(mid as usize);
                if mid + add >= val as i32 {
                    r = mid;
                } else {
                    l = mid;
                }
            }
            self.steps[idx] = r as usize;
            fenw.add(r as usize, 1);
        }
        self.steps.sort();
        for idx in self.steps.iter() {
            fenw.add(*idx, -1);
        }
    }

    fn create(b: Vec<usize>, fenw: &mut Fenwick) -> Self {
        let steps = vec![0; b.len()];
        let mut res = Self { b, steps };
        res.recalc_steps(fenw);
        res
    }

    fn update(&mut self, pos: usize, val: usize, fenw: &mut Fenwick) {
        self.b[pos] = val;
        self.recalc_steps(fenw);
    }

    fn get(&self, pos: usize, mut val: usize) -> usize {
        if pos == 0 {
            let mut l = -1;
            let mut r = self.steps.len() as i32;
            while r - l > 1 {
                let mid = (l + r) / 2;
                if self.steps[mid as usize] <= val {
                    l = mid;
                } else {
                    r = mid;
                }
            }
            return val + (r as usize);
        }
        for &x in self.b[pos..].iter() {
            if x <= val {
                val += 1;
            }
        }
        val
    }
}

struct Solver {
    blocks: Vec<Block>
}

impl Solver {
    fn create(b: Vec<usize>, fenw: &mut Fenwick) -> Self {
        let mut blocks = vec![];
        for start in (0..b.len()).step_by(BUBEN) {
            let small_b = b[start..min(start + BUBEN, b.len())].iter().cloned().collect();
            blocks.push(Block::create(small_b, fenw));
        }
        Self { blocks }
    }

    fn update(&mut self, pos: usize, val: usize, fenw: &mut Fenwick) {
        let id = pos / BUBEN;
        self.blocks[id].update(pos % BUBEN, val, fenw);
    }

    fn get(&self, pos: usize) -> usize {
        let id = pos / BUBEN;
        let mut val = self.blocks[id].b[pos % BUBEN];
        val = self.blocks[id].get((pos % BUBEN) + 1, val);
        for b in self.blocks[id + 1..].iter() {
            val = b.get(0, val);
        }
        return val;
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let mut fenw = Fenwick::new(n);
    let b = sc.vec::<usize>(n);
    let mut solver = Solver::create(b, &mut fenw);
    let q = sc.usize();
    for _ in 0..q {
        let q_type = sc.usize();
        let pos = sc.usize() - 1;
        if q_type == 1 {
            let val = sc.usize();
            solver.update(pos, val, &mut fenw);
        } else {
            let cur = solver.get(pos);
            writeln!(out, "{}", n - cur).unwrap();
        }
    }
}
