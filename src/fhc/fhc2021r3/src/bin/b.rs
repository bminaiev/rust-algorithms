use std::{io, fs};
use std::io::{Write, BufWriter};
use std::fs::File;
use std::time::SystemTime;
use std::collections::{BTreeSet, VecDeque};

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

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct ModifiedFile {
    modified: SystemTime,
    path: String,
}

fn get_last_modified_file() -> String {
    let mut all_files = vec![];

    for entry in fs::read_dir("/home/borys/Downloads").unwrap() {
        let entry = entry.unwrap();
        let path = String::from(entry.path().to_str().unwrap());

        let metadata = fs::metadata(&path).unwrap();
        let modified = metadata.modified().unwrap();

        all_files.push(ModifiedFile { path, modified });
    }

    all_files.sort();

    let last = all_files.last().unwrap();
    println!("Last file is {}", last.path);
    last.path.clone()
}

mod modulo {
    const MODULO: i32 = 1_000_000_007;

    #[derive(Copy, Clone, Eq, PartialEq)]
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
use std::cmp::min;

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

trait LazySegTreeNodeSpec: Clone + Default {
    fn unite(l: &Self, r: &Self) -> Self;

    fn apply_update(node: &mut Self, update: &Self::Update);
    fn join_updates(current: &mut Self::Update, add: &Self::Update);

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

const SZ: usize = 3;

#[derive(Clone, Default, Debug)]
struct Node {
    dist: [[i32; SZ]; SZ],
}


impl LazySegTreeNodeSpec for Node {
    type Update = usize;

    fn unite(left: &Self, right: &Self) -> Self {
        let mut dist = [[MAX; SZ]; SZ];
        for c1 in 0..SZ {
            for c2 in 0..SZ {
                for c3 in 0..SZ {
                    dist[c1][c3] = min(dist[c1][c3], left.dist[c1][c2] + 1 + right.dist[c2][c3]);
                }
            }
        }

        Self { dist }
    }


    fn apply_update(node: &mut Self, update: &Self::Update) {
        let c = *update;
        node.dist[c][c] = 0;
        if c == 0 || c == 2 {
            if node.dist[1][1] == 0 {
                node.dist[c][1] = 1;
                node.dist[1][c] = 1;
                let another = 2 - c;
                if node.dist[another][another] == 0 {
                    node.dist[c][another] = 2;
                    node.dist[another][c] = 2;
                }
            }
        } else {
            if node.dist[0][0] == 0 {
                node.dist[0][1] = 1;
                node.dist[1][0] = 1;
            }
            if node.dist[2][2] == 0 {
                node.dist[2][1] = 1;
                node.dist[1][2] = 1;
            }
            if node.dist[0][0] == 0 && node.dist[2][2] == 0 {
                node.dist[0][2] = 2;
                node.dist[2][0] = 2;
            }
        }
    }

    fn join_updates(current: &mut Self::Update, add: &Self::Update) {
        // DO: nothing!
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
struct Cell {
    required: i32,
    r: usize,
    c: usize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Query {
    lvl: i32,
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
    id: usize,
}

const MAX: i32 = 100_000_000;

fn solve_one_test(sc: &mut Scanner, out: &mut BufWriter<File>, test_n: usize) {
    let n = sc.usize();
    let m = sc.usize();
    let mut cells = Vec::with_capacity(3 * n);
    for x in 0..n {
        for y in 0..3 {
            let required = sc.i32();
            cells.push(Cell { required, r: x, c: y })
        }
    }
    cells.sort();
    let mut queries = Vec::with_capacity(m);
    for id in 0..m {
        let r1 = sc.usize() - 1;
        let c1 = sc.usize() - 1;
        let r2 = sc.usize() - 1;
        let c2 = sc.usize() - 1;
        let lvl = sc.i32();
        if r1 < r2 {
            queries.push(Query { lvl, r1, c1, r2, c2, id });
        } else {
            queries.push(Query { lvl, r1: r2, c1: c2, r2: r1, c2: c1, id });
        }
    }
    queries.sort();
    let mut iter = 0;
    let mut set_by_col = vec![BTreeSet::new(); 3];
    let mut unset_by_col = vec![BTreeSet::new(); 3];
    for row in 0..n {
        for col in 0..3 {
            unset_by_col[col].insert(row);
        }
    }
    let mut res = Mod::ONE;
    let mut alive = vec![vec![false; 3]; n];

    let init_val = Node { dist: [[MAX; SZ]; SZ] };
    let mut tree = LazySegTree::new(&init_val, n);

    for query in queries.iter() {
        while iter != cells.len() && cells[iter].required <= query.lvl {
            let cell = cells[iter];
            set_by_col[cell.c].insert(cell.r);
            unset_by_col[cell.c].remove(&cell.r);
            alive[cell.r][cell.c] = true;
            tree.modify(cell.r, cell.r + 1, cell.c);
            iter += 1;
        }
        assert!(alive[query.r1][query.c1]);
        assert!(alive[query.r2][query.c2]);

        let gen_row = |r: usize, c: usize| -> Vec<i32> {
            let mut res = vec![MAX; 3];
            res[c] = 0;
            if c == 0 || c == 2 {
                if alive[r][1] {
                    res[1] = 1;
                    if alive[r][2 - c] {
                        res[2 - c] = 2;
                    }
                } else {
                    let another = 2 - c;
                    let from0 = match unset_by_col[0].range(..r).last() {
                        None => 0,
                        Some(row) => row + 1,
                    };
                    let to0 = match unset_by_col[0].range(r..).next() {
                        None => n,
                        Some(&row) => row,
                    };
                    let from2 = match unset_by_col[2].range(..r).last() {
                        None => 0,
                        Some(row) => row + 1,
                    };
                    let to2 = match unset_by_col[2].range(r..).next() {
                        None => n,
                        Some(&row) => row,
                    };
                    {
                        match set_by_col[1].range(..r).last() {
                            None => {}
                            Some(&row) => {
                                if row >= from0 && row >= from2 {
                                    assert!(alive[row][0] && alive[row][1] && alive[row][2]);
                                    // TODO: delete it!
                                    // for rr in row + 1..r {
                                    //     assert!(!alive[rr][1]);
                                    // }
                                    res[another] = min(res[another], (2 + (r - row) * 2) as i32);
                                }
                            }
                        }
                    }
                    {
                        match set_by_col[1].range(r..).next() {
                            None => {}
                            Some(&row) => {
                                if row < to0 && row < to2 {
                                    assert!(alive[row][0] && alive[row][1] && alive[row][2]);
                                    // TODO: delete it!
                                    // for rr in r..row {
                                    //     assert!(!alive[rr][1]);
                                    // }
                                    res[another] = min(res[another], (2 + (row - r) * 2) as i32);
                                }
                            }
                        }
                    }
                }
            } else {
                if alive[r][0] {
                    res[0] = 1;
                }
                if alive[r][2] {
                    res[2] = 1;
                }
            }

            res
        };

        let d1 = gen_row(query.r1, query.c1);
        let d2 = gen_row(query.r2, query.c2);
        let from_tree = tree.get(query.r1, query.r2 + 1);
        let mut cur_res = MAX;
        for c1 in 0..SZ {
            for c2 in 0..SZ {
                cur_res = min(cur_res, d1[c1] + d2[c2] + from_tree.dist[c1][c2]);
            }
        }
        if cur_res >= MAX {
            cur_res = 1;
        }

        if false {
            let mut dist = vec![vec![MAX; 3]; n];
            dist[query.r1][query.c1] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(Pos { r: query.r1, c: query.c1 });
            let mut slow_res = 1;
            while let Some(pos) = queue.pop_front() {
                if pos.r == query.r2 && pos.c == query.c2 {
                    slow_res = dist[query.r2][query.c2];
                }
                for shift in SHIFTS.iter() {
                    let nr = (pos.r as i32) + shift.dr;
                    let nc = (pos.c as i32) + shift.dc;
                    if nr >= 0 && nr < n as i32 && nc >= 0 && nc < 3 {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        if alive[nr][nc] && dist[nr][nc] == MAX {
                            dist[nr][nc] = dist[pos.r][pos.c] + 1;
                            queue.push_back(Pos { r: nr, c: nc });
                        }
                    }
                }
            }
            if slow_res != cur_res {
                if n < 5000 {
                    for r in 0..n {
                        print!("!! {}: ", r);
                        for c in 0..3 {
                            print!("{}", if alive[r][c] { 1 } else { 0 });
                        }
                        println!()
                    }
                    println!("{} {} -> {} {} : {}", query.r1, query.c1, query.r2, query.c2, cur_res);
                }


                dbg!(slow_res);
                dbg!(cur_res);
                dbg!(dist);
                dbg!(from_tree);
                dbg!(tree.get(27, 28));
                dbg!(alive[27]);
            }
            assert_eq!(slow_res, cur_res);
        }


        res *= Mod::new(cur_res);
    }
    writeln!(out, "Case #{}: {}", test_n, res).unwrap();
}

struct Shift {
    dr: i32,
    dc: i32,
}

const SHIFTS: [Shift; 4] = [Shift { dr: 1, dc: 0 }, Shift { dr: -1, dc: 0 }, Shift { dr: 0, dc: 1 }, Shift { dr: 0, dc: -1 }, ];

struct Pos {
    r: usize,
    c: usize,
}

pub fn main() {
    let input_file = get_last_modified_file();
    const OUTPUT_FILE: &str = "out/b.out";
    let mut out = std::io::BufWriter::new(File::create(OUTPUT_FILE).unwrap());
    let mut sc = Scanner::new_file(&input_file);

    let tc = sc.usize();
    for test_n in 1..=tc {
        dbg!("started", test_n, tc);
        solve_one_test(&mut sc, &mut out, test_n);
    }

    let source_code_file = concat!(env!("CARGO_MANIFEST_DIR"), "/", file!());
    dbg!(source_code_file);
    const OUTPUT_DIR: &str = "/home/borys/fb-output";

    fs::create_dir_all(OUTPUT_DIR).unwrap();
    fs::copy(source_code_file, String::from(OUTPUT_DIR) + "/solution.rs").unwrap();
    out.flush().unwrap();
    fs::copy(OUTPUT_FILE, String::from(OUTPUT_DIR) + "/answer.txt").unwrap();
}
