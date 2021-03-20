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

mod flow {
    use std::collections::VecDeque;
    use std::cmp::min;

    struct Edge {
        fr: usize,
        to: usize,
        flow: i64,
        cap: i64,
    }

    pub struct Dinic {
        edges: Vec<Edge>,
        graph: Vec<Vec<usize>>,
        pub n: usize,
    }

    impl Dinic {
        pub fn new(n: usize) -> Self {
            let graph = vec![Vec::new(); n];
            let edges = vec![];
            Self { edges, graph, n }
        }

        pub fn add_edge(&mut self, fr: usize, to: usize, cap: i64) -> usize {
            let id = self.edges.len();
            let e1 = Edge { fr, to, flow: 0, cap };
            let e2 = Edge { fr: to, to: fr, flow: 0, cap: 0};
            self.graph[fr].push(id);
            self.graph[to].push(id ^ 1);
            self.edges.push(e1);
            self.edges.push(e2);
            id
        }

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

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let m = sc.usize();
    let a = sc.vec::<i64>(n);

    let mut flow = flow::Dinic::new(1 + n + m + 1);
    for i in 0..n {
        flow.add_edge(0, 1 + i, a[i]);
    }
    let mut res = 0;
    for i in 0..m {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        let cost = sc.i64();
        flow.add_edge(1 + fr, 1 + n + i, std::i64::MAX);
        flow.add_edge(1 + to, 1 + n + i, std::i64::MAX);
        flow.add_edge(1 + n + i, flow.n - 1, cost);
        res += cost;
    }
    res -= flow.find_flow();
    writeln!(out, "{}", res).unwrap();
}
