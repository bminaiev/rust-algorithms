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

struct Dsu {
    p: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        Self { p }
    }

    fn get(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        } else {
            self.p[v] = self.get(self.p[v]);
            return self.p[v];
        }
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        self.p[x] = y;
    }
}

mod flow {
    use std::cmp::min;
    use std::collections::VecDeque;

    #[allow(dead_code)]
    struct Edge {
        fr: usize,
        to: usize,
        flow: i64,
        cap: i64,
    }

    #[allow(dead_code)]
    pub struct Dinic {
        edges: Vec<Edge>,
        graph: Vec<Vec<usize>>,
        pub n: usize,
    }

    impl Dinic {
        #[allow(dead_code)]
        pub fn new(n: usize) -> Self {
            let graph = vec![Vec::new(); n];
            let edges = vec![];
            Self { edges, graph, n }
        }

        #[allow(dead_code)]
        pub fn add_edge(&mut self, fr: usize, to: usize, cap: i64) -> usize {
            let id = self.edges.len();
            let e1 = Edge {
                fr,
                to,
                flow: 0,
                cap,
            };
            let e2 = Edge {
                fr: to,
                to: fr,
                flow: 0,
                cap: 0,
            };
            self.graph[fr].push(id);
            self.graph[to].push(id ^ 1);
            self.edges.push(e1);
            self.edges.push(e2);
            id
        }

        #[allow(dead_code)]
        fn bfs(&self, source: usize, height: &mut [usize], queue: &mut VecDeque<usize>) {
            queue.clear();
            for x in height.iter_mut() {
                *x = std::usize::MAX;
            }
            height[source] = 0;
            queue.push_back(source);
            while let Some(v) = queue.pop_front() {
                for &e_id in &self.graph[v] {
                    let edge = &self.edges[e_id];
                    if edge.flow == edge.cap {
                        continue;
                    }
                    if height[edge.to] != std::usize::MAX {
                        continue;
                    }
                    height[edge.to] = height[edge.fr] + 1;
                    queue.push_back(edge.to);
                }
            }
        }

        #[allow(dead_code)]
        fn dfs(
            &mut self,
            height: &[usize],
            v: usize,
            target: usize,
            cur_flow: i64,
            iter: &mut [usize],
        ) -> i64 {
            if target == v || cur_flow == 0 {
                return cur_flow;
            }
            while iter[v] < self.graph[v].len() {
                let e_id = self.graph[v][iter[v]];
                iter[v] += 1;
                let e = &self.edges[e_id];
                if height[e.to] != height[e.fr] + 1 || e.flow == e.cap {
                    continue;
                }
                let to = e.to;
                let next_flow = min(cur_flow, e.cap - e.flow);
                let add = self.dfs(height, to, target, next_flow, iter);
                if add == 0 {
                    continue;
                }
                self.edges[e_id].flow += add;
                self.edges[e_id ^ 1].flow -= add;
                return add;
            }
            return 0;
        }

        #[allow(dead_code)]
        pub fn find_flow(&mut self) -> i64 {
            let source = 0;
            let target = self.n - 1;
            let mut res = 0;
            let mut height = vec![0; self.n];
            let mut queue = VecDeque::new();
            let mut iter = vec![0; self.n];
            loop {
                self.bfs(source, &mut height, &mut queue);
                if height[target] == std::usize::MAX {
                    break;
                }
                for x in iter.iter_mut() {
                    *x = 0;
                }
                loop {
                    let pushed = self.dfs(&height, source, target, std::i64::MAX, &mut iter);
                    if pushed == 0 {
                        break;
                    }
                    res += pushed;
                }
            }
            res
        }
    }
}

use flow::*;

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let s: Vec<_> = (0..n).map(|_| sc.string()).collect();
    let t: Vec<_> = (0..n).map(|_| sc.string()).collect();

    let mut ids = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            ids[i][j] = i * m + j;
        }
    }

    let mut dsu = Dsu::new(n * m);
    for i in 0..n {
        for j in 0..m {
            if i + 1 < n {
                if s[i][j] == s[i + 1][j] {
                    dsu.unite(ids[i][j], ids[i + 1][j]);
                }
            }
            if j + 1 < m {
                if s[i][j] == s[i][j + 1] {
                    dsu.unite(ids[i][j], ids[i][j + 1]);
                }
            }
        }
    }

    let mut flow = Dinic::new(2 + n * m);
    let mut sizes = vec![0; n * m];
    for i in 0..n * m {
        sizes[dsu.get(i)] += 1;
    }
    let mut cur_same_now = vec![0; n * m];
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == t[i][j] {
                cur_same_now[dsu.get(ids[i][j])] += 1;
            }
        }
    }
    let mut flow_id = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            flow_id[i][j] = 1 + dsu.get(ids[i][j]);л
        }
    }
    let mut max_sum_deltas = 0;
    let mut score_without_changes = 0;
    for i in 0..n {
        for j in 0..m {
            let id = ids[i][j];
            if dsu.get(id) == id {
                let changed_same = sizes[id] - cur_same_now[id];
                score_without_changes += cur_same_now[id];
                if changed_same > cur_same_now[id] {
                    let diff = changed_same - cur_same_now[id];
                    if s[i][j] == b'0' {
                        flow.add_edge(0, flow_id[i][j], diff as i64);
                    } else {
                        flow.add_edge(flow_id[i][j], flow.n - 1, diff as i64);
                    }
                    max_sum_deltas += diff;
                }
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            if i + 1 < n {
                if s[i][j] != s[i + 1][j] {
                    if s[i][j] == b'0' {
                        flow.add_edge(flow_id[i][j], flow_id[i + 1][j], i64::MAX);
                    } else {
                        flow.add_edge(flow_id[i + 1][j], flow_id[i][j], i64::MAX);
                    }
                }
            }
            if j + 1 < m {
                if s[i][j] != s[i][j + 1] {
                    if s[i][j] == b'0' {
                        flow.add_edge(flow_id[i][j], flow_id[i][j + 1], i64::MAX);
                    } else {
                        flow.add_edge(flow_id[i][j + 1], flow_id[i][j], i64::MAX);
                    }
                }
            }
        }
    }

    let max_flow = flow.find_flow();
    let max_more = max_sum_deltas - max_flow as usize;
    let max_same = score_without_changes + max_more;
    writeln!(out, "{}", n * m - max_same).unwrap();
}
