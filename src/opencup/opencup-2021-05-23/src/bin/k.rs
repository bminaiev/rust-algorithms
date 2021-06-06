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


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n_words = sc.usize();
    const N: usize = 25;
    let mut word_for_mask = vec![None; 1 << N];
    let mut cnt_submasks = vec![0; 1 << N];
    let words = sc.vec::<String>(n_words);
    for i in 0..n_words {
        let word = words[i].clone().into_bytes();
        let mut mask = 0;
        for len in 0..word.len() {
            let c = word[len] - b'a';
            if c >= N as u8 {
                continue;
            }
            mask |= 1 << c;
            if word_for_mask[mask] == None {
                word_for_mask[mask] = Some(&words[i]);
                cnt_submasks[mask] = 1;
            }
        }
    }
    for bit in 0..N {
        for mask in 0..1 << N {
            if (mask & (1 << bit)) == 0 {
                cnt_submasks[mask | (1 << bit)] += cnt_submasks[mask];
            }
        }
    }
    let queries = sc.usize();
    for _ in 0..queries {
        let mut need_mask = 0;
        let mut can_mask = 0;
        let mut bit = 0;
        for _ in 0..5 {
            let s = sc.string();
            for y in 0..5 {
                if s[y] == b'r' {
                    need_mask |= 1 << bit;
                } else if s[y].is_ascii_uppercase() {
                    can_mask |= 1 << bit;
                }
                bit += 1;
            }
        }
        assert!(need_mask < 1 << N);
        assert!(can_mask < 1 << N);
        let exist = |can_mask: i32, need_mask: i32| -> bool {
            let mut sum_ways = 0;
            let mut submask_need = need_mask;
            loop {
                let check_mask = submask_need | can_mask;
                let ways = cnt_submasks[check_mask as usize];
                let diff_mask = need_mask ^ submask_need;
                if diff_mask.count_ones() % 2 == 0 {
                    sum_ways += ways;
                } else {
                    sum_ways -= ways;
                }
                if submask_need == 0 {
                    break;
                }
                submask_need = (submask_need - 1) & need_mask;
            }
            assert!(sum_ways >= 0);
            sum_ways > 0
        };
        if !exist(can_mask, need_mask) {
            writeln!(out, "IMPOSSIBLE").unwrap();
        } else {
            let mut real_used_can_mask = can_mask;
            for bit in 0..N {
                if (1 << bit) & real_used_can_mask == 0 {
                    continue;
                }
                if exist(real_used_can_mask ^ (1 << bit), need_mask) {
                    real_used_can_mask ^= 1 << bit;
                }
            }
            let used_full_mask = real_used_can_mask | need_mask;
            if let Some(word) = &word_for_mask[used_full_mask as usize] {
                writeln!(out, "{} 9", word).unwrap();
            } else {
                assert!(false);
            }
        }
    }
}
