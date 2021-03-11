use std::io;
use std::io::Write;
use std::cmp::min;

pub fn main() {
    let mut sc = Scanner::default();
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let n: usize = sc.next();
    let m: usize = sc.next();
    let mut dist = vec![vec![std::i32::MAX / 3; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        let fr = sc.next::<usize>() - 1;
        let to = sc.next::<usize>() - 1;
        dist[fr][to] = 1;
        dist[to][fr] = 1;
        graph[fr].push(to);
        graph[to].push(fr);
    }
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                dist[y][z] = min(dist[y][x] + dist[x][z], dist[y][z]);
            }
        }
    }
    for root in 0..n {
        let mut layers = vec![Vec::new(); n];
        for v in 0..n {
            let d = dist[root][v] as usize;
            layers[d].push(v);
        }
        for another_root in 0..n {
            let mut ways = 1;
            for height in 1..n {
                let mut cnt_on_path = 0;
                for &v in layers[height].iter() {
                    let on_path = dist[root][v] + dist[v][another_root] == dist[root][another_root];
                    cnt_on_path += if on_path { 1 } else { 0 };
                    if !on_path {
                        let mut cnt_possible_prev = 0;
                        for &possible_prev in graph[v].iter() {
                            if dist[root][possible_prev] + 1 == dist[root][v] && dist[another_root][possible_prev] + 1 == dist[another_root][v] {
                                cnt_possible_prev += 1;
                            }
                        }
                        ways = mul(ways, cnt_possible_prev);
                    }
                }
                if height <= dist[root][another_root] as usize {
                    if cnt_on_path != 1 {
                        ways = 0;
                    }
                } else {
                    if cnt_on_path != 0 {
                        ways = 0;
                    }
                }
                if ways == 0 {
                    break;
                }
            }
            write!(out, "{} ", ways).unwrap();
        }
        writeln!(out).unwrap();
    }
}

const MOD: i32 = 998244353;

fn add(x: i32, y: i32) -> i32 {
    let res = x + y;
    if res >= MOD { res - MOD } else { res }
}

fn mul(x: i32, y: i32) -> i32 {
    ((x as i64) * (y as i64) % (MOD as i64)) as i32
}


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