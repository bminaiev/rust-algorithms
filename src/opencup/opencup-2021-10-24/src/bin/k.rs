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

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/


#[derive(Copy, Clone)]
struct Person {
    p1: usize,
    p2: usize,
    p3: usize,
}

#[derive(Copy, Clone, Debug)]
struct Node {
    left: i32,
    right: i32,
}

#[derive(Debug)]
struct Edge {
    fr: i32,
    to: i32,
}

#[derive(Default, Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges_raw: Vec<Edge>,
    // edges: Vec<Vec<usize>>,
}

const EMPTY_NODE: Node = Node { left: 0, right: 0 };

impl Graph {
    fn add_vertex(&mut self) -> usize {
        self.nodes.push(EMPTY_NODE);
        // self.edges.push(vec![]);
        self.nodes.len() - 1
    }
}

fn add_edges(my_node: usize, v: usize, l: usize, r: usize, after: usize, graph: &mut Graph) {
    if v == 0 {
        return;
    }
    if l >= after {
        graph.edges_raw.push(Edge { fr: my_node as i32, to: v as i32 });
        return;
    }
    if r <= after {
        return;
    }
    let m = (l + r) >> 1;
    add_edges(my_node, graph.nodes[v].left as usize, l, m, after, graph);
    add_edges(my_node, graph.nodes[v].right as usize, m, r, after, graph);
}

fn add_vertex(my_node: usize, v: usize, l: usize, r: usize, pos: usize, graph: &mut Graph) -> usize {
    let cloned_v = graph.add_vertex();
    graph.nodes[cloned_v].left = graph.nodes[v].left;
    graph.nodes[cloned_v].right = graph.nodes[v].right;

    if l + 1 == r {
        graph.edges_raw.push(Edge { fr: cloned_v as i32, to: my_node as i32 });
    } else {
        let m = (l + r) >> 1;
        if m > pos {
            graph.nodes[cloned_v].left = add_vertex(my_node, graph.nodes[cloned_v].left as usize, l, m, pos, graph) as i32;
        } else {
            graph.nodes[cloned_v].right = add_vertex(my_node, graph.nodes[cloned_v].right as usize, m, r, pos, graph) as i32;
        }
    }

    cloned_v
}

struct State {
    v: i32,
    g_pos: i32,
}

fn dfs1(v: usize, used: &mut [bool], g: &ZippedGraph, order: &mut Vec<i32>, nodes: &[Node]) {
    let mut stack = vec![];
    stack.push(State { v: v as i32, g_pos: 0 });
    used[v] = true;
    while !stack.is_empty() {
        let cur_state = stack.last_mut().unwrap();
        let edges = g.edges(cur_state.v as usize);
        if edges.len() + 2 == cur_state.g_pos as usize {
            order.push(cur_state.v);
            stack.pop();
            continue;
        }
        let next = if edges.len() + 1 == cur_state.g_pos as usize {
            nodes[cur_state.v as usize].left
        } else if edges.len() == cur_state.g_pos as usize {
            nodes[cur_state.v as usize].right
        } else {
            edges[cur_state.g_pos as usize]
        } as usize;
        if used[next] || next == 0 {
            cur_state.g_pos += 1;
            continue;
        }
        used[next] = true;
        stack.push(State { v: next as i32, g_pos: 0 });
    }
}

fn dfs2(v: usize, used: &mut [bool], g_rev: &ZippedGraph, cur_comp_id: usize, comp_id: &mut [usize]) {
    let mut stack = vec![];
    stack.push(State { v: v as i32, g_pos: 0 });
    while !stack.is_empty() {
        let cur_state = stack.last_mut().unwrap();
        if g_rev.edges(cur_state.v as usize).len() == cur_state.g_pos as usize {
            comp_id[cur_state.v as usize] = cur_comp_id;
            stack.pop();
            continue;
        }
        let next = g_rev.edges(cur_state.v as usize)[cur_state.g_pos as usize] as usize;
        if used[next] {
            cur_state.g_pos += 1;
            continue;
        }
        used[next] = true;
        stack.push(State { v: next as i32, g_pos: 0 });
    }
}

struct ZippedGraph {
    pos: Vec<i32>,
    edges: Vec<i32>,
}

impl ZippedGraph {
    fn new(mut cnt: Vec<i32>) -> Self {
        for i in 0..cnt.len() - 1 {
            cnt[i + 1] += cnt[i];
        }
        cnt.push(*cnt.last().unwrap());
        Self { edges: vec![0; *cnt.last().unwrap() as usize], pos: cnt }
    }

    fn add_edge(&mut self, fr: usize, to: usize) {
        self.pos[fr] -= 1;
        self.edges[self.pos[fr] as usize] = to as i32;
    }

    fn edges(&self, v: usize) -> &[i32] {
        &self.edges[self.pos[v] as usize..self.pos[v + 1] as usize]
    }
}

fn find_strongly_connected_component(g_raw: &[Edge], n: usize, nodes: &[Node]) -> Vec<usize> {
    let mut comp_id = vec![0; n];
    let mut used = vec![false; n];

    let mut order = vec![];
    {
        let mut cnt_from = vec![0; n];
        for e in g_raw.iter() {
            cnt_from[e.fr as usize] += 1;
        }
        let mut g = ZippedGraph::new(cnt_from);
        for e in g_raw.iter() {
            g.add_edge(e.fr as usize, e.to as usize);
        }

        for v in 0..n {
            if !used[v] {
                dfs1(v, &mut used, &g, &mut order, &nodes);
            }
        }
    }

    let mut used = vec![false; n];
    let mut cur_comp_id = 0;
    {
        let mut cnt_to = vec![0; n];
        for e in g_raw.iter() {
            cnt_to[e.to as usize] += 1;
        }
        for (idx, node) in nodes.iter().enumerate() {
            if node.left != 0 {
                cnt_to[node.left as usize] += 1;
            }
            if node.right != 0 {
                cnt_to[node.right as usize] += 1;
            }
        }
        let mut graph = ZippedGraph::new(cnt_to);

        for e in g_raw.iter() {
            graph.add_edge(e.to as usize, e.fr as usize);
        }


        for (idx, node) in nodes.iter().enumerate() {
            if node.left != 0 {
                graph.add_edge(node.left as usize, idx);
            }
            if node.right != 0 {
                graph.add_edge(node.right as usize, idx);
            }
        }


        for &v in order.iter().rev() {
            if used[v as usize] {
                continue;
            }
            dfs2(v as usize, &mut used, &graph, cur_comp_id, &mut comp_id);
            cur_comp_id += 1;
        }
    }
    comp_id
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let mut a = vec![];
    for _ in 0..n {
        let p1 = sc.usize() - 1;
        let p2 = sc.usize() - 1;
        let p3 = sc.usize() - 1;
        a.push(Person { p1, p2, p3 });
    }

    let mut graph = Graph::default();
    for _ in 0..=n {
        graph.add_vertex();
    }
    for rot in 0..2 {
        let mut who = vec![0; n];
        for (idx, p) in a.iter().enumerate() {
            who[p.p1] = idx;
        }

        let mut roots = vec![0; 2];

        for pos in (0..n).rev() {
            let p_id = who[pos];

            let p = a[p_id];

            for tree_id in 0..(2 - rot) {
                let my_pos = if tree_id == 0 { p.p3 } else { p.p2 };
                add_edges(p_id + 1, roots[tree_id], 0, n, my_pos, &mut graph);
                roots[tree_id] = add_vertex(p_id + 1, roots[tree_id], 0, n, my_pos, &mut graph);
            }
        }

        for i in 0..n {
            let p1 = a[i].p1;
            let p2 = a[i].p2;
            a[i].p1 = p2;
            a[i].p2 = p1;
        }
    }
    for v in 0..graph.nodes.len() {
        if graph.nodes[v].left != 0 {
            graph.edges_raw.push(Edge { fr: v as i32, to: graph.nodes[v].left as i32 });
        }
        if graph.nodes[v].right != 0 {
            graph.edges_raw.push(Edge { fr: v as i32, to: graph.nodes[v].right as i32 });
        }
    }
    let comps = find_strongly_connected_component(&graph.edges_raw, graph.nodes.len(), &graph.nodes);
    let tc = sc.usize();
    for _ in 0..tc {
        let from = sc.usize();
        let to = sc.usize();
        let ans = if comps[from] <= comps[to] { "YES" } else { "NO" };
        writeln!(out, "{}", ans).unwrap();
    }
}
