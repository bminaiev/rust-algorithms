use std::io;
use std::io::Write;
use std::cmp::Ordering;

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


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let n = sc.usize();
    let m = sc.usize();

    let a = (0..n).map(|_| sc.vec::<usize>(m)).collect::<Vec<_>>();
    let b = (0..n).map(|_| sc.vec::<usize>(m)).collect::<Vec<_>>();

    let mut result = vec![];

    let mut bad_column = vec![0; m];
    for i in 0..(n - 1) {
        for j in 0..m {
            if b[i][j] > b[i + 1][j] {
                bad_column[j] += 1;
            }
        }
    }
    let mut already_sorted = vec![false; m];
    let mut block_start = vec![false; n + 1];
    block_start[0] = true;
    block_start[n] = true;

    loop {
        let mut sort_by = None;
        for i in 0..m {
            if already_sorted[i] {
                continue;
            }
            if bad_column[i] == 0 {
                sort_by = Some(i);
                break;
            }
        }
        if let Some(sort_by) = sort_by {
            already_sorted[sort_by] = true;
            result.push(sort_by);

            let mut from = 0;
            while from != n {
                let mut to = from + 1;
                while !block_start[to] {
                    to += 1;
                }
                for x in from..(to - 1) {
                    if b[x][sort_by] < b[x + 1][sort_by] {
                        block_start[x + 1] = true;
                        for y in 0..m {
                            if b[x][y] > b[x + 1][y] {
                                bad_column[y] -= 1;
                            }
                        }
                    }
                }
                from = to;
            }
        } else {
            break;
        }
    }

    let mut order: Vec<_> = (0..n).collect();
    order.sort_by(|&id1, &id2| -> Ordering {
        for &col in &result {
            let cmp = a[id1][col].cmp(&a[id2][col]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        };
        id1.cmp(&id2)
    }
    );
    for i in 0..n {
        for j in 0..m {
            let a_sorted = a[order[i]][j];
            if a_sorted != b[i][j] {
                writeln!(out, "-1").unwrap();
                return;
            }
        }
    }
    writeln!(out, "{}", result.len()).unwrap();
    for &x in result.iter().rev() {
        write!(out, "{} ", x + 1).unwrap();
    }

    writeln!(out).unwrap();
}
