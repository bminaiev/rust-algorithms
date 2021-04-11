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

fn gcd(x: u128, y: u128) -> u128 {
    if x == 0 {
        y
    } else {
        gcd(y % x, x)
    }
}

fn reduce(up: u128, down: u128) -> (u128, u128) {
    let g = gcd(up, down);
    (up / g, down / g)
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let tc = sc.usize();

    for t in 0..tc {
        let n = sc.usize();
        let q = sc.usize();
        let mut scores = vec![0; n];
        let mut answers = vec![vec![false; q]; n];
        for i in 0..n {
            let s = sc.string();
            for j in 0..q {
                answers[i][j] = s[j] == b'T';
            }
            scores[i] = sc.usize();
        }
        let mut inverted = vec![false; q];
        for i in 0..q {
            if answers[0][i] {
                inverted[i] = true;
                for j in 0..n {
                    answers[j][i] = !answers[j][i];
                }
            }
        }
        let mut cnt_positions = vec![0; 4];
        let mut masks = vec![0; q];
        for i in 0..q {
            let mut mask = 0;
            for j in 0..(n - 1) {
                if answers[j + 1][i] {
                    mask |= 1 << j;
                }
            }
            masks[i] = mask;
            cnt_positions[mask] += 1;
        }
        let mut c = vec![vec![0u128; q + 1]; q + 1];
        c[0][0] = 1;
        for i in 1..c.len() {
            c[i][0] = 1u128;
            for j in 1..c[i].len() {
                c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
            }
        }
        let mut tot_ways = 0u128;
        let mut sol_exp_score = vec![0u128; 1 << 4];
        // [m_i] - number of things, where correct answer is false
        for m0 in 0..=cnt_positions[0] {
            for m1 in 0..=cnt_positions[1] {
                for m2 in 0..=cnt_positions[2] {
                    for m3 in 0..=cnt_positions[3] {
                        let score0 = m0 + m1 + m2 + m3;
                        let score1 = m0 + m2 + cnt_positions[1] + cnt_positions[3] - m1 - m3;
                        let score2 = m0 + m1 + cnt_positions[2] + cnt_positions[3] - m2 - m3;
                        if score0 != scores[0] {
                            continue;
                        }
                        if n >= 2 && score1 != scores[1] {
                            continue;
                        }
                        if n >= 3 && score2 != scores[2] {
                            continue;
                        }
                        let ways = {
                            c[cnt_positions[0]][m0] *
                                c[cnt_positions[1]][m1] *
                                c[cnt_positions[2]][m2] *
                                c[cnt_positions[3]][m3]
                        };
                        tot_ways += ways;
                        for sol in 0..sol_exp_score.len() {
                            let mut sol_score = 0;
                            if (sol & 1) == 0 {
                                sol_score += m0 as u128;
                            } else {
                                sol_score += (cnt_positions[0] - m0) as u128;
                            }
                            if (sol & 2) == 0 {
                                sol_score += m1 as u128;
                            } else {
                                sol_score += (cnt_positions[1] - m1) as u128;
                            }
                            if (sol & 4) == 0 {
                                sol_score += m2 as u128;
                            } else {
                                sol_score += (cnt_positions[2] - m2) as u128;
                            }
                            if (sol & 8) == 0 {
                                sol_score += m3 as u128;
                            } else {
                                sol_score += (cnt_positions[3] - m3) as u128;
                            }
                            sol_exp_score[sol] += sol_score * ways;
                        }
                    }
                }
            }
        }
        assert_ne!(tot_ways, 0);
        let mut best_sol = 0;
        for sol in 0..sol_exp_score.len() {
            if sol_exp_score[sol] > sol_exp_score[best_sol] {
                best_sol = sol;
            }
        }
        let ans = reduce(sol_exp_score[best_sol], tot_ways);
        write!(out, "Case #{}: ", t + 1).unwrap();
        for i in 0..q {
            let mask = masks[i];
            let mut best_answer = ((best_sol >> mask) & 1) == 1;
            if inverted[i] {
                best_answer = !best_answer;
            }
            let c = if best_answer { "T" } else { "F" };
            write!(out, "{}", c).unwrap();
        }
        writeln!(out, " {}/{}", ans.0, ans.1).unwrap();
    }
}
