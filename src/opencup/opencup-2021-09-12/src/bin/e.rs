use std::io;
use std::io::Write;
use std::cmp::max;
use std::collections::HashMap;

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

#[derive(Copy, Clone, Debug)]
struct Edge {
    fr: usize,
    to: usize,
}

#[derive(Debug)]
struct OldValue {
    v: usize,
    old_p: usize,
    old_ans: i32,
}

#[derive(Debug)]
struct Dsu {
    p: Vec<usize>,
    ans: Vec<i32>,
    history: Vec<OldValue>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        Self { p, ans: vec![0; n], history: vec![] }
    }

    fn save(&mut self, v: usize) {
        self.history.push(OldValue { v, old_p: self.p[v], old_ans: self.ans[v] });
    }

    fn get(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        } else {
            self.save(v);
            self.p[v] = self.get(self.p[v]);
            return self.p[v];
        }
    }

    fn get_ans(&mut self, v: usize) -> i32 {
        let v = self.get(v);
        self.ans[v]
    }

    fn update_ans(&mut self, v: usize, ans: i32) {
        let v = self.get(v);
        self.save(v);
        self.ans[v] = max(self.ans[v], ans);
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        self.save(x);
        self.save(y);
        self.p[x] = y;
        self.ans[y] = max(self.ans[x], self.ans[y]);
    }

    fn restore(&mut self, pos: usize) {
        while self.history.len() != pos {
            let old_value = self.history.pop().unwrap();
            self.p[old_value.v] = old_value.old_p;
            self.ans[old_value.v] = old_value.old_ans;
        }
    }

    fn get_pos(&self) -> usize {
        self.history.len()
    }
}

struct State {
    all_cur_levels: Vec<i32>,
    ans: Vec<i32>,
    vertices_by_cur_lvl: HashMap<i32, Vec<usize>>,
    dsu: Dsu,
    cur_level: Vec<i32>,
    max_ok_level: Vec<i32>,
    how_good: Vec<i32>,
}

fn dfs(state: &mut State, l: usize, r: usize, edges: &Vec<Edge>, vertices: &Vec<usize>) {
    let real_from_lvl = state.all_cur_levels[l];
    let real_to_lvl = state.all_cur_levels[r - 1] + 1;
    let cur_dsu_pos = state.dsu.get_pos();

    let mut next_edges = vec![];
    for edge in edges.iter() {
        if state.max_ok_level[edge.fr] < real_from_lvl || state.max_ok_level[edge.to] < real_from_lvl {
            continue;
        }
        if state.max_ok_level[edge.fr] < real_to_lvl - 1 || state.max_ok_level[edge.to] < real_to_lvl - 1 {
            next_edges.push(edge.clone());
            continue;
        }
        if state.cur_level[edge.fr] <= real_from_lvl && state.cur_level[edge.to] <= real_from_lvl {
            state.dsu.unite(edge.fr, edge.to);
        } else if state.cur_level[edge.fr] <= real_from_lvl && state.cur_level[edge.to] >= real_to_lvl {
            let cur_ans = state.ans[edge.to];
            assert_ne!(cur_ans, 0);
            state.dsu.update_ans(edge.fr, cur_ans);
        } else if state.cur_level[edge.fr] >= real_to_lvl {
            // not interesting any more
        } else {
            next_edges.push(edge.clone());
        }
    }

    let mut next_vertices = vec![];
    for &v in vertices.iter() {
        if state.max_ok_level[v] >= real_to_lvl - 1 {
            state.dsu.update_ans(v, state.how_good[v]);
        } else if state.max_ok_level[v] >= real_from_lvl {
            next_vertices.push(v);
        }
    }

    if real_from_lvl + 1 == real_to_lvl {
        assert_eq!(next_vertices.len(), 0);
        for &v in state.vertices_by_cur_lvl[&real_from_lvl].iter() {
            state.ans[v] = state.dsu.get_ans(v);
        }
    } else {
        let m = (l + r) >> 1;
        dfs(state, m, r, &next_edges, &next_vertices);
        dfs(state, l, m, &next_edges, &next_vertices);
    }

    state.dsu.restore(cur_dsu_pos);
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let mut cur_level = vec![0; n];
    let mut max_ok_level = vec![0; n];
    let mut how_good = vec![0; n];
    for i in 0..n {
        cur_level[i] = sc.i32();
        max_ok_level[i] = sc.i32();
        assert!(cur_level[i] <= max_ok_level[i]);
        how_good[i] = sc.i32();
    }
    let mut edges = vec![];
    for _ in 0..m {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        if cur_level[fr] <= cur_level[to] {
            edges.push(Edge { fr, to });
        } else {
            edges.push(Edge { fr: to, to: fr });
        }
    }
    let mut vertices_by_cur_lvl: HashMap<i32, Vec<usize>> = HashMap::new();
    for v in 0..n {
        vertices_by_cur_lvl.entry(cur_level[v]).or_default().push(v);
    }
    let mut all_cur_levels: Vec<_> = cur_level.iter().cloned().collect();
    all_cur_levels.sort();
    let mut state = State { all_cur_levels, ans: vec![0; n], vertices_by_cur_lvl, dsu: Dsu::new(n), cur_level, max_ok_level, how_good };
    let r = state.all_cur_levels.len();
    dfs(&mut state, 0, r, &edges, &(0..n).collect());
    for v in 0..n {
        assert_ne!(state.ans[v], 0);
        write!(out, "{} ", state.ans[v]).unwrap();
    }
    writeln!(out).unwrap()
}
