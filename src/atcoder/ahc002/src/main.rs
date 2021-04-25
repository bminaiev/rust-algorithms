use std::io;
use std::io::Write;
use std::cmp::{max, Ordering};
use std::collections::{HashMap, VecDeque, HashSet};
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

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

const N: usize = 50;

#[derive(Clone)]
struct Answer {
    moves: String,
    score: u32,
}

struct Task {
    start: Position,
    ids: Vec<Vec<usize>>,
    scores: Vec<Vec<u32>>,
    max_id: usize,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Edge {
    cost: u32,
    to: Vertex,
    name: &'static str,
}


struct FlatTask {
    ids: Vec<Comp>,
    g: Vec<Vec<Edge>>,
    start_pos: Vertex,
    start_score: u32,
    max_id: usize,
    #[allow(unused)]
    conv_ids_back: Vec<Position>,
}

struct State {
    best_answer: Answer,
    seen: Vec<bool>,
    pos: Vertex,
    total_iters: u64,
    moves: String,
    cur_score: u32,
    rnd: Random,
}

impl State {
    pub(crate) fn should_finish(&self) -> bool {
        self.total_iters > MAX_ITERS
    }
}

const MAX_ITERS: u64 = 10_000;
const MAX_TIME_MS: u128 = 300;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Vertex(usize);

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
struct Comp(usize);

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct EdgeWithScore {
    score: usize,
    edge: Edge,
}


fn dfs(t: &FlatTask, state: &mut State) {
    // state.total_iters += 1;
    // if state.total_iters > MAX_ITERS {
    //     return;
    // }
    if state.cur_score > state.best_answer.score {
        state.best_answer = Answer { moves: state.moves.clone(), score: state.cur_score }
    }


    let cost_coef = state.rnd.next_in_range(5, 10);
    let mut edges: Vec<_> = t.g[state.pos.0].iter().map(|e| -> EdgeWithScore {
        let mut best_score = 0;
        let checks = state.rnd.next_in_range(50, 100);
        for _ in 0..checks {
            let mut seen: HashSet<Comp> = HashSet::new();
            seen.insert(t.ids[e.to.0]);
            let mut cur_pos = e.to;
            let mut score = 0;
            loop {
                let mut found_next_edge = false;
                let mut cur_edges = t.g[cur_pos.0].clone();
                for i in 1..cur_edges.len() {
                    let j = state.rnd.next_in_range(0, i + 1);
                    cur_edges.swap(i, j);
                }
                for cur_edge in cur_edges.iter() {
                    let next_comp = t.ids[cur_edge.to.0];
                    if state.seen[next_comp.0] || seen.contains(&next_comp) {
                        continue;
                    }
                    seen.insert(next_comp);
                    score += cur_edge.cost;
                    cur_pos = cur_edge.to;
                    found_next_edge = true;
                    break;
                }
                if !found_next_edge {
                    break;
                }
            }
            best_score = max(best_score, score);
        }
        EdgeWithScore { edge: *e, score: (best_score + (e.cost / cost_coef as u32)) as usize }
    }).collect();
    edges.sort();
    edges.reverse();


    for e_with_score in edges.iter() {
        let e = e_with_score.edge;
        let id = t.ids[e.to.0];
        if state.seen[id.0] {
            continue;
        }
        {
            let prev_pos = state.pos;
            state.pos = e.to;
            state.seen[id.0] = true;
            state.moves += e.name;
            state.cur_score += e.cost;

            dfs(t, state);

            state.cur_score -= e.cost;
            state.moves.pop();
            state.seen[id.0] = false;
            state.pos = prev_pos;
        }
        break;
    }
}

struct ModifiableAnswer {
    score_after: Vec<u32>,
    pos_after: Vec<Vertex>,
    moves: String,
    seen_comps: HashSet<Comp>,
    when_vertex: HashMap<Vertex, usize>,
}

impl ModifiableAnswer {
    fn from_ans(t: &FlatTask, ans: Answer) -> Self {
        let mut cur_pos = t.start_pos;
        let mut score_after = vec![t.start_score];
        let mut seen_comps = HashSet::new();
        seen_comps.insert(t.ids[t.start_pos.0]);
        let mut when_vertex = HashMap::new();
        when_vertex.insert(t.start_pos, 0);
        let mut pos_after = vec![t.start_pos];
        for (pos, c) in ans.moves.chars().enumerate() {
            let mut used_edge = None;
            for edge in t.g[cur_pos.0].iter() {
                if edge.name.starts_with(c) {
                    used_edge = Some(edge);
                }
            }
            assert_ne!(used_edge, None);
            let used_edge = used_edge.unwrap();
            cur_pos = used_edge.to;
            score_after.push(score_after.last().unwrap() + used_edge.cost);
            when_vertex.insert(cur_pos, pos + 1);
            seen_comps.insert(t.ids[cur_pos.0]);
            pos_after.push(cur_pos);
        }
        assert_eq!(score_after.last(), Some(&ans.score));
        Self {
            score_after,
            moves: ans.moves,
            seen_comps,
            when_vertex,
            pos_after,
        }
    }
}

fn local_optimize(t: &FlatTask, ans: Answer, rnd: &mut Random, timer: &Instant) -> Answer {
    let mut modifiable_answer = ModifiableAnswer::from_ans(t, ans);

    loop {
        if timer.elapsed().as_millis() > MAX_TIME_MS {
            break;
        }
        let idx = rnd.next_in_range(0, modifiable_answer.pos_after.len());
        let mut cur_pos = modifiable_answer.pos_after[idx];
        let mut additional_score = 0;
        let mut found_optimize = false;
        let mut seen_comps_in_path = HashSet::new();
        let mut path_moves = String::new();
        loop {
            let mut edges = vec![];
            for edge in t.g[cur_pos.0].iter() {
                let to_vertex = edge.to;
                match modifiable_answer.when_vertex.get(&to_vertex) {
                    None => {
                        let next_comp = t.ids[to_vertex.0];
                        if !modifiable_answer.seen_comps.contains(&next_comp)
                            && !seen_comps_in_path.contains(&next_comp) {
                            edges.push(edge);
                        }
                    }
                    Some(&idx_when) => {
                        if idx_when > idx {
                            let old_score_diff = modifiable_answer.score_after[idx_when - 1] - modifiable_answer.score_after[idx];
                            if old_score_diff < additional_score {
                                path_moves += edge.name;
                                modifiable_answer.moves.replace_range(idx..idx_when, &path_moves);

                                let answer = Answer { moves: modifiable_answer.moves, score: modifiable_answer.score_after.last().unwrap() + additional_score - old_score_diff };

                                modifiable_answer = ModifiableAnswer::from_ans(t, answer);

                                found_optimize = true;
                                break;
                            }
                        }
                    }
                }
            }
            if found_optimize || edges.is_empty() {
                break;
            }
            let use_edge = edges[rnd.next_in_range(0, edges.len())];
            additional_score += use_edge.cost;
            cur_pos = use_edge.to;
            seen_comps_in_path.insert(t.ids[cur_pos.0]);
            path_moves += use_edge.name;
        }
    }
    Answer { moves: modifiable_answer.moves, score: *modifiable_answer.score_after.last().unwrap() }
}

fn solve_flat(t: &mut FlatTask) -> Answer {
    let mut seen = vec![false; t.max_id + 1];
    seen[t.ids[t.start_pos.0].0] = true;

    let mut state = State {
        best_answer: Answer { moves: String::new(), score: t.start_score },
        seen,
        pos: t.start_pos,
        total_iters: 0,
        moves: String::new(),
        cur_score: t.start_score,
        rnd: Random::new(787788),
    };

    let global_timer = Instant::now();
    let mut real_best_ans = Answer { moves: String::new(), score: 0 };
    while global_timer.elapsed().as_millis() < 1_500 {
        // dbg!("new iter?");
        state.moves.clear();
        state.cur_score = t.start_score;
        state.seen = vec![false; t.max_id + 1];
        state.seen[t.ids[t.start_pos.0].0] = true;
        state.pos = t.start_pos;
        state.best_answer.score = 0;

        dfs(t, &mut state);

        let ans = state.best_answer.clone();
        let timer = Instant::now();
        let ans = local_optimize(t, ans, &mut state.rnd, &timer);
        if ans.score > real_best_ans.score {
            real_best_ans = ans;
            // dbg!("best score optimize: ", real_best_ans.score);
        }
    }

    real_best_ans
}

fn solve(t: &Task) -> Answer {
    let mut conv_ids = vec![vec![Vertex(0); N]; N];
    let mut cur_id = 0;
    let mut conv_ids_back = vec![];
    for i in 0..N {
        for j in 0..N {
            conv_ids[i][j] = Vertex(cur_id);
            conv_ids_back.push(Position { x: i, y: j });
            cur_id += 1;
        }
    }

    let n = N * N;
    let mut flat_ids = vec![Comp(0); n];
    for i in 0..N {
        for j in 0..N {
            flat_ids[conv_ids[i][j].0] = Comp(t.ids[i][j]);
        }
    }

    let mut g = vec![vec![]; n];
    for i in 0..N {
        for j in 0..N {
            if i + 1 < N {
                g[conv_ids[i][j].0].push(Edge { cost: t.scores[i + 1][j], to: conv_ids[i + 1][j], name: "D" });
                g[conv_ids[i + 1][j].0].push(Edge { cost: t.scores[i][j], to: conv_ids[i][j], name: "U" });
            }
            if j + 1 < N {
                g[conv_ids[i][j].0].push(Edge { cost: t.scores[i][j + 1], to: conv_ids[i][j + 1], name: "R" });
                g[conv_ids[i][j + 1].0].push(Edge { cost: t.scores[i][j], to: conv_ids[i][j], name: "L" });
            }
        }
    }

    let mut flat_task = FlatTask { ids: flat_ids, g, start_pos: conv_ids[t.start.x][t.start.y], start_score: t.scores[t.start.x][t.start.y], max_id: t.max_id, conv_ids_back };
    solve_flat(&mut flat_task)
}

fn solve_scanner(sc: &mut Scanner) -> Answer {
    let sx = sc.usize();
    let sy = sc.usize();
    let start = Position { x: sx, y: sy };
    let mut ids = vec![];
    let mut max_id = 0;
    for _ in 0..N {
        let cur_line = sc.vec::<usize>(N);
        for &id in &cur_line {
            max_id = max(max_id, id);
        }
        ids.push(cur_line);
    }
    let mut scores = vec![];
    for _ in 0..N {
        scores.push(sc.vec::<u32>(N));
    }
    let task = Task { start, ids, scores, max_id };
    solve(&task)
}

fn main_submit() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();
    let answer = solve_scanner(&mut sc);
    writeln!(out, "{}", answer.moves).unwrap();
}

fn main_test() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sum_scores = 0.0;
    eprintln!("hello?");
    for tc in 0..10 {
        let mut sc = Scanner::new_file(&format!("inputs/{:04}.txt", tc));
        let answer = solve_scanner(&mut sc);
        sum_scores += answer.score as f64;
        eprintln!("test = {}, expected score: {}, av_score = {}", tc, answer.score, sum_scores / ((tc as f64) + 1.0));
        writeln!(out, "{}", answer.moves).unwrap();
    }
}

pub fn main() {
    main_submit();
}
