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


fn solve(cnt : &[usize], k : usize) {
    let mut cnt : Vec<_> = cnt.iter().cloned().collect();
    let n = cnt.len();

    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let mut sorted_ids: Vec<_> = (0..n).collect();
    sorted_ids.sort_by(|a, b| cnt[*a].cmp(&cnt[*b]));
    let mut last_pos = vec![None; n];
    let mut a = vec![];

    let total_size: usize = (0..n).map(|id| cnt[id]).sum();
    dbg!(total_size);
    let is_ok = |last_pos: &Vec<Option<usize>>, a: &[usize], sorted_ids: &[usize], cnt: &[usize]| -> bool {
        let mut max_iter = n;
        for pos in a.len()..a.len() + k {
            if pos == total_size {
                break;
            }
            let total_left = total_size - pos;
            let max_can_put = 1 + (total_left - 1) / k;
            if max_iter != 0 {
                while let Some(prev_pos) = last_pos[sorted_ids[max_iter - 1]] {
                    if prev_pos + k <= pos {
                        break;
                    }
                    max_iter -= 1;
                    if max_iter == 0 {
                        break;
                    }
                }
            }
            let iter_val = if max_iter == 0 { 0 } else { sorted_ids[max_iter - 1] };
            let mut use_from_iter = true;
            if pos >= k {
                let check_val = a[pos - k];
                if (cnt[check_val] > cnt[iter_val] || max_iter == 0) && cnt[check_val] != 0 {
                    if cnt[check_val] > max_can_put || cnt[check_val] == 0 {
                        return false;
                    }
                    use_from_iter = false;
                }
            }
            if use_from_iter {
                assert!(max_iter > 0);
                if cnt[iter_val] > max_can_put || cnt[iter_val] == 0 {
                    return false;
                }
                max_iter -= 1;
            }
        }
        true
    };
    if !is_ok(&last_pos, &a, &sorted_ids, &cnt) {
        writeln!(out, "-1").unwrap();
        return;
    }
    for pos in 0..total_size {
        let mut found = false;
        for value in 0..n {
            if cnt[value] == 0 {
                continue;
            }
            if let Some(prev_pos) = last_pos[value] {
                if prev_pos + k > pos {
                    continue;
                }
            }
            let prev_last_pos = last_pos[value];
            last_pos[value] = Some(pos);
            a.push(value);
            cnt[value] -= 1;
            if is_ok(&last_pos, &a, &sorted_ids, &cnt) {
                found = true;
                dbg!("found!", a, cnt);
                for pos in (1..n).rev() {
                    if cnt[sorted_ids[pos - 1]] > cnt[sorted_ids[pos]] {
                        sorted_ids.swap(pos - 1, pos);
                    }
                }
                break;
            }
            cnt[value] += 1;
            last_pos[value] = prev_last_pos;
            a.pop();
        }
        assert!(found);
    }
    for &v in a.iter() {
        write!(out, "{} ", v + 1).unwrap();
    }
    writeln!(out).unwrap();
}


#[allow(dead_code)]
struct Random {
    state: usize
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

pub fn main() {
    for tc in 687.. {
        dbg!(tc);
        let mut rnd = Random::new(tc);
        const MAX : usize = 6;
        let n = rnd.next_in_range(1, MAX);
        let k = rnd.next_in_range(1, n + 1);
        let mut cnt = vec![0; n];
        for i in 0..n {
            cnt[i] = rnd.next_in_range(1, 4);
        }
        dbg!(cnt, k);
        solve(&cnt, k);
    }
}
