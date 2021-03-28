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

#[allow(dead_code)]
struct Random {
    state: usize
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
        Random { state: seed }
    }
}

fn f(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Question {
    solved: usize,
    id: usize,
}

fn predict_skill_by_number_of_wins(difficulties: &[f64]) -> Vec<f64> {
    let n_qs = difficulties.len();
    let mut skill_by_wins = vec![0.0; n_qs + 1];
    for wins in 0..=n_qs {
        let mut l = -3.0;
        let mut r = 3.0;
        for _ in 0..60 {
            let check_skill = (l + r) / 2.0;
            let mut exp_wins = 0.0;
            for &diff in difficulties {
                exp_wins += f(check_skill - diff);
            }
            if exp_wins > wins as f64 {
                r = check_skill
            } else {
                l = check_skill
            }
        }
        skill_by_wins[wins] = (l + r) / 2.0;
    }
    for pair in skill_by_wins.windows(2) {
        assert!(pair[0] <= pair[1]);
    }
    skill_by_wins
}

#[derive(PartialOrd, PartialEq, Debug, Clone)]
struct Skills {
    diff: f64,
    by_first: f64,
    by_second: f64,
    id: usize,
    solved_in_first: usize,
    solved_in_second: usize,
}

fn find_cheater(results: &[Vec<bool>], precalc: &Precalc) -> usize {
    let n = results.len();
    let n_questions = results[0].len();


    let mut questions: Vec<_> = (0..n_questions).map(|id| {
        let solved = results.iter().map(|a| if a[id] { 1 } else { 0 }).sum();
        Question { solved, id }
    }).collect();
    questions.sort();
    // first questions hard

    let mut skills = vec![];
    for pl_id in 0..n {
        let mut solved_in_first = 0;
        let mut solved_in_second = 0;
        for q_id in 0..n_questions {
            if results[pl_id][questions[q_id].id] {
                if q_id < n_questions / 2 {
                    solved_in_first += 1;
                } else {
                    solved_in_second += 1;
                }
            }
        }
        if solved_in_first < 2000 {
            continue;
        }
        solved_in_first = solved_in_first * (precalc.skills_by_wins_first.len()) / (n_questions / 2);
        solved_in_second = solved_in_second * (precalc.skills_by_wins_second.len()) / (n_questions / 2);
        let by_first = precalc.skills_by_wins_first[solved_in_first];
        let by_second = precalc.skills_by_wins_second[solved_in_second];
        skills.push(Skills { by_first, by_second, diff: by_first - by_second, id: pl_id, solved_in_first, solved_in_second });
    }

    skills.sort_by(|x1, x2| x1.partial_cmp(x2).unwrap());
    skills.reverse();

    if skills.is_empty() {
        return 0;
    }

    if skills[0].diff > 0.25 {
        skills[0].id
    } else {
        skills.sort_by(|x1, x2| (x1.solved_in_first).cmp(&x2.solved_in_first));
        skills.reverse();
        skills[0].id
    }
}

fn _do_one_test(seed: usize, precalc: &Precalc) -> bool {
    let n = 100;
    let n_questions = 10000;

    let mut rnd = Random::new(seed);
    for _ in 0..rnd.next_in_range(0, 100) {
        rnd.next_double();
    }
    let skills: Vec<_> = (0..n).map(|_| rnd.next_double() * 6.0 - 3.0).collect();
    let questions: Vec<_> = (0..n_questions).map(|_| rnd.next_double() * 6.0 - 3.0).collect();
    let mut results = vec![vec![false; n_questions]; n];
    for (idx, &skill) in skills.iter().enumerate() {
        for (q_id, &diff) in questions.iter().enumerate() {
            let prob_solve = f(skill - diff);
            if prob_solve > rnd.next_double() {
                results[idx][q_id] = true;
            }
        }
    }
    let cheater_id = rnd.next_in_range(0, n);
    for q_id in 0..n_questions {
        if rnd.next_in_range(0, 2) == 0 {
            results[cheater_id][q_id] = true;
        }
    }

    let found_cheater = find_cheater(&results, precalc);
    cheater_id == found_cheater
}

struct Precalc {
    skills_by_wins_first: Vec<f64>,
    skills_by_wins_second: Vec<f64>,
}

fn precalc() -> Precalc {
    const MAX: usize = 400;
    let skills_by_wins_first = {
        let difficulties: Vec<_> = (0..MAX).map(|pos| (pos as f64) / ((MAX - 1) as f64) * 3.0).collect();
        predict_skill_by_number_of_wins(&difficulties)
    };
    let skills_by_wins_second = {
        let difficulties: Vec<_> = (0..MAX).map(|pos| (pos as f64) / ((MAX - 1) as f64) * -3.0).collect();
        predict_skill_by_number_of_wins(&difficulties)
    };
    Precalc { skills_by_wins_first, skills_by_wins_second }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::default();

    let precalc = precalc();

    let tc = sc.usize();
    sc.usize();
    for t in 0..tc {
        let n = 100;
        let mut results = vec![];
        for _ in 0..n {
            let s = sc.string();
            let s_bool: Vec<_> = s.iter().map(|&x| x == b'1').collect();
            results.push(s_bool);
        }
        let cheater_id = find_cheater(&results, &precalc);
        writeln!(out, "Case #{}: {}", t + 1, cheater_id + 1).unwrap();
    }
}
