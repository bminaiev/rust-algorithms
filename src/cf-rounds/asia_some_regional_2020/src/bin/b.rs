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


#[derive(Debug)]
enum AST {
    Concat(Box<AST>, Box<AST>),
    Or(Box<AST>, Box<AST>),
    Plus(Box<AST>),
    DigitSeq(Vec<u8>),
}

#[derive(Debug)]
struct ParseRes {
    pos: usize,
    ast: AST,
}

fn parse_atomic(regex: &Vec<u8>, pos: usize) -> ParseRes {
    let first = regex[pos];
    if first == b'(' {
        let res = parse_or(regex, pos + 1);
        assert_eq!(regex[res.pos], b')');
        ParseRes { pos: res.pos + 1, ast: res.ast }
    } else if first == b'[' {
        let mut digits = vec![];
        let mut cur_pos = pos + 1;
        while regex[cur_pos] != b']' {
            digits.push(regex[cur_pos]);
            cur_pos += 1;
        }
        ParseRes { pos: cur_pos + 1, ast: AST::DigitSeq(digits) }
    } else {
        assert!(first >= b'0' && first <= b'9');
        let ast = AST::DigitSeq(vec![first]);
        ParseRes { pos: pos + 1, ast }
    }
}

fn parse_plus(regex: &Vec<u8>, pos: usize) -> ParseRes {
    let atomic = parse_atomic(regex, pos);
    let next_pos = atomic.pos;
    if next_pos != regex.len() {
        if regex[next_pos] == b'+' {
            let mut real_next_pos = next_pos;
            while real_next_pos != regex.len() && regex[real_next_pos] == b'+' {
                real_next_pos += 1;
            }
            return ParseRes { pos: real_next_pos, ast: AST::Plus(Box::new(atomic.ast)) };
        }
    }
    return atomic;
}


fn parse_concat(regex: &Vec<u8>, pos: usize) -> ParseRes {
    let from_plus = parse_plus(regex, pos);
    let next_pos = from_plus.pos;
    if next_pos != regex.len() && regex[next_pos] != b'|' && regex[next_pos] != b')' {
        let right = parse_concat(regex, next_pos);
        ParseRes { pos: right.pos, ast: AST::Concat(Box::new(from_plus.ast), Box::new(right.ast)) }
    } else {
        from_plus
    }
}

fn parse_or(regex: &Vec<u8>, pos: usize) -> ParseRes {
    let from_concat = parse_concat(regex, pos);
    let next_pos = from_concat.pos;
    if next_pos != regex.len() && regex[next_pos] == b'|' {
        // assert_eq!(regex[next_pos], b'|');
        let right = parse_or(regex, next_pos + 1);
        ParseRes { pos: right.pos, ast: AST::Or(Box::new(from_concat.ast), Box::new(right.ast)) }
    } else {
        from_concat
    }
}

#[derive(Debug)]
struct Automata {
    start: usize,
    end: usize,
}

#[derive(Copy, Clone, Debug)]
enum EdgeType {
    EPS,
    DIGIT(u8),
}

#[derive(Copy, Clone, Debug)]
struct Edge {
    to: usize,
    type_: EdgeType,
}

#[derive(Default, Debug)]
struct Graph {
    nodes: Vec<Vec<Edge>>,
    by_dist: Vec<Vec<usize>>,
    dist: Vec<usize>,
    next_dists: Vec<usize>,
}

impl Graph {
    fn new_node(&mut self) -> usize {
        self.nodes.push(vec![]);
        self.nodes.len() - 1
    }

    fn add_edge(&mut self, from: usize, to: usize, edge: EdgeType) {
        self.nodes[from].push(Edge { type_: edge, to });
    }

    fn init_dists(&mut self, start: usize) {
        self.dist = vec![std::usize::MAX; self.nodes.len()];
        self.dist[start] = 0;
        self.next_dists = vec![std::usize::MAX; self.nodes.len()];
    }

    fn init_next_dists(&mut self) {
        for x in self.next_dists.iter_mut() {
            *x = std::usize::MAX;
        }
    }

    fn set_next_dists(&mut self) {
        for (x, y) in self.dist.iter_mut().zip(self.next_dists.iter()) {
            *x = *y;
        }
    }

    fn one_step(&mut self, c: u8) {
        self.init_next_dists();
        for v in 0..self.nodes.len() {
            if self.dist[v] == std::usize::MAX {
                continue;
            }
            self.next_dists[v] = min(self.next_dists[v], self.dist[v] + 1);
            for edge in self.nodes[v].iter() {
                match edge.type_ {
                    | EdgeType::EPS => {}
                    | EdgeType::DIGIT(digit) => {
                        let ndist = if digit == c {
                            0
                        } else {
                            1
                        } + self.dist[v];
                        self.next_dists[edge.to] = min(self.next_dists[edge.to], ndist);
                    }
                }
            }
        }
        self.set_next_dists();
        self.relax();
    }

    fn relax(&mut self) {
        for by in self.by_dist.iter_mut() {
            by.clear();
        }
        for v in 0..self.nodes.len() {
            let d = self.dist[v];
            if d == std::usize::MAX {
                continue;
            }
            while self.by_dist.len() <= d {
                self.by_dist.push(vec![]);
            }
            self.by_dist[d].push(v);
        }
        {
            let mut cur_d = 0;
            while cur_d < self.by_dist.len() {
                let mut cur_iter = 0;
                while cur_iter < self.by_dist[cur_d].len() {
                    let v = self.by_dist[cur_d][cur_iter];
                    if self.dist[v] == cur_d {
                        for edge in self.nodes[v].iter() {
                            let next_d = cur_d + match edge.type_ {
                                | EdgeType::EPS => 0,
                                | EdgeType::DIGIT(_) => 1,
                            };
                            if self.dist[edge.to] > next_d {
                                self.dist[edge.to] = next_d;
                                if self.by_dist.len() == next_d {
                                    self.by_dist.push(vec![]);
                                }
                                self.by_dist[next_d].push(edge.to);
                            }
                        }
                    }
                    cur_iter += 1;
                }
                cur_d += 1;
            }
        }
    }
}

fn build_graph(graph: &mut Graph, ast: &AST) -> Automata {
    match ast {
        | AST::Concat(lhs, rhs) => {
            let lhs = build_graph(graph, lhs);
            let rhs = build_graph(graph, rhs);
            graph.add_edge(lhs.end, rhs.start, EdgeType::EPS);
            Automata { start: lhs.start, end: rhs.end }
        }
        | AST::Or(lhs, rhs) => {
            let lhs = build_graph(graph, lhs);
            let rhs = build_graph(graph, rhs);
            let start = graph.new_node();
            let end = graph.new_node();
            graph.add_edge(start, lhs.start, EdgeType::EPS);
            graph.add_edge(start, rhs.start, EdgeType::EPS);
            graph.add_edge(lhs.end, end, EdgeType::EPS);
            graph.add_edge(rhs.end, end, EdgeType::EPS);
            Automata { start, end }
        }
        | AST::Plus(ast) => {
            let inner = build_graph(graph, ast);
            graph.add_edge(inner.end, inner.start, EdgeType::EPS);
            inner
        }
        | AST::DigitSeq(digits) => {
            let start = graph.new_node();
            let end = graph.new_node();
            for d in digits.iter() {
                graph.add_edge(start, end, EdgeType::DIGIT(*d));
            }
            Automata { start, end }
        }
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let regex = sc.string();
    let s = sc.string();

    let parse_res = parse_or(&regex, 0);
    // dbg!(parse_res);
    assert_eq!(parse_res.pos, regex.len());
    let mut graph = Graph::default();
    let automata = build_graph(&mut graph, &parse_res.ast);

    // for (id, edges) in graph.nodes.iter().enumerate() {
    //     dbg!(id, edges);
    // }
    //
    graph.init_dists(automata.start);
    graph.relax();
    for c in s.iter() {
        graph.one_step(*c);
        // dbg!(graph.dist);
    }
    let cost = graph.dist[automata.end];
    writeln!(out, "{}", cost).unwrap();
}
