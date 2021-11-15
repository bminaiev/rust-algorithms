use std::collections::BTreeSet;
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

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Vertex {
    dist: i64,
    v: usize,
}

#[derive(Copy, Clone)]
struct Edge {
    cost: i64,
    fr: usize,
    to: usize,
}

const INF: i64 = i64::MAX / 10;

fn dijkstra(source: usize, was: &mut [bool], vertices: &mut [Vertex], graph: &Vec<Vec<Edge>>) {
    for x in vertices.iter_mut() {
        x.dist = INF;
    }
    for x in was.iter_mut() {
        *x = false;
    }
    vertices[source] = Vertex { dist: 0, v: source };

    let mut heap = BTreeSet::new();
    heap.insert(vertices[source]);

    while !heap.is_empty() {
        let vertex = heap.iter().next().unwrap().clone();
        heap.remove(&vertex);
        if was[vertex.v] {
            continue;
        }
        was[vertex.v] = true;
        for e in graph[vertex.v].iter() {
            let edge_cost = e.cost;
            assert!(edge_cost >= 0);
            let new_dist = vertices[e.fr].dist + e.cost;
            if vertices[e.to].dist > new_dist {
                assert!(!was[e.to]);
                vertices[e.to] = Vertex {
                    v: e.to,
                    dist: new_dist,
                };
                heap.insert(vertices[e.to]);
            }
        }
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let c = sc.i64();

    let mut graph = vec![vec![]; n];
    let mut any = false;
    for _ in 0..m {
        let fr = sc.usize() - 1;
        let to = sc.usize() - 1;
        let cost = sc.i64();
        if cost <= c {
            any = true;
        }
        graph[fr].push(Edge { fr, to, cost });
    }

    let mut was = vec![false; n];
    let mut vertices = vec![Vertex { v: 0, dist: INF }; n];

    let mut exist_cycle = false;

    for root in 0..n {
        dijkstra(root, &mut was, &mut vertices, &graph);
        for v in 0..n {
            for edge in graph[v].iter() {
                if edge.to == root && vertices[v].dist + edge.cost <= c {
                    exist_cycle = true;
                }
            }
        }
    }

    let res = if exist_cycle {
        2
    } else if any {
        1
    } else {
        0
    };
    writeln!(out, "{}", res).unwrap();
}
