use std::cmp::min;
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

#[derive(Copy, Clone)]
struct Edge {
    to: usize,
    color: usize,
}

struct State {
    used: Vec<i32>,
    best_cost: Vec<i32>,
    mult: Vec<i32>,
}

fn dfs(v: usize, graph: &Vec<Vec<Edge>>, state: &mut State, cur_cost: i32) {
    state.best_cost[v] = min(state.best_cost[v], cur_cost);
    for e in graph[v].iter() {
        state.used[e.color] += 1;
        dfs(
            e.to,
            graph,
            state,
            cur_cost + state.used[e.color] * state.mult[e.color],
        );
        state.used[e.color] -= 1;
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let mult = sc.vec::<i32>(m);

    let mut g = vec![vec![]; n];

    for _ in 0..m {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        let color = sc.usize() - 1;
        g[fr].push(Edge { to, color });
        g[to].push(Edge { to: fr, color });
    }

    let mut dist = vec![n; n];
    dist[0] = 0;
    loop {
        let mut changed = false;
        for i in 0..n {
            for e in g[i].iter() {
                if dist[e.to] > dist[i] + 1 {
                    dist[e.to] = dist[i] + 1;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    for i in 0..n {
        assert_ne!(dist[i], n);
    }

    let mut g2 = vec![vec![]; n];
    for v in 0..n {
        for &e in g[v].iter() {
            if dist[e.to] == dist[v] + 1 {
                g2[v].push(e);
            }
        }
    }

    let mut state = State {
        best_cost: vec![i32::MAX; n],
        mult,
        used: vec![0; m],
    };
    dfs(0, &g2, &mut state, 0);

    for v in 1..n {
        writeln!(out, "{}", state.best_cost[v]).unwrap();
    }
}
