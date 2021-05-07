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

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

trait LazySegTreeNodeSpec: Clone + Default {
    fn unite(l: &Self, r: &Self) -> Self;
    fn push(into: &mut Self, what: &Self);
    fn clear_push(node: &mut Self);
    fn apply_update(node: &mut Self, update: &Self::Update);

    type Update;
}


#[derive(Clone, Default, Debug)]
struct Node {
    sum_a: i64,
    sum_b: i64,
    need_push_cnt_times_add_a: i64,
    need_push_cnt_times_add_a_xor_1: i64,
    need_push_xor_1: i64,
    len: i64,
}

enum Update {
    Xor,
    BplusA,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self) -> Self {
        Self {
            sum_a: l.sum_a + r.sum_a,
            sum_b: l.sum_b + r.sum_b,
            need_push_cnt_times_add_a: 0,
            need_push_cnt_times_add_a_xor_1: 0,
            need_push_xor_1: 0,
            len: l.len + r.len,
        }
    }

    fn push(into: &mut Self, what: &Self) {
        into.sum_b += what.need_push_cnt_times_add_a * into.sum_a;
        if into.need_push_xor_1 == 0 {
            into.need_push_cnt_times_add_a += what.need_push_cnt_times_add_a;
            into.need_push_cnt_times_add_a_xor_1 += what.need_push_cnt_times_add_a_xor_1;
        } else {
            into.need_push_cnt_times_add_a += what.need_push_cnt_times_add_a_xor_1;
            into.need_push_cnt_times_add_a_xor_1 += what.need_push_cnt_times_add_a;
        }
        into.sum_b += what.need_push_cnt_times_add_a_xor_1 * (into.len - into.sum_a);
        into.need_push_xor_1 ^= what.need_push_xor_1;
        if what.need_push_xor_1 == 1 {
            into.sum_a = into.len - into.sum_a;
        }
    }

    fn clear_push(node: &mut Self) {
        node.need_push_cnt_times_add_a = 0;
        node.need_push_cnt_times_add_a_xor_1 = 0;
        node.need_push_xor_1 = 0;
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        match update {
            Update::Xor => {
                node.need_push_xor_1 ^= 1;
                node.sum_a = node.len - node.sum_a;
            }
            Update::BplusA => {
                node.sum_b += node.sum_a;
                if node.need_push_xor_1 == 0 {
                    node.need_push_cnt_times_add_a += 1;
                } else {
                    node.need_push_cnt_times_add_a_xor_1 += 1;
                }
            }
        }
    }

    type Update = Update;
}

struct LazySegTree2<T: LazySegTreeNodeSpec> {
    n: usize,
    tree: Vec<T>,
}

impl<T: LazySegTreeNodeSpec> LazySegTree2<T> {
    fn new(init_val: &T, n: usize) -> Self {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let mut res = LazySegTree2 { n, tree };
        res.build(0, 0, n, init_val);
        res
    }

    fn pull(&mut self, v: usize, vr: usize) {
        self.tree[v] = T::unite(&self.tree[v + 1], &self.tree[vr]);
    }

    fn build(&mut self, v: usize, l: usize, r: usize, init_val: &T) {
        if l + 1 == r {
            self.tree[v] = init_val.clone();
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.build(v + 1, l, m, init_val);
            self.build(vr, m, r, init_val);
            self.pull(v, vr);
        }
    }

    fn push(&mut self, v: usize, l: usize, r: usize) {
        let as_slices = self.tree.split_at_mut(v + 1);
        let to_push = as_slices.0.last_mut().unwrap();
        let right = as_slices.1;
        T::push(&mut right[0], &to_push);
        T::push(&mut right[((r - l) & !1) - 1], &to_push);
        T::clear_push(to_push);
    }

    fn get_(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> T {
        assert!(qr >= l);
        assert!(ql < r);
        if ql <= l && r <= qr {
            return self.tree[v].clone();
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);
        let res = if ql >= m {
            self.get_(vr, m, r, ql, qr)
        } else {
            if qr <= m {
                self.get_(v + 1, l, m, ql, qr)
            } else {
                T::unite(&self.get_(v + 1, l, m, ql, qr), &self.get_(vr, m, r, ql, qr))
            }
        };
        self.pull(v, vr);
        res
    }

    fn modify_(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize, update: &T::Update) {
        assert!(qr >= l);
        assert!(ql < r);
        if ql <= l && r <= qr {
            T::apply_update(&mut self.tree[v], update);
            return;
        }
        let m = (l + r) >> 1;
        let vr = v + ((m - l) << 1);
        self.push(v, l, r);
        if ql >= m {
            self.modify_(vr, m, r, ql, qr, update);
        } else {
            if qr <= m {
                self.modify_(v + 1, l, m, ql, qr, update);
            } else {
                self.modify_(v + 1, l, m, ql, qr, update);
                self.modify_(vr, m, r, ql, qr, update);
            }
        };
        self.pull(v, vr);
    }

    pub fn modify(&mut self, ql: usize, qr: usize, update: T::Update) {
        self.modify_(0, 0, self.n, ql, qr, &update);
    }

    pub fn get(&mut self, ql: usize, qr: usize) -> T {
        self.get_(0, 0, self.n, ql, qr)
    }
}


#[derive(Copy, Clone)]
struct Query {
    id: usize,
    r: usize,
}

struct Solver {
    seg_tree: LazySegTree2<Node>,
    n: usize,
}

impl Solver {
    fn create(n: usize) -> Self {
        let mut init_val = Node::default();
        init_val.len = 1;
        Self { seg_tree: LazySegTree2::new(&init_val, n), n }
    }

    fn xor(&mut self, from: usize, to: usize) {
        self.seg_tree.modify(from, to, Update::Xor);
    }

    fn plus(&mut self) {
        self.seg_tree.modify(0, self.n, Update::BplusA);
    }

    fn get(&mut self, from: usize, to: usize) -> i64 {
        self.seg_tree.get(from, to).sum_b
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let a = sc.vec::<usize>(n);
    let m = sc.usize();
    let mut queries = vec![vec![]; n];
    for id in 0..m {
        let l = sc.usize() - 1;
        let r = sc.usize();
        queries[l].push(Query { id, r });
    }
    let mut last_time = vec![n; n + 1];
    let mut solver = Solver::create(n);
    let mut results = vec![0; m];
    for left in (0..n).rev() {
        solver.xor(left, last_time[a[left]]);
        solver.plus();
        for query in queries[left].iter() {
            results[query.id] = solver.get(left, query.r);
        }
        last_time[a[left]] = left;
    }
    for &x in results.iter() {
        writeln!(out, "{}", x).unwrap();
    }
}
