use std::io;
use std::io::Write;
use std::cmp::max;

pub fn main() {
    let mut sc = Scanner::default();
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let n: usize = sc.next();
    let a: Vec<usize> = (0..n).map(|_| sc.next::<usize>() - 1).collect();
    let mut pos_of_max_elem = 0;
    for i in 0..n {
        if a[i] == n - 1 {
            pos_of_max_elem = i;
        }
    }
    if pos_of_max_elem == 0 || pos_of_max_elem == n - 1 {
        writeln!(out, "0").unwrap();
        return;
    }
    let mut moves_left = 0;
    let mut it = pos_of_max_elem;
    while it != 0 {
        if a[it - 1] < a[it] {
            moves_left += 1;
            it -= 1;
        } else {
            break;
        }
    }

    let mut moves_right = 0;
    it = pos_of_max_elem;
    while it != n - 1 {
        if a[it] > a[it + 1] {
            moves_right += 1;
            it += 1;
        } else {
            break;
        }
    }

    if conv_moves(moves_left) >= moves_right {
        writeln!(out, "0").unwrap();
        return;
    }

    if conv_moves(moves_right) >= moves_left {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut max_up_len_l = vec![0; n];
    for i in 1..n {
        if a[i - 1] > a[i] {
            max_up_len_l[i] = max_up_len_l[i - 1] + 1
        }
    }
    let mut max_up_len_r = vec![0; n];
    for i in (0..(n - 1)).rev() {
        if a[i] < a[i + 1] {
            max_up_len_r[i] = max_up_len_r[i + 1] + 1;
        }
    }
    let mut max_up = 0;
    for i in 0..n {
        if i != pos_of_max_elem - moves_left {
            max_up = max(max_up, max_up_len_r[i]);
        }
        if i != pos_of_max_elem + moves_right {
            max_up = max(max_up, max_up_len_l[i]);
        }
    }

    if max_up >= moves_left && max_up >= moves_right {
        writeln!(out, "0").unwrap();
        return;
    }

    writeln!(out, "1").unwrap();
}

fn conv_moves(z: usize) -> usize {
    if z % 2 == 0 {
        z - 1
    } else {
        z
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