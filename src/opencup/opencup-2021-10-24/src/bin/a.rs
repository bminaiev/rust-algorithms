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

mod flow {
    use std::collections::VecDeque;
    use std::cmp::min;

    #[allow(dead_code)]
    pub struct Edge {
        fr: usize,
        to: usize,
        pub(crate) flow: i64,
        cap: i64,
    }

    #[allow(dead_code)]
    pub struct Dinic {
        pub(crate) edges: Vec<Edge>,
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
            let e1 = Edge { fr, to, flow: 0, cap };
            let e2 = Edge { fr: to, to: fr, flow: 0, cap: 0 };
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
        fn dfs(&mut self, height: &[usize], v: usize, target: usize, cur_flow: i64, iter: &mut [usize]) -> i64 {
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

use flow::Dinic;


/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

fn each_row_cnt(a: &[Vec<i64>], total_cnt: i64) -> Vec<i64> {
    let n = a.len();
    let mut res = vec![0; n];
    let mut pref_sum = 0;

    for r in 0..(n - 1) {
        let diff = a[r][0] + a[r][n - 1] - a[r + 1][0] - a[r + 1][n - 1];
        assert_eq!(diff % 2, 0);
        let cnt_down2 = diff / 2 + total_cnt;
        assert_eq!(cnt_down2 % 2, 0);
        res[r] = total_cnt - (cnt_down2 / 2) - pref_sum;
        assert!(res[r] >= 0);
        pref_sum += res[r];
    }
    res[n - 1] = total_cnt - pref_sum;
    assert!(res[n - 1] >= 0);
    res
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            a[i][j] = sc.i64();
        }
    }
    let sum_corners = a[0][0] + a[0][n - 1] + a[n - 1][0] + a[n - 1][n - 1];
    let total_cnt = sum_corners / (4 * (n - 1) as i64);
    let each_row = each_row_cnt(&a, total_cnt);
    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = a[j][i];
        }
    }
    let each_col = each_row_cnt(&b, total_cnt);

    let mut flow = Dinic::new(1 + n + n + 1);
    for r in 0..n {
        flow.add_edge(0, 1 + r, each_row[r]);
    }
    let mut edge_ids = vec![vec![0; n]; n];
    for r in 0..n {
        for c in 0..n {
            edge_ids[r][c] = flow.add_edge(1 + r, 1 + n + c, i64::MAX);
        }
    }
    for c in 0..n {
        flow.add_edge(1 + n + c, 1 + n + n, each_col[c]);
    }
    assert_eq!(flow.find_flow(), total_cnt);
    for r in 0..n {
        for c in 0..n {
            write!(out, "{} ", flow.edges[edge_ids[r][c]].flow).unwrap();
        }
        writeln!(out).unwrap();
    }
}
