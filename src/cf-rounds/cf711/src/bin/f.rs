use std::io;
use std::io::Write;
use std::collections::{HashMap, BTreeSet};

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

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
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
            std::io::stdin().read_line(&mut input).expect("Failed read");
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

#[derive(Clone, Eq, PartialEq, Hash)]
struct State {
    cnts: Vec<usize>
}

#[allow(unused)]
fn grundy(s: &State, cached: &mut HashMap<State, usize>, parents: &[usize]) -> usize {
    match cached.get(s) {
        Some(&res) => return res,
        None => {
            let mut seen = BTreeSet::new();
            for pos in 0..s.cnts.len() {
                if s.cnts[pos] > 0 {
                    for remove in 1..=s.cnts[pos] {
                        let mut new_cnts = s.cnts.clone();
                        new_cnts[pos] -= remove;
                        if pos > 0 {
                            new_cnts[parents[pos]] += remove;
                        }

                        let next_grundy = grundy(&State { cnts: new_cnts }, cached, parents);
                        seen.insert(next_grundy);
                    }
                }
            }
            for not_seen in 0..std::usize::MAX {
                if seen.contains(&not_seen) {
                    continue;
                }
                cached.insert(s.clone(), not_seen);
                return not_seen;
            }
            unreachable!();
        }
    }
}

fn _test() {
    let mut cached: HashMap<State, usize> = HashMap::new();
    const MAX: usize = 4;
    let parents = vec![0, 0, 1, 1, 1];
    for x in 0..MAX {
        for y in 0..MAX {
            for z in 0..MAX {
                for w in 0..MAX {
                    for t in 0..MAX {
                        let cnts = vec![x, y, z, w, t];
                        let grundy = grundy(&State { cnts: cnts.clone() }, &mut cached, &parents);
                        let expected = x ^ z ^ w ^ t;
                        dbg!(cnts, grundy, expected);
                        assert_eq!(grundy, expected);
                    }
                }
            }
        }
    }
}

fn calc_xor_down(v: usize, p: usize, g: &Vec<Vec<usize>>, xor_down: &mut Vec<Vec<i32>>, a: &[i32]) {
    let n_layers = xor_down[v].len();
    xor_down[v][0] ^= a[v];
    for &to in &g[v] {
        if to == p {
            continue;
        }
        calc_xor_down(to, v, g, xor_down, a);
        for layer in 0..n_layers {
            xor_down[v][(layer + 1) % n_layers] ^= xor_down[to][layer];
        }
    }
}


fn calc_xor_whole(v: usize, p: usize, g: &Vec<Vec<usize>>, xor_down: &Vec<Vec<i32>>, xor_whole: &mut Vec<Vec<i32>>) {
    let n_layers = xor_down[v].len();
    for layer in 0..n_layers {
        xor_whole[v][layer] ^= xor_down[v][layer];
    }
    if p != v {
        for layer in 0..n_layers {
            let without_me = xor_whole[p][layer] ^ xor_down[v][(layer + n_layers - 1) % n_layers];
            xor_whole[v][(layer + 1) % n_layers] ^= without_me;
        }
    }
    for &to in &g[v] {
        if to == p {
            continue;
        }
        calc_xor_whole(to, v, g, xor_down, xor_whole);
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let k = sc.usize();
    let mut g = vec![vec![]; n];
    for _ in 1..n {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        g[fr].push(to);
        g[to].push(fr);
    }
    let a = sc.vec::<i32>(n);
    let layers = k * 2;
    let mut xor_down = vec![vec![0; layers]; n];
    calc_xor_down(0, 0, &g, &mut xor_down, &a);
    let mut xor_whole = vec![vec![0; layers]; n];
    calc_xor_whole(0, 0, &g, &xor_down, &mut xor_whole);
    let mut res = vec![0; n];
    for v in 0..n {
        for layer in k..(2 * k) {
            res[v] ^= xor_whole[v][layer];
        }
    }
    for &xor in &res {
        let first_win = if xor == 0 { 0 } else { 1 };
        write!(out, "{} ", first_win).unwrap();
    }
}
