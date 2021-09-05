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

struct Vertex {
    any: bool,
    left: usize,
    right: usize,
}

#[derive(Default)]
struct Tree {
    vertices: Vec<Vertex>,
}

const EMPTY: usize = 0;

impl Tree {
    fn new() -> Self {
        let mut res = Self { vertices: vec![] };
        res.build_empty();
        res
    }
    fn new_node(&mut self, any: bool) -> usize {
        self.vertices.push(Vertex { any, left: EMPTY, right: EMPTY });
        self.vertices.len() - 1
    }

    fn build_tree(&mut self, left: usize, right: usize) -> usize {
        let node = self.new_node(true);
        if right - left > 1 {
            let mid = (left + right) >> 1;
            self.vertices[node].left = self.build_tree(left, mid);
            self.vertices[node].right = self.build_tree(mid, right);
        }
        return node;
    }

    fn build_empty(&mut self) -> usize {
        self.new_node(false)
    }

    fn add(&mut self, v: usize, from: usize, l: usize, r: usize, need_l: usize, need_r: usize) -> usize {
        if !self.vertices[from].any {
            return from;
        }
        if need_l >= r || need_r <= l {
            return v;
        }
        if need_l <= l && need_r >= r {
            return from;
        }
        let mid = (l + r) >> 1;
        let left_ch = self.add(self.vertices[v].left, self.vertices[from].left, l, mid, need_l, need_r);
        let right_ch = self.add(self.vertices[v].right, self.vertices[from].right, mid, r, need_l, need_r);
        let new_node = self.new_node(self.vertices[left_ch].any || self.vertices[right_ch].any);
        self.vertices[new_node].left = left_ch;
        self.vertices[new_node].right = right_ch;
        new_node
    }

    fn get(&self, v: usize, l: usize, r: usize, need_l: usize, need_r: usize) -> bool {
        if !self.vertices[v].any {
            return false;
        }
        if need_l >= r || need_r <= l {
            return false;
        }
        if l >= need_l && r <= need_r {
            return self.vertices[v].any;
        }
        let mid = (l + r) >> 1;
        let left = self.get(self.vertices[v].left, l, mid, need_l, need_r);
        let right = self.get(self.vertices[v].right, mid, r, need_l, need_r);
        left || right
    }
}


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let q = sc.usize();

    let mut tree = Tree::new();
    let mut roots = vec![EMPTY; q + 2];
    roots[0] = tree.build_tree(0, n);
    for _ in 0..q {
        let type_ = sc.usize();
        let l = sc.usize() - 1;
        let r = sc.usize();
        if type_ == 1 {
            let x = sc.usize();
            if x + 1 < roots.len() {
                roots[x + 1] = tree.add(roots[x + 1], roots[x], 0, n, l, r);
            }
        } else {
            assert_eq!(type_, 2);
            let mut left = 0;
            let mut right = roots.len();
            while right - left > 1 {
                let mid = (left + right) >> 1;
                if tree.get(roots[mid], 0, n, l, r) {
                    left = mid;
                } else {
                    right = mid;
                }
            }
            writeln!(out, "{}", left).unwrap();
        }
    }
}
