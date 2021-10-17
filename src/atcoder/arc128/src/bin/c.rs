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

#[derive(Debug)]
struct Suffix {
    pos: usize,
    av_score: f64,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let max_val = sc.i32() as f64;
    let total_sum = sc.i32() as f64;
    let a = sc.vec::<f64>(n);

    let mut suffixes = vec![];
    let mut cur_sum_a = 0.0;

    let mut av_scores = vec![0.0; n + 1];

    for pos in (0..n).rev() {
        cur_sum_a += a[pos];
        av_scores[pos] = cur_sum_a / ((n - pos) as f64);
        suffixes.push(Suffix { pos, av_score: cur_sum_a / ((n - pos) as f64) })
    }
    suffixes.sort_by(|a, b| a.av_score.partial_cmp(&b.av_score).unwrap());
    suffixes.reverse();


    let mut res = 0.0;

    if false {
        let mut alr_from = n;
        let mut alr_av_score = 0.0;

        for suff in suffixes.iter() {
            if suff.pos > alr_from {
                continue;
            }
            let max_total_s = ((n - suff.pos) as f64) * max_val;
            if max_total_s <= total_sum {
                alr_from = suff.pos;
                alr_av_score = suff.av_score;
                res = alr_av_score * max_val * ((n - alr_from) as f64);
                dbg!("??", res);
                continue;
            }
            // h * (n - suff.pos) + (max_val - h) * (n - alr_from) == total_sum
            // h * (alr_from - suff.pos) + max_val * n - max_val * alr_from == total_sum
            let h = (total_sum - max_val * ((n - alr_from) as f64)) / ((alr_from - suff.pos) as f64);
            res = h * suff.av_score * ((n - suff.pos) as f64) + (max_val - h) * alr_av_score * ((n - alr_from) as f64);
            dbg!("$$", res, h, suff);
            break;
        }
    }
    for pos1 in 0..n {
        for pos2 in pos1 + 1..=n {
            let h = (total_sum - max_val * ((n - pos2) as f64)) / ((pos2 - pos1) as f64);
            if h < 0.0 || h > max_val {
                continue;
            }
            let cur_res = h * av_scores[pos1] * ((n - pos1) as f64) + (max_val - h) * av_scores[pos2] * ((n - pos2) as f64);

            if cur_res > res {
                res = cur_res;
            }
        }
    }
    writeln!(out, "{}", res).unwrap();
}
