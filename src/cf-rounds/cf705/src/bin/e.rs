use std::cmp::{min, max};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

// Scanner code is copied from Russell Emerine's solution
// http://codeforces.com/contest/1477/submission/105755265
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    #[allow(dead_code)]
    fn next_string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}

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
    fn new(seed: usize) -> Self {
        Random { state: seed }
    }
}

fn cost(len: usize, rnd: &mut Random) -> usize {
    let mut res = std::usize::MAX;
    let mut divs = Vec::new();
    for div in 2..len {
        if len % div != 0 {
            continue;
        }
        divs.push(div);
    }
    for it in 0..1000 {
        for pos in 0..divs.len() {
            let another = rnd.next_in_range(0, pos + 1);
            divs.swap(pos, another);
        }
        let mut cur_res = 0;
        for _ in 0..1000 {
            let mut ok = vec![false; divs.len()];
            let cnt_ok = rnd.next_in_range(0, divs.len() + 1);
            for _ in 0..cnt_ok {
                ok[rnd.next_in_range(0, divs.len())] = true;
            }
            for i in 0..divs.len() {
                if !ok[i] {
                    continue;
                }
                for j in 0..divs.len() {
                    if divs[j] % divs[i] == 0 {
                        ok[j] = true;
                    }
                }
            }
            let mut my_cost = 0;
            let mut known = vec![false; divs.len()];
            for pos in 0..divs.len() {
                if known[pos] {
                    continue;
                }
                for i in pos + 1..divs.len() {
                    if divs[i] % divs[pos] == 0 && ok[pos] {
                        known[i] = true;
                    }
                    if divs[pos] % divs[i] == 0 && !ok[pos] {
                        known[i] = true;
                    }
                }
                my_cost += 2;
            }
            cur_res = max(cur_res, my_cost);
        }
        res = min(res, cur_res);
    }
    res
}

pub fn main() {
    let mut scanner = Scanner::default();
    let mut rnd = Random::new(787788);
    // for len in 2..=1000 {
    let len = 720;
    let cost = cost(len, &mut rnd);
    // if cost > 40 {
    eprintln!("len = {}, cost = {}", len, cost);
    // }
    // }
}