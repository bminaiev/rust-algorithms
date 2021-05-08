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

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            match &mut self.input_source {
                InputSource::Stdin => {
                    std::io::stdin().read_line(&mut input).expect("Failed read");
                }
                InputSource::FromFile(lines) => {
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

#[derive(Clone, Default, Debug)]
struct Node {
    sum_a: i64,
    sum_b: i64,
    len: i64,
}

#[derive(Clone)]
struct Update {
    cnt_times_add_a: i64,
    cnt_times_add_a_xor_1: i64,
    need_xor_a_after_all: i64,
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self) -> Self {
        Self {
            sum_a: l.sum_a + r.sum_a,
            sum_b: l.sum_b + r.sum_b,
            len: l.len + r.len,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.sum_b += node.sum_a * update.cnt_times_add_a;
        node.sum_b += (node.len - node.sum_a) * update.cnt_times_add_a_xor_1;
        if update.need_xor_a_after_all == 1 {
            node.sum_a = node.len - node.sum_a;
        }
    }

    fn join_updates(current: &mut Update, add: &Self::Update) {
        if current.need_xor_a_after_all == 0 {
            current.cnt_times_add_a += add.cnt_times_add_a;
            current.cnt_times_add_a_xor_1 += add.cnt_times_add_a_xor_1;
        } else {
            current.cnt_times_add_a += add.cnt_times_add_a_xor_1;
            current.cnt_times_add_a_xor_1 += add.cnt_times_add_a;
        }
        current.need_xor_a_after_all ^= add.need_xor_a_after_all;
    }

    type Update = Update;
}

trait LazySegTreeNodeSpec: Clone + Default {
    fn unite(l: &Self, r: &Self) -> Self;

    fn apply_update(node: &mut Self, update: &Self::Update);
    fn join_updates(current: &mut Self::Update, add: &Self::Update);

    type Update: Clone;
}

struct LazySegTree2<T: LazySegTreeNodeSpec> {
    n: usize,
    tree: Vec<T>,
    updates_to_push: Vec<Option<T::Update>>,
}

impl<T: LazySegTreeNodeSpec> LazySegTree2<T> {
    fn new(init_val: &T, n: usize) -> Self {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let updates_to_push = vec![None; 2 * n - 1];
        let mut res = LazySegTree2 {
            n,
            tree,
            updates_to_push,
        };
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
        let update = self.updates_to_push[v].clone();
        self.updates_to_push[v] = None;
        match update {
            None => {}
            Some(update) => {
                self.apply_update(v + 1, &update);
                self.apply_update(v + ((r - l) & !1), &update);
            }
        }
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
                T::unite(
                    &self.get_(v + 1, l, m, ql, qr),
                    &self.get_(vr, m, r, ql, qr),
                )
            }
        };
        self.pull(v, vr);
        res
    }

    fn join_updates(current: &mut Option<T::Update>, add: &T::Update) {
        match current {
            None => *current = Some(add.clone()),
            Some(current) => T::join_updates(current, add),
        };
    }

    fn apply_update(&mut self, v: usize, update: &T::Update) {
        T::apply_update(&mut self.tree[v], update);
        Self::join_updates(&mut self.updates_to_push[v], update);
    }

    fn modify_(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize, update: &T::Update) {
        assert!(qr >= l);
        assert!(ql < r);
        if ql <= l && r <= qr {
            self.apply_update(v, update);
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
        Self {
            seg_tree: LazySegTree2::new(&init_val, n),
            n,
        }
    }

    fn xor(&mut self, from: usize, to: usize) {
        self.seg_tree.modify(
            from,
            to,
            Update {
                cnt_times_add_a: 0,
                cnt_times_add_a_xor_1: 0,
                need_xor_a_after_all: 1,
            },
        );
    }

    fn plus(&mut self) {
        self.seg_tree.modify(
            0,
            self.n,
            Update {
                cnt_times_add_a: 1,
                cnt_times_add_a_xor_1: 0,
                need_xor_a_after_all: 0,
            },
        );
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
