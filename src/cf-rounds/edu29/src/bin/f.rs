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

mod min_cost_max_flow {
    use std::collections::BinaryHeap;
    use std::cmp::{min, Ordering};

    struct Edge {
        flow: i64,
        cap: i64,
        cost: i64,
        fr: usize,
        to: usize,
    }

    pub struct MinCostMaxFlow {
        graph: Vec<Vec<usize>>,
        edges: Vec<Edge>,
        pub n: usize,
    }

    #[derive(Default)]
    pub struct CostAndFlow {
        pub cost: i64,
        pub flow: i64,
    }

    #[derive(Ord, Eq, PartialEq, Copy, Clone, Debug)]
    struct Vertex {
        dist: i64,
        v: usize,
        edge_id: usize,
    }

    impl PartialOrd for Vertex {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.dist.cmp(&other.dist).reverse())
        }
    }

    impl MinCostMaxFlow {
        const INF: i64 = std::i64::MAX;

        pub fn new(n: usize) -> Self {
            let graph = vec![Vec::new(); n];
            let edges = vec![];
            Self { graph, edges, n }
        }

        pub fn add_edge(&mut self, fr: usize, to: usize, cap: i64, cost: i64) -> usize {
            let id = self.edges.len();
            let e1 = Edge { fr, to, flow: 0, cap, cost };
            let e2 = Edge { to: fr, fr: to, flow: 0, cap: 0, cost: -cost };
            self.edges.push(e1);
            self.edges.push(e2);
            self.graph[fr].push(id);
            self.graph[to].push(id ^ 1);
            id
        }

        fn dijkstra(&mut self, source: usize, was: &mut [bool], h: &[i64], vertices: &mut [Vertex]) {
            for x in vertices.iter_mut() {
                x.dist = Self::INF;
            }
            for x in was.iter_mut() {
                *x = false;
            }
            vertices[source] = Vertex { dist: 0, v: source, edge_id: 0 };

            let mut heap = BinaryHeap::new();
            heap.push(vertices[source]);
            while let Some(vertex) = heap.pop() {
                if was[vertex.v] {
                    continue;
                }
                was[vertex.v] = true;
                for &e_id in &self.graph[vertex.v] {
                    let e = &self.edges[e_id];
                    if e.flow >= e.cap {
                        continue;
                    }
                    let edge_cost = e.cost + h[e.fr] - h[e.to];
                    assert!(edge_cost >= 0);
                    let new_dist = vertices[e.fr].dist + e.cost + h[e.fr] - h[e.to];
                    if vertices[e.to].dist > new_dist {
                        assert!(!was[e.to]);
                        vertices[e.to] = Vertex { v: e.to, edge_id: e_id, dist: new_dist };
                        heap.push(vertices[e.to]);
                    }
                }
            }
        }

        pub fn get_min_cost_max_flow(&mut self, source: usize, target: usize) -> CostAndFlow {
            let mut h = vec![0; self.n];
            loop {
                let mut changed = false;
                for e in &self.edges {
                    if e.cap > 0 && h[e.to] > h[e.fr] + e.cost {
                        h[e.to] = h[e.fr] + e.cost;
                        changed = true;
                    }
                }
                if !changed {
                    break;
                }
            }
            let mut was = vec![false; self.n];
            let mut ans = CostAndFlow::default();
            let mut vertices: Vec<Vertex> = (0..self.n).map(|v| Vertex { dist: 0, v, edge_id: 0 }).collect();
            loop {
                self.dijkstra(source, &mut was, &h, &mut vertices);
                if vertices[target].dist == Self::INF {
                    break;
                }
                let mut cur_v = target;
                let mut add_flow = Self::INF;
                while cur_v != source {
                    let e_id = vertices[cur_v].edge_id;
                    add_flow = min(add_flow, self.edges[e_id].cap - self.edges[e_id].flow);
                    cur_v = self.edges[e_id].fr;
                }
                assert!(add_flow > 0);
                let path_cost = vertices[target].dist + h[target] - h[source];
                ans.cost += path_cost * add_flow;
                ans.flow += add_flow;

                cur_v = target;
                while cur_v != source {
                    let e_id = vertices[cur_v].edge_id;
                    self.edges[e_id].flow += add_flow;
                    self.edges[e_id ^ 1].flow -= add_flow;
                    cur_v = self.edges[e_id].fr;
                }
                for i in 0..self.n {
                    if vertices[i].dist != Self::INF {
                        h[i] += vertices[i].dist;
                    }
                }
            }
            ans
        }
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let q = sc.usize();

    let mut at_least = vec![1; n];
    let mut at_most = vec![n; n];
    for _ in 0..q {
        let query_type = sc.usize();
        let fr = sc.usize() - 1;
        let to = sc.usize();
        let val = sc.usize();
        if query_type == 1 {
            for x in at_least[fr..to].iter_mut() {
                *x = max(*x, val)
            }
        } else {
            for x in at_most[fr..to].iter_mut() {
                *x = min(*x, val)
            }
        }
    }
    let mut graph = min_cost_max_flow::MinCostMaxFlow::new(1 + n + n + 1);
    for i in 0..n {
        graph.add_edge(0, i + 1, 1, 0);
        for j in 0..n {
            if j >= at_least[i] - 1 && j <= at_most[i] - 1 {
                graph.add_edge(i + 1, 1 + n + j, 1, 0);
            }
        }
        for cnt in 1..=n {
            let cost = cnt * cnt - (cnt - 1) * (cnt - 1);
            graph.add_edge(1 + n + i, graph.n - 1, 1, cost as i64);
        }
    }
    let ans = graph.get_min_cost_max_flow(0, graph.n - 1);
    if ans.flow == n as i64 {
        writeln!(out, "{}", ans.cost).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
