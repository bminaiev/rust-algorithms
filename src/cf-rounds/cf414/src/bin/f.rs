use std::io;
use std::io::Write;

pub fn main() {
    let mut sc = Scanner::default();
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let n: usize = sc.next();
    let q: usize = sc.next();
    let a: Vec<usize> = (0..n).map(|_| sc.next()).collect();
    let mut seg_tree = LazySegTree::new(n, &|pos| -> Spec {
        let mut replace_with = [0usize; DIGITS];
        for i in 0..DIGITS {
            replace_with[i] = i;
        }
        let mut mult = [0; DIGITS];
        let mut cur_value = a[pos];
        let mut pw10 = 1;
        while cur_value != 0 {
            let digit = cur_value % 10;
            mult[digit] += pw10;
            cur_value /= 10;
            pw10 *= 10;
        }
        Spec { replace_with, mult }
    });
    for _ in 0..q {
        let op_type: usize = sc.next();
        if op_type == 1 {
            let l = sc.next::<usize>() - 1;
            let r = sc.next::<usize>();
            let replace: usize = sc.next();
            let with: usize = sc.next();
            seg_tree.modify(l, r, (replace, with));
        } else {
            let l = sc.next::<usize>() - 1;
            let r = sc.next::<usize>();
            let node = seg_tree.get(l, r);
            let res: i64 = (0..DIGITS).map(|d| node.mult[d] * d as i64).sum();
            writeln!(out, "{}", res).unwrap();
        }
    }
}

const DIGITS: usize = 10;

#[derive(Clone, Default)]
struct Spec {
    replace_with: [usize; DIGITS],
    mult: [i64; DIGITS],
}

impl LazySegTreeNodeSpec for Spec {
    type Update = (usize, usize);

    fn unite(l: &Self, r: &Self) -> Self {
        let mut replace_with = [0usize; DIGITS];
        for i in 0..DIGITS {
            replace_with[i] = i;
        }
        let mut mult = [0; DIGITS];
        for i in 0..DIGITS {
            mult[i] = l.mult[i] + r.mult[i];
        }
        Self { replace_with, mult }
    }

    fn push(into: &mut Self, what: &Self) {
        let mut next_mult = [0; DIGITS];
        for d in 0..DIGITS {
            next_mult[what.replace_with[d]] += into.mult[d];
            into.replace_with[d] = what.replace_with[into.replace_with[d]];
        }
        for d in 0..DIGITS {
            into.mult[d] = next_mult[d];
        }
    }

    fn clear_push(node: &mut Self) {
        for d in 0..DIGITS {
            node.replace_with[d] = d;
        }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        for d in 0..DIGITS {
            if node.replace_with[d] == update.0 {
                node.replace_with[d] = update.1;
            }
        }
        if update.0 != update.1 {
            node.mult[update.1 as usize] += node.mult[update.0 as usize];
            node.mult[update.0 as usize] = 0
        }
    }
}

trait LazySegTreeNodeSpec: Clone + Default {
    fn unite(l: &Self, r: &Self) -> Self;
    fn push(into: &mut Self, what: &Self);
    fn clear_push(node: &mut Self);
    fn apply_update(node: &mut Self, update: &Self::Update);


    type Update;
}


struct LazySegTree<T: LazySegTreeNodeSpec> {
    n: usize,
    tree: Vec<T>,
}

impl<T: LazySegTreeNodeSpec> LazySegTree<T> {
    fn new<F>(n: usize, init_foo: &F) -> Self where F: Fn(usize) -> T {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let mut res = LazySegTree { n, tree };
        res.build(0, 0, n, init_foo);
        res
    }

    fn pull(&mut self, v: usize, vr: usize) {
        self.tree[v] = T::unite(&self.tree[v + 1], &self.tree[vr]);
    }

    fn build<F>(&mut self, v: usize, l: usize, r: usize, init_foo: &F) where F: Fn(usize) -> T {
        if l + 1 == r {
            self.tree[v] = init_foo(l);
        } else {
            let m = (l + r) >> 1;
            let vr = v + ((m - l) << 1);
            self.build(v + 1, l, m, init_foo);
            self.build(vr, m, r, init_foo);
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


#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

// Scanner code is copied from Russell Emerine's solution
// http://codeforces.com/contest/1477/submission/105755265
impl Scanner {
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
    fn next_string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}