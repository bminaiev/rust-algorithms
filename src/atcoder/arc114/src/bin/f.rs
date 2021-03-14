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
    let k = sc.usize();
    let mut p = sc.vec::<usize>(n);
    for i in 0..n {
        p[i] -= 1;
    }
    let mut pos_of = vec![0; n];
    for (pos, &val) in p.iter().enumerate() {
        pos_of[val] = pos;
    }
    let mut more_joins = n - k;
    let mut start_of_piece = vec![true; n];

    let mut first_piece = 0;
    for val in (0..n).rev() {
        let pos = pos_of[val];
        if more_joins == 0 {
            break;
        }
        if pos == first_piece {
            first_piece += 1;
            while more_joins > 0 {
                while first_piece != n && !start_of_piece[first_piece] {
                    first_piece += 1;
                }
                if first_piece != n && p[first_piece] + more_joins + 1 < val {
                    more_joins -= 1;
                    start_of_piece[first_piece] = false;
                } else {
                    break;
                }
            }
            // dbg!("finish join to 0", first_piece, more_joins);
            continue;
        }
        more_joins -= 1;
        start_of_piece[pos] = false;
    }
    let mut pieces = vec![];
    let mut fr = 0;
    while fr != n {
        let mut to = fr + 1;
        while to != n && !start_of_piece[to] {
            to += 1;
        }
        // dbg!(p[fr..to]);
        pieces.push(p[fr..to].to_owned());
        fr = to;
    }
    pieces.sort();
    pieces.reverse();
    for piece in pieces.iter() {
        for &p in piece.iter() {
            write!(out, "{} ", p + 1).unwrap();
        }
    }
    writeln!(out).unwrap();
}
