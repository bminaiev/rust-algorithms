use std::io;
use std::io::Write;
use std::cmp::min;

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

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            match &mut self.input_source {
                | InputSource::Stdin => { std::io::stdin().read_line(&mut input).expect("Failed read"); }
                | InputSource::FromFile(lines) => {
                    let line = lines.pop().unwrap();
                    input = line;
                }
            }

            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    #[allow(dead_code)]
    fn string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}


mod min_cost_max_flow {
    use std::collections::BTreeSet;
    use std::cmp::min;

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

    #[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
    struct Vertex {
        dist: i64,
        v: usize,
        edge_id: usize,
    }

    impl MinCostMaxFlow {
        const INF: i64 = std::i64::MAX;

        #[allow(dead_code)]
        pub fn new(n: usize) -> Self {
            let graph = vec![Vec::new(); n];
            let edges = vec![];
            Self { graph, edges, n }
        }

        #[allow(dead_code)]
        pub fn get_edge_flow(&self, edge_id: usize) -> i64 {
            self.edges[edge_id].flow
        }

        #[allow(dead_code)]
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

            let mut heap = BTreeSet::new();
            heap.insert(vertices[source]);

            while !heap.is_empty() {
                let vertex = heap.iter().next().unwrap().clone();
                heap.remove(&vertex);
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
                        heap.insert(vertices[e.to]);
                    }
                }
            }
        }

        #[allow(dead_code)]
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

use min_cost_max_flow::*;

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

struct Pos {
    x: usize,
    y: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for t in 0..tc {
        let n = sc.usize();
        let m = sc.usize();
        let flip_cost = sc.i64();
        let shift_cost = sc.i64();
        let mut from = vec![];
        for _ in 0..n {
            from.push(sc.string());
        }
        let mut to = vec![];
        for _ in 0..n {
            to.push(sc.string());
        }
        let mut res = std::i64::MAX;
        let mut free_m = vec![];
        let mut free_g = vec![];
        for x in 0..n {
            for y in 0..m {
                if from[x][y] == to[x][y] {
                    continue;
                }
                let pos = Pos { x, y };
                if from[x][y] == b'M' {
                    free_m.push(pos);
                } else if from[x][y] == b'G' {
                    free_g.push(pos);
                } else {
                    unreachable!();
                }
            }
        }
        for used in 0..=min(free_m.len(), free_g.len()) {
            let mut graph = MinCostMaxFlow::new(2 + free_m.len() + free_g.len() + 1);
            graph.add_edge(0, 1, used as i64, 0);
            for i in 0..free_m.len() {
                graph.add_edge(1, 2 + i, 1, 0);
            }
            for i in 0..free_g.len() {
                graph.add_edge(2 + free_m.len() + i, graph.n - 1, 1, 0);
            }
            for i in 0..free_m.len() {
                for j in 0..free_g.len() {
                    let dx = free_m[i].x as i32 - free_g[j].x as i32;
                    let dy = free_m[i].y as i32 - free_g[j].y as i32;
                    let cost = ((dx.abs() + dy.abs()) as i64) * shift_cost;
                    graph.add_edge(2 + i, 2 + free_m.len() + j, 1, cost);
                }
            }
            let flow_res = graph.get_min_cost_max_flow(0, graph.n - 1);
            assert_eq!(flow_res.flow, used as i64);
            let to_flip = free_m.len() - used + free_g.len() - used;
            let cur_res = flow_res.cost + to_flip as i64 * flip_cost;
            res = min(res, cur_res);
        }
        writeln!(out, "Case #{}: {}", t + 1, res).unwrap();
    }
}
