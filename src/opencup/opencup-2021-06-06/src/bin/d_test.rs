use std::io;
use std::io::Write;
use std::cmp::max;
use std::collections::BTreeSet;

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
    fn usize_opt(&mut self) -> Option<usize> {
        self.next_opt::<usize>()
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
            match &mut self.input_source {
                | InputSource::Stdin => { std::io::stdin().read_line(&mut input).expect("Failed read"); }
                | InputSource::FromFile(lines) => {
                    let line = lines.pop().unwrap();
                    input = line;
                }
            }

            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    fn next_opt<T: std::str::FromStr>(&mut self) -> Option<T> {
        for _ in 0..3 {
            if let Some(token) = self.buffer.pop() {
                return Some(token.parse().ok().expect("Failed parse"));
            }
            let mut input = String::new();
            match &mut self.input_source {
                | InputSource::Stdin => { std::io::stdin().read_line(&mut input).expect("Failed read"); }
                | InputSource::FromFile(lines) => {
                    let line = lines.pop().unwrap();
                    input = line;
                }
            }

            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
        return None;
    }


    #[allow(dead_code)]
    fn string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}


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
        assert_ne!(seed, 0);
        Self {
            state: seed,
        }
    }
}


/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
}

struct State {
    a: Vec<Vec<usize>>,
    where_: Vec<Vec<usize>>,
    maybe_empty: Vec<usize>,
    maybe_alone_and_top: Vec<usize>,
    maybe_alone: Vec<usize>,
    moves: Vec<Move>,
}

impl State {
    fn create(n: usize, m: usize) -> Self {
        let a = vec![vec![]; m];
        let where_ = vec![vec![]; n];
        Self { a, where_, maybe_empty: vec![], maybe_alone_and_top: vec![], maybe_alone: vec![], moves: vec![] }
    }

    fn push(&mut self, pos: usize, val: usize) {
        self.a[pos].push(val);
        self.where_[val].push(pos);
    }

    fn init(&mut self) {
        for pos in 0..self.a.len() {
            if self.a[pos].is_empty() {
                self.maybe_empty.push(pos);
            }
            if self.a[pos].len() == 1 {
                let val = self.a[pos][0];
                self.maybe_alone.push(val);
                self.maybe_alone_and_top.push(val);
            }
        }
    }

    fn move_(&mut self, from: usize, to: usize) {
        let val = self.a[from].pop().unwrap();
        self.a[to].push(val);
        for x in self.where_[val].iter_mut() {
            if *x == from {
                *x = to;
            }
        }
        if self.a[from].is_empty() {
            self.maybe_empty.push(from);
        }
        if self.a[from].len() == 1 {
            let last_val = self.a[from][0];
            self.maybe_alone_and_top.push(last_val);
            self.maybe_alone.push(last_val);
        }
        self.maybe_alone.push(val);
        self.maybe_alone_and_top.push(val);
        self.moves.push(Move { from, to });
    }

    // fix top 2 of empty
    fn relax_empty(&mut self) {
        loop {
            if self.maybe_empty.is_empty() {
                break;
            }
            if !self.a[*self.maybe_empty.last().unwrap()].is_empty() {
                self.maybe_empty.pop();
                continue;
            }
            if self.maybe_empty.len() == 1 {
                break;
            }
            let prev = self.maybe_empty[self.maybe_empty.len() - 2];
            if prev == *self.maybe_empty.last().unwrap() || !self.a[prev].is_empty() {
                self.maybe_empty.remove(self.maybe_empty.len() - 2);
                continue;
            }
            break;
        }
    }

    fn is_top_and_alone(&self, val: usize) -> bool {
        if self.where_[val][0] == self.where_[val][1] {
            return false;
        }
        // dbg!("check is  top and alone", val);
        let p1 = self.where_[val][0];
        let p2 = self.where_[val][1];
        if self.a[p1].len() == 1 && self.a[p2].len() == 1 {
            return true;
        }
        if self.a[p1].len() == 1 && self.a[p2][1] == val {
            return true;
        }
        if self.a[p2].len() == 1 && self.a[p1][1] == val {
            return true;
        }
        return false;
    }

    fn get_top_and_alone(&mut self) -> Option<usize> {
        loop {
            let candidate = self.maybe_alone_and_top.last();
            if candidate == None {
                return None;
            }
            let candidate = *candidate.unwrap();
            if self.is_top_and_alone(candidate) {
                return Some(candidate);
            }
            self.maybe_alone_and_top.pop();
        }
    }

    fn get_alone(&mut self) -> Option<usize> {
        loop {
            let candidate = self.maybe_alone.last();
            if candidate == None {
                return None;
            }
            let candidate = *candidate.unwrap();
            let p1 = self.where_[candidate][0];
            let p2 = self.where_[candidate][1];
            let alone_p1 = self.a[p1].len() == 1;
            let alone_p2 = self.a[p2].len() == 1;
            if p1 != p2 && (alone_p1 || alone_p2) {
                return Some(candidate);
            } else {
                self.maybe_alone.pop();
                continue;
            }
        }
    }

    fn do_good_moves(&mut self) -> bool {
        let mut done = false;
        loop {
            match self.get_top_and_alone() {
                Some(val) => {
                    // dbg!("top and alone", val);
                    let p1 = self.where_[val][0];
                    let p2 = self.where_[val][1];
                    if self.a[p1].len() == 1 {
                        self.move_(p2, p1);
                    } else {
                        self.move_(p1, p2);
                    }
                    done = true;
                }
                None => {
                    match self.get_alone() {
                        Some(val) => {
                            self.relax_empty();
                            let empty = self.maybe_empty.last();
                            match empty {
                                None => return done,
                                Some(empty) => {
                                    let p1 = self.where_[val][0];
                                    let p2 = self.where_[val][1];
                                    if self.a[p1].len() == 1 {
                                        self.move_(p2, *empty);
                                        self.move_(p2, p1);
                                    } else {
                                        self.move_(p1, *empty);
                                        self.move_(p1, p2);
                                    }
                                    done = true
                                }
                            }
                        }
                        None => {
                            return done;
                        }
                    }
                    return done;
                }
            }
        }
    }

    fn this_value_is_ok_already(&self, val: usize) -> bool {
        return self.where_[val][0] == self.where_[val][1];
    }

    fn get_comp(&self, val: usize) -> Vec<usize> {
        let mut queue = vec![val];
        let mut seen = BTreeSet::new();
        seen.insert(val);
        while let Some(v) = queue.pop() {
            for pos in self.where_[v].iter() {
                for &to in self.a[*pos].iter() {
                    if !seen.contains(&to) {
                        seen.insert(to);
                        queue.push(to);
                    }
                }
            }
        }
        return seen.into_iter().collect();
    }

    fn is_bad(&self, val: usize) -> bool {
        let p1 = self.where_[val][0];
        let p2 = self.where_[val][1];
        return self.a[p1][0] == val && self.a[p2][0] == val;
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let mut rnd = Random::new(787788);

    loop {

        dbg!("test!");

        let n =rnd.next_in_range(1, 6);

        let m = rnd.next_in_range(n, n + 10);
        let mut a = vec![vec![]; m];
        for x in 0..n {
            for _ in 0..2 {
                let mut ok = false;
                while !ok {
                    let pos = rnd.next_in_range(0, n);
                    if a[pos].len() < 2 {
                        a[pos].push(x);
                        ok = true;
                        break;
                    }
                }
            }
        }
        let mut state = State::create(n, m);
        for i in 0..m {
            let cnt = a[i].len();
            for j in 0..cnt {
                let v = a[i][j];
                state.push(i, v);
            }
        }
        state.init();
        let mut idx = 0;
        // dbg!("start test!");
        loop {
            state.do_good_moves();

            while idx != n && state.this_value_is_ok_already(idx) {
                idx += 1;
            }
            // dbg!(idx);
            if idx == n {
                break;
            }
            let comp = state.get_comp(idx);
            let mut bad = vec![];
            let mut ok = vec![];
            for &x in comp.iter() {
                if state.is_bad(x) {
                    bad.push(x);
                } else {
                    ok.push(x);
                }
            }
            state.relax_empty();
            // dbg!(bad, ok, state.maybe_empty);
            if bad.len() >= 2 && state.maybe_empty.len() < 2 {
                break;
            }
            if state.maybe_empty.len() == 0 {
                break;
            }
            // dbg!(bad, ok);
            if bad.len() >= 2 {
                let some_bad = bad[0];
                let p1 = state.where_[some_bad][0];
                let p2 = state.where_[some_bad][1];
                let empty1 = state.maybe_empty[state.maybe_empty.len() - 1];
                let empty2 = state.maybe_empty[state.maybe_empty.len() - 2];
                state.move_(p1, empty1);
                state.move_(p2, empty2);
            } else {
                let mut some_ok_pos = 0;
                let mut found = false;
                for &x in ok.iter() {
                    for tt in 0..2 {
                        let p1 = state.where_[x][tt];
                        if state.a[p1][0] == x {
                            some_ok_pos = p1;
                            found = true;
                            break;
                        }
                    }
                }
                dbg!("???", some_ok_pos, ok, bad);
                if !found {
                    let x = bad[0];
                    some_ok_pos = state.where_[x][0];
                    found = true;
                }
                assert!(found);
                assert!(!state.maybe_empty.is_empty());
                let empty = *state.maybe_empty.last().unwrap();
                state.move_(some_ok_pos, empty);
            }
            loop {
                if !state.do_good_moves() {
                    break;
                }
            }
            // TODO: check everything is good!
        }
        // dbg!(state.a);
        let mut good = idx == n;
        for i in 0..n {
            if !state.this_value_is_ok_already(i) {
                good = false;
            }
        }
        if good {
            let max_moves = n * 3 / 2;
            if state.moves.len() > max_moves {
                dbg!(a);
                dbg!(state.moves);
            }
            assert!(state.moves.len() <= max_moves);
            // writeln!(out, "{}", state.moves.len()).unwrap();
            // for move_ in state.moves.iter() {
            //     writeln!(out, "{} {}", move_.from + 1, move_.to + 1).unwrap();
            // }
        } else {
            // writeln!(out, "-1").unwrap();
        }
    }
}
