use std::cmp::max;
use std::io;
use std::io::Write;
use std::ops::{Index, IndexMut};
use std::time::Instant;

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

struct Array2D<T> {
    m: usize,
    v: Vec<T>,
}

impl<T> Array2D<T>
where
    T: Clone,
{
    fn new(empty: T, n: usize, m: usize) -> Self {
        Self {
            m,
            v: vec![empty; n * m],
        }
    }

    fn calc_submatrix(&self, shift_x: usize, shift_y: usize) -> Array2DSubmatrix<T> {
        Array2DSubmatrix {
            m: self.m,
            v: &self.v,
            shift_x,
            shift_y,
        }
    }

    fn resize(&mut self, n1: usize, m1: usize) {
        self.m = m1;
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[(index) * self.m..(index + 1) * self.m]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[(index) * self.m..(index + 1) * self.m]
    }
}

struct Array2DSubmatrix<'a, T> {
    m: usize,
    v: &'a Vec<T>,
    shift_x: usize,
    shift_y: usize,
}

impl<T> Index<usize> for Array2DSubmatrix<'_, T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[(index + self.shift_x) * self.m + self.shift_y..(index + 1 + self.shift_x) * self.m]
    }
}

#[derive(Copy, Clone)]
struct Query {
    // inclusive, exclusive
    l1: usize,
    r1: usize,
    l2: usize,
    r2: usize,
    q_id: usize,
}

impl Query {
    fn swap(&mut self) {
        let tmp = self.l1;
        self.l1 = self.l2;
        self.l2 = tmp;
        let tmp = self.r1;
        self.r1 = self.r2;
        self.r2 = tmp;
    }
}

struct RecState {
    queries_by_lvl: Vec<Vec<Query>>,
    install_first: Vec<i32>,
    install_second: Vec<i32>,
    w: Array2D<i32>,
    w_rev: Array2D<i32>,
    w_mirror: Array2D<i32>,
    w_rev_mirror: Array2D<i32>,
    part_install_first: Vec<i32>,
    part_install_second: Vec<i32>,
    dp_res_right: Array2D<Dp>,
    dp_res_left: Array2D<Dp>,
}

#[derive(Copy, Clone, Debug)]
struct Dp {
    d00: i32,
    d01: i32,
    d10: i32,
    d11: i32,
}

impl Dp {
    fn max(&self) -> i32 {
        max(max(self.d00, self.d01), max(self.d10, self.d11))
    }
}

const NEG_INF: i32 = i32::MIN / 2;
const DP_INF: Dp = Dp {
    d00: NEG_INF,
    d01: NEG_INF,
    d10: NEG_INF,
    d11: NEG_INF,
};

fn solve(
    install_first: &[i32],
    install_second: &[i32],
    w: &Array2DSubmatrix<i32>,
    dp: &mut Array2D<Dp>,
    use_mid: usize,
    n: usize,
    m: usize,
) {
    for x in 0..=n {
        for y in 0..=m {
            dp[x][y] = DP_INF;
        }
    }
    if use_mid == 1 {
        dp[0][1].d01 = -install_second[0];
    } else {
        dp[0][1].d00 = 0;
    };
    let upd_max = |x: &mut i32, b: i32| {
        // *what = ((*what > with) as i32) * *what + ((with > *what) as i32) * with;
        // *what = max(*what, with);
        // *x = *x ^ ((*x ^ y) & -((*x < y) as i32));
        let a = *x;
        *x = a - ((a - b) & ((a - b) >> 31));
    };

    for x in 0..=n {
        for y in 1..=m {
            let cur = dp[x][y];
            if x < n {
                let mut next_x = dp[x + 1][y];
                let m1 = max(cur.d00, cur.d10);
                let m2 = max(cur.d01, cur.d11);
                upd_max(&mut next_x.d00, m1);
                upd_max(&mut next_x.d10, m1 - install_first[x]);
                upd_max(&mut next_x.d01, m2);
                upd_max(&mut next_x.d11, m2 - install_first[x] + w[x][y - 1]);

                dp[x + 1][y] = next_x;
            }
            if y < m {
                let mut next_y = dp[x][y + 1];
                let m1 = max(cur.d00, cur.d01);
                let m2 = max(cur.d10, cur.d11);
                upd_max(&mut next_y.d00, m1);
                upd_max(&mut next_y.d01, m1 - install_second[y]);

                upd_max(&mut next_y.d10, m2);
                if x > 0 {
                    upd_max(&mut next_y.d11, m2 - install_second[y] + w[x - 1][y]);
                }
                dp[x][y + 1] = next_y;
            }
        }
    }
}

fn go(
    state: &mut RecState,
    lvl: usize,
    l1: usize,
    r1: usize,
    l2: usize,
    r2: usize,
    answers: &mut Vec<i32>,
    is_swapped: bool,
) {
    if state.queries_by_lvl[lvl].is_empty() {
        return;
    }
    let len1 = r1 - l1;
    let len2 = r2 - l2;
    if len1 < len2 {
        for query in state.queries_by_lvl[lvl].iter_mut() {
            query.swap();
        }
        go(state, lvl, l2, r2, l1, r1, answers, !is_swapped);
        return;
    }
    if len1 == 1 {
        let cost_to_install = state.install_first[l1] + state.install_second[l2];
        let cost_to_add = state.w[l1][l2];
        if cost_to_add > cost_to_install {
            let answer = cost_to_add - cost_to_install;
            for query in state.queries_by_lvl[lvl].iter() {
                answers[query.q_id] = max(answers[query.q_id], answer);
            }
        }
    } else {
        let mid1 = (l1 + r1) / 2;

        //  left : [l1 .. mid1)
        // right : [mid1..r1)

        for mid2 in l2..r2 {
            for use_mid in 1..2 {
                // go right
                let size_first = r1 - mid1;
                let size_second = r2 - mid2;
                let w = if !is_swapped { &state.w } else { &state.w_rev };
                let part_w = w.calc_submatrix(mid1, mid2);
                let install_first = if !is_swapped {
                    &state.install_first
                } else {
                    &state.install_second
                };
                let install_second = if !is_swapped {
                    &state.install_second
                } else {
                    &state.install_first
                };

                state.dp_res_right.resize(size_first + 1, size_second + 1);

                solve(
                    &install_first[mid1..],
                    &install_second[mid2..],
                    &part_w,
                    &mut state.dp_res_right,
                    use_mid,
                    size_first,
                    size_second,
                );

                // go left
                let size_first = mid1 - l1;
                let size_second = mid2 - l2 + 1;
                let start_first = mid1 - 1;
                let start_second = mid2;
                let w_mirror = if !is_swapped {
                    &state.w_mirror
                } else {
                    &state.w_rev_mirror
                };
                let n = w_mirror.m;
                let part_w = w_mirror.calc_submatrix(n - start_first - 1, n - start_second - 1);
                for x in 0..size_first {
                    state.part_install_first[x] = if !is_swapped {
                        state.install_first[start_first - x]
                    } else {
                        state.install_second[start_first - x]
                    };
                }
                for y in 0..size_second {
                    state.part_install_second[y] = if !is_swapped {
                        state.install_second[start_second - y]
                    } else {
                        state.install_first[start_second - y]
                    };
                }

                state.dp_res_left.resize(size_first + 1, size_second + 1);

                solve(
                    &state.part_install_first,
                    &state.part_install_second,
                    &part_w,
                    &mut state.dp_res_left,
                    use_mid,
                    size_first,
                    size_second,
                );

                // TODO: union

                let add_more = if use_mid == 0 {
                    0
                } else {
                    if !is_swapped {
                        state.install_second[mid2]
                    } else {
                        state.install_first[mid2]
                    }
                };

                for query in state.queries_by_lvl[lvl].iter() {
                    if query.l1 < mid1 && query.r1 >= mid1 && query.l2 <= mid2 && mid2 < query.r2 {
                        let idx_r = query.r1 - mid1;
                        let idx_r2 = query.r2 - mid2;
                        let dp_r = state.dp_res_right[idx_r][idx_r2].max();
                        let idx_l = mid1 - query.l1;
                        let idx_l2 = mid2 + 1 - query.l2;
                        let dp_l = state.dp_res_left[idx_l][idx_l2].max();
                        let val = dp_r + dp_l + add_more;
                        answers[query.q_id] = max(answers[query.q_id], val);
                    }
                }
            }
        }
        // go left
        state.queries_by_lvl[lvl + 1].clear();
        let queries = state.queries_by_lvl.split_at_mut(lvl + 1);
        for &query in queries.0[lvl].iter() {
            if query.r1 < mid1 {
                queries.1[0].push(query);
            }
        }
        go(state, lvl + 1, l1, mid1, l2, r2, answers, is_swapped);

        // go right
        state.queries_by_lvl[lvl + 1].clear();
        let queries = state.queries_by_lvl.split_at_mut(lvl + 1);
        for &query in queries.0[lvl].iter() {
            if query.l1 >= mid1 {
                queries.1[0].push(query);
            }
        }
        go(state, lvl + 1, mid1, r1, l2, r2, answers, is_swapped);
        // recursive call?
    }
}

fn main_solve(
    install_first: Vec<i32>,
    install_second: Vec<i32>,
    w: Array2D<i32>,
    queries: Vec<Query>,
    print_ans: bool,
) {
    let n = install_first.len();

    let mut w_rev = Array2D::new(0, n, n);
    for x in 0..n {
        for y in 0..n {
            w_rev[x][y] = w[y][x];
        }
    }

    let mut w_mirror = Array2D::new(0, n, n);
    for x in 0..n {
        for y in 0..n {
            w_mirror[n - 1 - x][n - 1 - y] = w[x][y];
        }
    }

    let mut w_rev_mirror = Array2D::new(0, n, n);
    for x in 0..n {
        for y in 0..n {
            w_rev_mirror[n - 1 - x][n - 1 - y] = w_rev[x][y];
        }
    }

    let q = queries.len();

    let mut queries_by_lvl = vec![vec![]; 50];
    queries_by_lvl[0] = queries;
    let mut rec_state = RecState {
        queries_by_lvl,
        install_first,
        install_second,
        w,
        w_rev,
        w_mirror,
        w_rev_mirror,
        part_install_first: vec![0; n],
        part_install_second: vec![0; n],
        dp_res_left: Array2D::new(DP_INF, n + 1, n + 1),
        dp_res_right: Array2D::new(DP_INF, n + 1, n + 1),
    };

    let mut answers = vec![0; q];

    go(&mut rec_state, 0, 0, n, 0, n, &mut answers, false);

    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    if print_ans {
        for q_id in 0..q {
            writeln!(out, "{}", answers[q_id]).unwrap();
        }
    }
}

#[allow(dead_code)]
struct Random {
    state: usize,
}

impl Random {
    fn next(&mut self) -> usize {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    #[allow(dead_code)]
    fn next_in_range(&mut self, from: usize, to: usize) -> usize {
        assert!(from < to);
        from + self.next() % (to - from)
    }

    #[allow(dead_code)]
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }

    #[allow(dead_code)]
    fn new(seed: usize) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }
}

pub fn stress() -> bool {
    let start = Instant::now();
    let n = 500; //sc.usize();
    let q = 300_000; //sc.usize();

    let mut rnd = Random::new(8988899);

    let mut install_first = vec![0; n];
    let mut install_second = vec![0; n];
    for i in 0..n {
        install_first[i] = rnd.next_in_range(0, 10000) as i32;

        install_second[i] = rnd.next_in_range(0, 10000) as i32;
    }

    let mut w = Array2D::new(0, n, n);
    for i in 0..n {
        for j in 0..n {
            w[i][j] = rnd.next_in_range(0, 10000) as i32;
        }
    }

    let mut queries = vec![];
    for q_id in 0..q {
        let mut l1 = rnd.next_in_range(0, n);
        let mut r1 = rnd.next_in_range(0, n);
        let mut l2 = rnd.next_in_range(0, n);
        let mut r2 = rnd.next_in_range(0, n);
        if l1 > r1 {
            let tmp = l1;
            l1 = r1;
            r1 = tmp;
        }
        if l2 > r2 {
            let tmp = l2;
            l2 = r2;
            r2 = tmp;
        }
        r2 += 1;
        r1 += 1;
        assert!(l1 < r1);
        assert!(l2 < r2);
        queries.push(Query {
            l1,
            r1,
            l2,
            r2,
            q_id,
        });
    }

    main_solve(install_first, install_second, w, queries, false);

    dbg!(start.elapsed().as_millis());
    true
}

pub fn main() {
    // if stress() {
    //     return;
    // }

    let mut sc = Scanner::new();

    // let n = 500;
    // writeln!(out, "{}", rec(n, n)).unwrap();
    let n = sc.usize();
    let q = sc.usize();

    let install_first = sc.vec::<i32>(n);
    let install_second = sc.vec::<i32>(n);

    let mut w = Array2D::new(0, n, n);
    for i in 0..n {
        for j in 0..n {
            w[i][j] = sc.i32();
        }
    }

    let mut queries = vec![];
    for q_id in 0..q {
        let l1 = sc.usize() - 1;
        let r1 = sc.usize();
        let l2 = sc.usize() - 1;
        let r2 = sc.usize();
        queries.push(Query {
            l1,
            r1,
            l2,
            r2,
            q_id,
        });
    }
    main_solve(install_first, install_second, w, queries, true);
}
