use std::collections::HashMap;

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

const MAX: usize = (2e5 + 10.0) as usize;

fn get_divs(mut val: usize, primes: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0; 0];
    while val != 1 {
        let p = if primes[val] == 0 { val } else { primes[val] };
        res.push(p);
        val /= p;
    }
    res
}

const MOD: i32 = 1_000_000_007;

fn mul(x: i32, y: i32) -> i32 {
    ((x as i64) * (y as i64) % (MOD as i64)) as i32
}

fn change(pos: usize, val: usize, primes: &Vec<usize>, by_val: &mut Vec<HashMap<usize, usize>>, n: usize) -> i32 {
    let divs = get_divs(val, primes);
    let mut res = 1;
    for d in divs {
        *by_val[d].entry(pos).or_default() += 1;
        while by_val[d].len() == n {
            res = mul(res, d as i32);
            for pos in 0..n {
                let cur = by_val[d].get(&pos).cloned();
                match cur {
                    Some(1) => { by_val[d].remove(&pos); }
                    Some(x) => { by_val[d].insert(pos, x - 1).unwrap(); }
                    None => {}
                }
            }
        }
    }
    res
}

pub fn main() {
    let mut primes = vec![0; MAX];
    for i in 2..MAX {
        if primes[i] != 0 {
            continue;
        }
        for j in (i * 2..MAX).step_by(i) {
            primes[j] = i;
        }
    }
    let mut by_val = vec![HashMap::<usize, usize>::new(); MAX];
    let mut sc = Scanner::default();
    let n: usize = sc.next();
    let q: usize = sc.next();
    let mut res = 1;
    for i in 0..n {
        res = mul(res, change(i, sc.next(), &primes, &mut by_val, n));
    }
    for _ in 0..q {
        let pos = sc.next::<usize>() - 1;
        let val: usize = sc.next();
        res = mul(res, change(pos, val, &primes, &mut by_val, n));
        println!("{}", res);
    }
}