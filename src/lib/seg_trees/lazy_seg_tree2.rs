use std::cmp::min;

trait LazySegTreeNodeSpec: Clone + Default {
    fn unite(l: &Self, r: &Self) -> Self;
    fn push(into: &mut Self, what: &Self);
    fn clear_push(node: &mut Self);
    fn apply_update(node: &mut Self, update: &Self::Update);

    type Update;
}

#[derive(Clone, Default)]
struct PlusMin {
    push: i64,
    min: i64,
}

impl LazySegTreeNodeSpec for PlusMin {
    type Update = i64;

    fn unite(l: &Self, r: &Self) -> Self {
        Self { push: 0, min: min(l.min, r.min) }
    }

    fn push(into: &mut Self, what: &Self) {
        into.push += what.push;
        into.min += what.push;
    }

    fn clear_push(node: &mut Self) {
        node.push = 0;
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.min += update;
        node.push += update;
    }
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

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use super::*;

    #[test]
    fn stress_plus_min() {
        const MAX_N: usize = 50;
        const MAX_VAL: i32 = 1000_000;
        const TESTS_N: usize = 300;
        const OPS_IN_TEST: usize = 100;
        const DEBUG: bool = false;

        for t in 0..TESTS_N {
            let mut rnd = StdRng::seed_from_u64(787788 + t as u64);
            let n: usize = rnd.gen_range(1..=MAX_N);
            let init_val = 123;
            let init_node = PlusMin { push: 0, min: init_val };
            let mut tree = LazySegTree2::<PlusMin>::new(&init_node, n);
            let mut slow_vec = vec![init_val; n];
            if DEBUG {
                eprintln!("start test {}, n = {}", t, n);
            }
            for _ in 0..OPS_IN_TEST {
                let left = rnd.gen_range(0..n);
                let right = rnd.gen_range((left + 1)..=n);
                if rnd.gen_bool(0.5) {
                    if DEBUG {
                        eprintln!("check min for [{}..{})", left, right);
                    }
                    let sum_from_tree = tree.get(left, right).min;
                    let sum_slow = *slow_vec[left..right].iter().min().unwrap();
                    assert_eq!(sum_from_tree, sum_slow);
                } else {
                    let change = rnd.gen_range(0..MAX_VAL) as i64;
                    if DEBUG {
                        eprintln!("add [{}..{}) += {}", left, right, change);
                    }
                    tree.modify(left, right, change);
                    for v in &mut slow_vec[left..right] {
                        *v += change;
                    }
                }
            }
        }
    }


    #[test]
    fn stress_speed() {
        const MAX_N: usize = 1_000_000;
        const MAX_VAL: i32 = 1_000_000;
        const TESTS_N: usize = 10;
        const OPS_IN_TEST: usize = 1_000_000;

        for t in 0..TESTS_N {
            let mut rnd = StdRng::seed_from_u64(787788 + t as u64);
            let now = std::time::Instant::now();
            let n: usize = MAX_N;
            let init_val = PlusMin { push: 0, min: 123 };
            let mut tree = LazySegTree2::<PlusMin>::new(&init_val, n);
            for _ in 0..OPS_IN_TEST {
                let left = rnd.gen_range(0..n);
                let right = rnd.gen_range((left + 1)..=n);
                if rnd.gen_bool(0.5) {
                    tree.get(left, right);
                } else {
                    let change = rnd.gen_range(0..MAX_VAL) as i64;
                    tree.modify(left, right, change);
                }
            }
            eprintln!("done with test in {}ms", now.elapsed().as_millis());
        }
    }
}