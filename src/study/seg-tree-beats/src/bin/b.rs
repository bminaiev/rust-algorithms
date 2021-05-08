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

trait LazySegTreeNodeSpec: Clone + Default {
    fn unite(l: &Self, r: &Self) -> Self;

    fn apply_update(node: &mut Self, update: &Self::Update) -> bool;
    fn join_updates(current: &mut Self::Update, add: &Self::Update);

    fn could_change(node: &Self, update: &Self::Update) -> bool;

    type Update: Clone;
}

#[allow(unused)]
struct LazySegTree<T: LazySegTreeNodeSpec> {
    n: usize,
    tree: Vec<T>,
    updates_to_push: Vec<Option<T::Update>>,
}

#[allow(unused)]
impl<T: LazySegTreeNodeSpec> LazySegTree<T> {
    fn new(init_val: &T, n: usize) -> Self {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let updates_to_push = vec![None; 2 * n - 1];
        let mut res = LazySegTree {
            n,
            tree,
            updates_to_push,
        };
        res.build(0, 0, n, init_val);
        res
    }

    fn new_f(n: usize, f: &dyn Fn(usize) -> T) -> Self {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let updates_to_push = vec![None; 2 * n - 1];
        let mut res = LazySegTree {
            n,
            tree,
            updates_to_push,
        };
        res.build_f(0, 0, n, f);
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

    fn build_f(&mut self, v: usize, l: usize, r: usize, f: &dyn Fn(usize) -> T) {
        if l + 1 == r {
            self.tree[v] = f(l);
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.build_f(v + 1, l, m, f);
            self.build_f(vr, m, r, f);
            self.pull(v, vr);
        }
    }

    fn push(&mut self, v: usize, l: usize, r: usize) {
        let update = self.updates_to_push[v].clone();
        self.updates_to_push[v] = None;
        match update {
            None => {}
            Some(update) => {
                let m = (l + r) >> 1;
                self.apply_update(v + 1, l, m, &update);
                self.apply_update(v + ((r - l) & !1), m, r, &update);
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

    fn apply_update(&mut self, v: usize, l: usize, r: usize, update: &T::Update) -> bool {
        if !T::apply_update(&mut self.tree[v], update) {
            return false;
        }
        if r - l > 1 {
            Self::join_updates(&mut self.updates_to_push[v], update);
        }
        true
    }

    fn modify_(&mut self, v: usize, l: usize, r: usize, ql: usize, qr: usize, update: &T::Update) {
        assert!(qr >= l);
        assert!(ql < r);
        if !T::could_change(&self.tree[v], update) {
            return;
        }
        if ql <= l && r <= qr {
            if self.apply_update(v, l, r, update) {
                return;
            }
            assert!(r - l > 1);
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

#[derive(Copy, Clone, Default)]
struct Node {
    max: i64,
    cnt_max: i64,
    second_max: i64,
    sum: i64,
    len: usize,
}

#[derive(Copy, Clone)]
enum Update {
    MinWith(i64),
}

impl LazySegTreeNodeSpec for Node {
    fn unite(l: &Self, r: &Self) -> Self {
        let nmax = max(l.max, r.max);
        let cnt_max = if l.max == nmax { l.cnt_max } else { 0 }
            + if r.max == nmax { r.cnt_max } else { 0 };
        let second_max = {
            if l.max == r.max {
                max(l.second_max, r.second_max)
            } else if l.max > r.max {
                max(l.second_max, r.max)
            } else {
                max(l.max, r.second_max)
            }
        };
        Self {
            max: nmax,
            cnt_max,
            second_max,
            sum: l.sum + r.sum,
            len: l.len + r.len,
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) -> bool {
        return match update {
            Update::MinWith(x) => {
                if *x <= node.second_max {
                    return false;
                }
                if *x >= node.max {
                    return true;
                }
                node.sum -= (node.max - *x) * node.cnt_max;
                node.max = *x;
                true
            }
        };
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        match (current, add) {
            (Update::MinWith(x), Update::MinWith(y)) => {
                *x = min(*x, *y);
            }
        }
    }

    fn could_change(node: &Self, update: &Self::Update) -> bool {
        match update {
            Update::MinWith(x) => node.max > *x
        }
    }

    type Update = Update;
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let a = sc.vec::<i64>(n);
    let m = sc.usize();
    let mut seg_tree = LazySegTree::new_f(n, &|pos| Node {
        max: a[pos],
        cnt_max: 1,
        second_max: -1,
        sum: a[pos],
        len: 1,
    });
    for _ in 0..m {
        let type_ = sc.usize();
        match type_ {
            1 => {
                let l = sc.usize() - 1;
                let r = sc.usize();
                let x = sc.i64();
                seg_tree.modify(l, r, Update::MinWith(x))
            }
            2 => {
                let l = sc.usize() - 1;
                let r = sc.usize();
                writeln!(out, "{}", seg_tree.get(l, r).sum).unwrap();
            }
            _ => unreachable!()
        }
    }
}
