use std::io;
use std::io::Write;
use std::cmp::max;

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

fn gcd(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if a == 0 {
        *x = 0;
        *y = 1;
        return b;
    }
    let mut x1 = 0;
    let mut y1 = 0;
    let d = gcd(b % a, a, &mut x1, &mut y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return d;
}

fn find_any_solution(a: i64, b: i64, c: i64, x0: &mut i64, y0: &mut i64, g: &mut i64) -> bool {
    *g = gcd(a.abs(), b.abs(), x0, y0);
    if c % *g != 0 {
        return false;
    }
    *x0 *= c / *g;
    *y0 *= c / *g;
    if a < 0 {
        *x0 *= -1;
    }
    if b < 0 {
        *y0 *= -1;
    }
    return true;
}

fn find_cnt_same(s1: i64, step1: i64, s2: i64, step2: i64) -> Option<(i64, i64)> {
    let mut x0 = 0;
    let mut y0 = 0;
    let mut g = 0;
    if find_any_solution(step1, -step2, s2 - s1, &mut x0, &mut y0, &mut g) {
        let mut tt = s1 + step1 * x0;
        let my_step = step1 * step2 / g;
        tt -= tt / my_step * my_step;
        if tt < 0 {
            tt += my_step;
        }
        Some((tt, my_step))
    } else {
        None
    }
}

fn find(first: i64, step: i64, till: i64) -> i64 {
    if first > till {
        0
    } else {
        1 + (till - first) / step
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let m = sc.usize();
    let k = sc.i64();
    let a = sc.vec::<usize>(n);
    let b = sc.vec::<usize>(m);
    let max_color = max(n, m) * 2 + 2;
    let mut colors = vec![vec![]; max_color];
    let mut sols = vec![None; max_color];
    for (pos, &val) in a.iter().enumerate() {
        colors[val].push((pos, a.len()));
    }
    for (pos, &val) in b.iter().enumerate() {
        colors[val].push((pos, b.len()));
    }
    for i in 0..colors.len() {
        if colors[i].len() < 2 {
            continue;
        }
        let f1 = colors[i][0];
        let f2 = colors[i][1];
        sols[i] = find_cnt_same(f1.0 as i64, f1.1 as i64, f2.0 as i64, f2.1 as i64);
    }
    let mut l = -1i64;
    let mut r = 1.2e18 as i64;
    while r - l > 1 {
        let mid = (l + r) >> 1;
        let mut cnt_same = 0;
        for i in 0..sols.len() {
            if let Some((first, step)) = sols[i] {
                cnt_same += find(first, step, mid);
            }
        }
        let cnt_diff = mid + 1 - cnt_same;
        if cnt_diff >= k {
            r = mid;
        } else {
            l = mid;
        }
    }
    writeln!(out, "{}", r + 1).unwrap();
}
