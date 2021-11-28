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
        Self {
            buffer: vec![],
            input_source: InputSource::Stdin,
        }
    }

    #[allow(dead_code)]
    fn new_file(filename: &str) -> Self {
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
        lines.reverse();
        Self {
            buffer: vec![],
            input_source: InputSource::FromFile(lines),
        }
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
            InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            InputSource::FromFile(lines) => match lines.pop() {
                Some(line) => input = line,
                None => return false,
            },
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

struct Op {
    from: usize,
    to: usize,
    char: u8,
}

fn pref_sums(s: &Vec<u8>, c: u8) -> Vec<usize> {
    let mut res = vec![0; s.len() + 1];
    for i in 0..s.len() {
        res[i + 1] = res[i] + (if s[i] == c { 1 } else { 0 })
    }
    res
}

const A: u8 = b'A';
const B: u8 = b'B';
const C: u8 = b'C';

fn solve2(
    pref_a: &[usize],
    pref_b: &[usize],
    pref_c: &[usize],
    a: u8,
    b: u8,
    c: u8,
    len_a: usize,
) -> Vec<Op> {
    let n = (pref_a.len() - 1) / 3;
    let more_b = n - pref_b[len_a];
    // let more_c = n - pref_c[len_a];
    vec![
        Op {
            from: len_a,
            to: len_a + more_b,
            char: b,
        },
        Op {
            from: len_a + more_b,
            to: n * 3,
            char: c,
        },
    ]
}

fn solve(s: &Vec<u8>, n: usize) -> Vec<Op> {
    let pref_a = pref_sums(s, A);
    let pref_b = pref_sums(s, B);
    let pref_c = pref_sums(s, C);
    if pref_a[n * 3] == n && pref_b[n * 3] == n && pref_c[n * 3] == n {
        return vec![];
    }
    for &change in [A, B, C].iter() {
        let (my_pref, p1, p2) = if change == A {
            (&pref_a, &pref_b, &pref_c)
        } else if change == B {
            (&pref_b, &pref_a, &pref_c)
        } else if change == C {
            (&pref_c, &pref_a, &pref_b)
        } else {
            unreachable!();
        };
        if my_pref[3 * n] > n {
            continue;
        }
        for last_changed in 0..3 * n {
            let more_right = my_pref[3 * n] - my_pref[last_changed + 1];
            let mut left = -1;
            let mut right = last_changed as i32;
            while right - left > 1 {
                let mid = ((left + right) / 2) as usize;
                let my_sum = more_right + (last_changed - mid + 1) + my_pref[mid];
                if my_sum <= n {
                    right = mid as i32;
                } else {
                    left = mid as i32;
                }
            }
            let first_change = right as usize;
            // dbg!(change, first_change, last_changed);
            // using [right..last_change]
            let my_color = more_right + (last_changed - first_change + 1) + my_pref[first_change];
            let c2 = p1[first_change] + p1[n * 3] - p1[last_changed + 1];
            let c3 = p2[first_change] + p2[n * 3] - p2[last_changed + 1];
            // dbg!(my_color, c2, c3);
            if my_color == n {
                if c2 == n {
                    if c3 == n {
                        return vec![Op {
                            from: first_change,
                            to: last_changed + 1,
                            char: change,
                        }];
                    }
                }
            }
        }
    }
    for pref_size in 0..3 * n {
        if pref_a[pref_size] == n {
            return solve2(&pref_a, &pref_b, &pref_c, A, B, C, pref_size);
        }
        if pref_b[pref_size] == n {
            return solve2(&pref_b, &pref_a, &pref_c, B, A, C, pref_size);
        }
        if pref_c[pref_size] == n {
            return solve2(&pref_c, &pref_b, &pref_a, C, B, A, pref_size);
        }
    }
    unreachable!();
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let s = sc.string();
    let ops = solve(&s, n);
    assert!(ops.len() <= 2);
    writeln!(out, "{}", ops.len()).unwrap();
    for op in ops.iter() {
        writeln!(out, "{} {} {}", op.from + 1, op.to, op.char as char).unwrap();
    }
}
