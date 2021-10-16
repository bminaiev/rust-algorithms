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

mod modulo {
    const MODULO: i32 = 1_000_000_123;

    #[derive(Copy, Clone, Eq, PartialEq, Default)]
    pub struct Mod(i32);

    impl Mod {
        #[allow(unused)]
        pub const ZERO: Self = Self(0);

        #[allow(unused)]
        pub const ONE: Self = Self(1);

        fn rev_rec(a: i32, m: i32) -> i32 {
            if a == 1 {
                return a;
            }
            return ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64) as i32;
        }

        #[allow(dead_code)]
        fn inv(self) -> Mod {
            Mod(Self::rev_rec(self.0, MODULO))
        }

        #[allow(dead_code)]
        pub fn new(mut x: i32) -> Self {
            if x < 0 {
                x += MODULO;
            } else if x >= MODULO {
                x -= MODULO;
            }
            assert!(0 <= x && x < MODULO);
            Self(x)
        }
    }

    impl std::fmt::Display for Mod {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::fmt::Debug for Mod {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            const MAX: usize = 100;
            if self.0 <= MAX as i32 {
                write!(f, "{}", self.0)
            } else if self.0 >= MODULO - MAX as i32 {
                write!(f, "-{}", MODULO - self.0)
            } else {
                for denum in 1..MAX {
                    for num in 1..MAX {
                        if Mod(num as i32) / Mod(denum as i32) == *self {
                            return write!(f, "{}/{}", num, denum);
                        }
                    }
                }
                write!(f, "(?? {} ??)", self.0)
            }
        }
    }

    impl std::ops::Add for Mod {
        type Output = Mod;

        fn add(self, rhs: Self) -> Self::Output {
            let res = self.0 + rhs.0;
            if res >= MODULO {
                Mod(res - MODULO)
            } else {
                Mod(res)
            }
        }
    }

    impl std::ops::AddAssign for Mod {
        fn add_assign(&mut self, rhs: Self) {
            self.0 += rhs.0;
            if self.0 >= MODULO {
                self.0 -= MODULO;
            }
        }
    }

    impl std::ops::Sub for Mod {
        type Output = Mod;

        fn sub(self, rhs: Self) -> Self::Output {
            let res = self.0 - rhs.0;
            if res < 0 {
                Mod(res + MODULO)
            } else {
                Mod(res)
            }
        }
    }

    impl std::ops::SubAssign for Mod {
        fn sub_assign(&mut self, rhs: Self) {
            self.0 -= rhs.0;
            if self.0 < 0 {
                self.0 += MODULO;
            }
        }
    }

    impl std::ops::Mul for Mod {
        type Output = Mod;

        fn mul(self, rhs: Self) -> Self::Output {
            let res = (self.0 as i64) * (rhs.0 as i64) % (MODULO as i64);
            Mod(res as i32)
        }
    }

    impl std::ops::MulAssign for Mod {
        fn mul_assign(&mut self, rhs: Self) {
            self.0 = ((self.0 as i64) * (rhs.0 as i64) % (MODULO as i64)) as i32;
        }
    }

    impl std::ops::Div for Mod {
        type Output = Mod;

        fn div(self, rhs: Self) -> Self::Output {
            let rhs_inv = rhs.inv();
            self * rhs_inv
        }
    }

    impl std::ops::DivAssign for Mod {
        fn div_assign(&mut self, rhs: Self) {
            *self *= rhs.inv();
        }
    }
}

use modulo::*;
use std::collections::BTreeSet;

trait LazySegTreeNodeSpec: Clone + Default {
    fn unite(l: &Self, r: &Self, powers: &[Mod]) -> Self;

    fn apply_update(node: &mut Self, update: &Self::Update);
    fn join_updates(current: &mut Self::Update, add: &Self::Update);

    type Update: Clone;
}

#[allow(unused)]
struct LazySegTree<T: LazySegTreeNodeSpec> {
    n: usize,
    tree: Vec<T>,
    updates_to_push: Vec<Option<T::Update>>,
    powers: Vec<Mod>,
}

#[allow(unused)]
impl<T: LazySegTreeNodeSpec> LazySegTree<T> {
    fn new(init_val: &T, n: usize) -> Self {
        assert!(n > 0);
        let tree = vec![T::default(); 2 * n - 1];
        let updates_to_push = vec![None; 2 * n - 1];
        let mut powers = vec![Mod::ONE; n + 1];
        for i in 1..powers.len() {
            powers[i] = powers[i - 1] * Mod::new(239);
        }
        let mut res = LazySegTree {
            n,
            tree,
            updates_to_push,
            powers,
        };
        res.build(0, 0, n, init_val);
        res
    }

    fn pull(&mut self, v: usize, vr: usize) {
        self.tree[v] = T::unite(&self.tree[v + 1], &self.tree[vr], &self.powers);
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
                    &self.powers,
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
        if ql == qr {
            return T::default();
        }
        self.get_(0, 0, self.n, ql, qr)
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
}


#[derive(Clone, Default)]
struct Hash {
    hash: Mod,
    len: usize,
}

impl LazySegTreeNodeSpec for Hash {
    type Update = i32;

    fn unite(l: &Self, r: &Self, powers: &[Mod]) -> Self {
        let hash = l.hash * powers[r.len] + r.hash;
        Self { hash, len: l.len + r.len }
    }

    fn apply_update(node: &mut Self, update: &Self::Update) {
        node.hash = Mod::new(*update);
        node.len = 1;
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {}
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let mut s = sc.string();
    let mut t = sc.string();
    let tc = sc.usize();
    let mut seg_tree_s = LazySegTree::new(&Hash { hash: Mod::ZERO, len: 1 }, s.len());
    let mut seg_tree_s_rev = LazySegTree::new(&Hash { hash: Mod::ZERO, len: 1 }, s.len());
    let mut seg_tree_t = LazySegTree::new(&Hash { hash: Mod::ZERO, len: 1 }, s.len());
    let mut seg_tree_t_rev = LazySegTree::new(&Hash { hash: Mod::ZERO, len: 1 }, s.len());
    let n = s.len();
    let mut diff = BTreeSet::new();
    for i in 0..n {
        seg_tree_s.modify(i, i + 1, s[i] as i32);
        seg_tree_t.modify(i, i + 1, t[i] as i32);
        let i_rev = n - 1 - i;
        seg_tree_s_rev.modify(i_rev, i_rev + 1, s[i] as i32);
        seg_tree_t_rev.modify(i_rev, i_rev + 1, t[i] as i32);
        if s[i] != t[i] {
            diff.insert(i);
        }
    }
    let mut print_ans = |seg_tree_s: &mut LazySegTree<Hash>, seg_tree_s_rev: &mut LazySegTree<Hash>, seg_tree_t: &mut LazySegTree<Hash>, diff: &BTreeSet<usize>| {
        let ok = if diff.is_empty() {
            true
        } else {
            let first = *diff.iter().next().unwrap();
            let last = *diff.iter().last().unwrap();
            let rev_cnt = last + first + 1;
            if rev_cnt > n {
                false
            } else {
                let first_half = seg_tree_s_rev.get(n - rev_cnt, n);
                let second_half = seg_tree_s.get(rev_cnt, n);
                let total_s = Hash::unite(&first_half, &second_half, &seg_tree_s.powers);
                let total_t = seg_tree_t.get(0, n);
                total_s.hash == total_t.hash
            }
        };
        if ok {
            writeln!(out, "Yes").unwrap();
        } else {
            writeln!(out, "No").unwrap();
        }
    };
    print_ans(&mut seg_tree_s, &mut seg_tree_s_rev, &mut seg_tree_t, &diff);
    for _ in 0..tc {
        let num = sc.usize();
        let pos = sc.usize() - 1;
        let pos_rev = n - pos - 1;
        let c = sc.string()[0];
        if num == 1 {
            s[pos] = c;
            seg_tree_s.modify(pos, pos + 1, s[pos] as i32);
            seg_tree_s_rev.modify(pos_rev, pos_rev + 1, s[pos] as i32);
        } else {
            assert_eq!(num, 2);
            t[pos] = c;
            seg_tree_t.modify(pos, pos + 1, t[pos] as i32);
            seg_tree_t_rev.modify(pos_rev, pos_rev + 1, t[pos] as i32);
        }
        if s[pos] == t[pos] {
            diff.remove(&pos);
        } else {
            diff.insert(pos);
        }
        print_ans(&mut seg_tree_s, &mut seg_tree_s_rev, &mut seg_tree_t, &diff);
    }
}
