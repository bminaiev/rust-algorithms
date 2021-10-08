use std::io;
use std::io::Write;
use std::cmp::{max, min};

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
struct Task {
    cp: i64,
    cm: i64,
    h0: i64,
    a1: i64,
    d1: i64,
    n: i64,
}

fn solve_with_const(t: &Task, my_attack: i64, my_defence: i64, brute_inside: bool) -> i64 {
    let one_attack = t.cp * max(1, t.a1 - my_defence);
    let hit_times = 1 + (t.h0 - 1) / one_attack;
    let more = t.n - my_attack - my_defence;
    let one_attack_by_me = t.cp * max(1, my_attack - t.d1);
    let mut res = hit_times * one_attack_by_me;
    if more >= hit_times {
        let one_magic_hit = t.cm * (more - hit_times);
        res = max(res, hit_times * one_magic_hit);
    }
    let expected_magic_power = (one_attack_by_me + more * t.cm) / 2 / t.cm;
    const B: i64 = 3;
    for real_magic_power in expected_magic_power - B..expected_magic_power + B {
        if real_magic_power <= more {
            let times_cast = min(more - real_magic_power, hit_times);
            res = max(res, times_cast * real_magic_power * t.cm + (hit_times - times_cast) * one_attack_by_me);
        }
    }
    if brute_inside {
        for my_magic_power in 0..=more {
            let magic_hit_times = min(more - my_magic_power, hit_times);
            res = max(res, magic_hit_times * t.cm * my_magic_power + (hit_times - magic_hit_times) * one_attack_by_me);
        }
    }
    res
}

fn solve(t: &Task, buben: i64, brute_inside: bool) -> i64 {
    let mut res = 0;
    for my_attack in t.d1..=t.d1 + buben {
        let mut my_real_attack = if my_attack == t.d1 { 0 } else { my_attack };
        let smallest_defence = max(0, t.a1 - buben);
        for my_defence in smallest_defence..=t.a1 {
            let mut my_real_defence = if my_defence == smallest_defence { 0 } else { my_defence };
            if my_real_attack == t.d1 + buben {
                my_real_attack = t.n;
            }
            if my_defence == t.a1 {
                my_real_defence = t.n - my_real_attack;
            }
            if my_real_attack + my_real_defence > t.n || my_real_defence < 0 {
                continue;
            }
            let now = solve_with_const(t, my_real_attack, my_real_defence, brute_inside);
            if now > res {
                dbg!(my_real_attack, my_real_defence, now);
                res = max(res, now);
            }
        }
    }
    res
}

fn solve_defence(t: &Task, brute_inside: bool, my_attack: i64) -> i64 {
    let mut res = solve_with_const(t, my_attack, 0, brute_inside);
    let mut left = t.a1;
    let mut right = t.n - my_attack;
    while right - left > 3 {
        let m1 = (left * 2 + right) / 3;
        let m2 = (left + right * 2) / 3;
        let v1 = solve_with_const(t, my_attack, m1, brute_inside);
        let v2 = solve_with_const(t, my_attack, m2, brute_inside);
        if v1 >= v2 {
            right = m2;
        } else {
            left = m2;
        }
    }
    for def in left..=right {
        res = max(res, solve_with_const(t, my_attack, def, brute_inside));
    }

    res
}

fn solve_attack(t: &Task, brute_inside: bool) -> i64 {
    let mut res = solve_defence(t, brute_inside, 0);
    let mut left = t.d1;
    let mut right = t.n;


    while right - left > 3 && false {
        let m1 = (left * 2 + right) / 3;
        let m2 = (left + right * 2) / 3;
        let v1 = solve_defence(t, brute_inside, m1);
        let v2 = solve_defence(t, brute_inside, m2);
        if v1 >= v2 {
            right = m2;
        } else {
            left = m2;
        }
    }
    for attack in left..=right {
        let now = solve_defence(t, brute_inside, attack);
        dbg!(attack, now);
        res = max(res, now);
    }
    res
}

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

fn stress() {
    for it in 18638.. {
        dbg!(it);
        let mut rnd = Random::new(it);
        const MAX: usize = 1000;
        let cp = rnd.next_in_range(1, MAX) as i64;
        let cm = rnd.next_in_range(1, MAX) as i64;
        let h0 = rnd.next_in_range(1, MAX) as i64;
        let a1 = rnd.next_in_range(1, MAX) as i64;
        let d1 = rnd.next_in_range(1, MAX) as i64;
        let n = rnd.next_in_range(1, MAX) as i64;
        let t = Task { cp, cm, h0, a1, d1, n };
        let res = solve_attack(&t, false);
        dbg!("next sol");
        let res2 = solve(&t, 620, false);
        if res != res2 {
            dbg!(res, res2);
            dbg!(t);
            assert!(false);
        }
    }
}

pub fn main() {
    stress();
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let cp = sc.i64();
        let cm = sc.i64();
        let h0 = sc.i64();
        let a1 = sc.i64();
        let d1 = sc.i64();
        let n = sc.i64();
        let t = Task { cp, cm, h0, a1, d1, n };
        let res = solve(&t, 172, false);
        writeln!(out, "{}", res).unwrap();
    }
}
