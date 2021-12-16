use std::collections::{BTreeSet, VecDeque};
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

struct State {
    meets_by_people: Vec<VecDeque<usize>>,
    maybe_bad: BTreeSet<usize>,
    person_known_good: Vec<bool>,
    meets: Vec<Vec<usize>>,
    meets_known_good: Vec<bool>,
    maybe_bad_by_meeting: Vec<BTreeSet<usize>>,
}

fn mark_meet_good(state: &mut State, meet_id: usize) {
    // dbg!("is good meeting", meet_id);
    if state.meets_known_good[meet_id] {
        return;
    }
    state.meets_known_good[meet_id] = true;
    let cur_meeting = state.meets[meet_id].clone();
    // dbg!(cur_meeting);
    for &person_id in cur_meeting.iter() {
        mark_person_good(state, person_id, meet_id);
    }
}

fn mark_person_good(state: &mut State, person_id: usize, time: usize) {
    // dbg!(person_id, time, "is good person");
    loop {
        if state.meets_by_people[person_id].is_empty() {
            // TODO: he is good?
            state.maybe_bad.remove(&person_id);
            state.person_known_good[person_id] = true;

            return;
        }
        let first_meet = *state.meets_by_people[person_id].iter().next().unwrap();
        if first_meet > time {
            state.maybe_bad_by_meeting[first_meet].remove(&person_id);
            if state.maybe_bad_by_meeting[first_meet].is_empty() {
                mark_meet_good(state, first_meet);
            }
            return;
        }
        state.meets_by_people[person_id].pop_front();
        mark_meet_good(state, first_meet);
    }
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let mut shift = 0;
        let n = sc.usize();
        let k = sc.usize();
        let decode = |p: usize, shift: usize| -> usize { (p - 1 + shift) % n };
        let person_known_good = vec![false; n];
        let mut known_bad = vec![false; n]; // just for asserts
        let maybe_bad: BTreeSet<usize> = (0..n).collect();

        let meets = vec![];
        let meets_known_good = vec![];

        let meets_by_people: Vec<VecDeque<usize>> = vec![VecDeque::new(); n];

        let mut state = State {
            meets_by_people,
            maybe_bad,
            person_known_good,
            meets,
            meets_known_good,
            maybe_bad_by_meeting: vec![],
        };

        for _ in 0..k {
            let ev_type = sc.string()[0];
            if ev_type == b'K' {
                // people meet
                let cnt = sc.usize();
                let mut ids = vec![0; cnt];
                for i in 0..cnt {
                    ids[i] = decode(sc.usize(), shift);
                    assert!(!known_bad[ids[i]]);
                }
                let mut maybe_bad_here = BTreeSet::new();
                for &id in ids.iter() {
                    if !state.person_known_good[id] {
                        maybe_bad_here.insert(id);
                    }
                }
                if !maybe_bad_here.is_empty() {
                    let meet_id = state.meets.len();
                    for &id in ids.iter() {
                        state.maybe_bad.insert(id);
                        state.person_known_good[id] = false;
                        state.meets_by_people[id].push_back(meet_id);
                    }
                    state.meets.push(ids);
                    state.meets_known_good.push(false);
                    state.maybe_bad_by_meeting.push(maybe_bad_here);
                }
            } else if ev_type == b'N' {
                // person is good
                let id = decode(sc.usize(), shift);

                mark_person_good(&mut state, id, usize::MAX);
            } else if ev_type == b'P' {
                // person is bad
                let id = decode(sc.usize(), shift);
                // they are certainly bad now
                state.maybe_bad.remove(&id);
                known_bad[id] = true;
            } else if ev_type == b'Q' {
                // who is good?
                let start_from = decode(sc.usize(), shift);
                if state.maybe_bad.is_empty() {
                    shift = 0;
                    writeln!(out, "TAK").unwrap();
                } else {
                    let id = match state.maybe_bad.range(start_from..).next() {
                        Some(&val) => val,
                        None => *state.maybe_bad.iter().next().unwrap(),
                    };
                    shift = id + 1;
                    writeln!(out, "NIE {}", shift).unwrap();
                }
            } else {
                unreachable!();
            }
        }
    }
}
