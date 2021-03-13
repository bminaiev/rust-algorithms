use std::io;
use std::io::Write;
use std::cmp::min;

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

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

impl Scanner {
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
            std::io::stdin().read_line(&mut input).expect("Failed read");
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

const BEST: usize = 11;

#[derive(Default, Clone, Ord, PartialOrd, Eq, PartialEq, Copy, Debug)]
struct Color {
    dist: usize,
    color: usize,
}

#[derive(Default, Clone, Debug)]
struct Top11 {
    best: [Color; BEST],
    cnt: usize,
}

impl Top11 {
    fn shift_one(&mut self) {
        for i in 0..self.cnt {
            self.best[i].dist += 1;
        }
    }

    fn one_color(c: usize) -> Self {
        let mut best = [Color::default(); BEST];
        best[0] = Color { dist: 0, color: c };
        Top11 { best, cnt: 1 }
    }

    fn join_with(&mut self, another: &Self, tmp_vec: &mut Vec<Color>, tmp_seen_colors: &mut [usize], tmp_seen_iter: &mut usize) {
        tmp_vec.clear();
        for i in 0..self.cnt {
            tmp_vec.push(self.best[i].clone());
        }
        for i in 0..another.cnt {
            tmp_vec.push(another.best[i].clone());
        }
        tmp_vec.sort();
        self.cnt = 0;
        *tmp_seen_iter += 1;
        for col in tmp_vec.iter() {
            if tmp_seen_colors[col.color] == *tmp_seen_iter {
                continue;
            }
            tmp_seen_colors[col.color] = *tmp_seen_iter;
            self.best[self.cnt] = col.clone();
            self.cnt += 1;
            if self.cnt == BEST {
                break;
            }
        }
    }

    fn get(&self, nth: usize) -> usize {
        if nth < self.cnt {
            self.best[nth].dist
        } else {
            std::usize::MAX
        }
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let q = sc.usize();
    let a: Vec<_> = (0..n).map(|_| sc.vec::<usize>(n)).collect();
    let max_color = n * n + 1;

    let mut tmp_vec = vec![];
    let mut tmp_seen_colors = vec![0; max_color];
    let mut tmp_seen_iter = 0;

    let mut dp_right = vec![vec![Top11::default(); n + 1]; n];
    for i in 0..n {
        for j in (0..n).rev() {
            let mut tmp = dp_right[i][j + 1].clone();
            tmp.shift_one();
            let one_c = Top11::one_color(a[i][j]);
            tmp.join_with(&one_c, &mut tmp_vec, &mut tmp_seen_colors, &mut tmp_seen_iter);
            dp_right[i][j] = tmp;
        }
    }

    let mut dp_down = vec![vec![Top11::default(); n]; n + 1];
    for i in (0..n).rev() {
        for j in 0..n {
            let mut tmp = dp_down[i + 1][j].clone();
            tmp.shift_one();
            let one_c = Top11::one_color(a[i][j]);
            tmp.join_with(&one_c, &mut tmp_vec, &mut tmp_seen_colors, &mut tmp_seen_iter);
            dp_down[i][j] = tmp;
        }
    }

    let mut res = vec![0; n + 1];
    let ni32 = n as i32;
    for x_minus_y in -ni32..ni32 {
        let mut cur_top = Top11::default();
        for x in (0..n).rev() {
            let yi32 = x as i32 - x_minus_y;
            if yi32 < 0 || yi32 >= ni32 {
                continue;
            }
            let y = yi32 as usize;
            cur_top.shift_one();
            cur_top.join_with(&dp_right[x][y], &mut tmp_vec, &mut tmp_seen_colors, &mut tmp_seen_iter);
            cur_top.join_with(&dp_down[x][y], &mut tmp_vec, &mut tmp_seen_colors, &mut tmp_seen_iter);
            let mut cur_ans = cur_top.get(q);
            cur_ans = min(cur_ans, n - x);
            cur_ans = min(cur_ans, n - y);
            res[cur_ans] += 1;
        }
    }
    for i in (1..=n).rev() {
        res[i - 1] += res[i];
    }
    for i in 1..=n {
        writeln!(out, "{}", res[i]).unwrap();
    }
}
