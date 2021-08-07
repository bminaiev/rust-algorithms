use std::io;
use std::io::Write;
use std::cmp::{max, min};
use std::collections::BTreeSet;
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

    fn parse_next_line(&mut self) -> bool {
        let mut input = String::new();
        match &mut self.input_source {
            | InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            | InputSource::FromFile(lines) => {
                match lines.pop() {
                    Some(line) => input = line,
                    None => return false,
                }
            }
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

struct Answer {
    score: usize,
    path: String,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

struct Task {
    field: Vec<Vec<u32>>,
    start: Pos,
}

struct Line {
    top_left: Pos,
    bottom_right: Pos,
}

fn intersect(l1: &Line, l2: &Line) -> Option<Pos> {
    let min_x = max(l1.top_left.x, l2.top_left.x);
    let min_y = max(l1.top_left.y, l2.top_left.y);
    let max_x = min(l1.bottom_right.x, l2.bottom_right.x);
    let max_y = min(l1.bottom_right.y, l2.bottom_right.y);
    if min_x > max_x || min_y > max_y {
        return None;
    }
    assert_eq!(min_x, max_x);
    assert_eq!(min_y, max_y);
    Some(Pos { x: min_x, y: min_y })
}

struct Edge {
    dx: i32,
    dy: i32,
    c: u8,
}

const edges: [Edge; 4] =
    [
        Edge { dx: -1, dy: 0, c: b'U' },
        Edge { dx: 1, dy: 0, c: b'D' },
        Edge { dx: 0, dy: 1, c: b'R' },
        Edge { dx: 0, dy: -1, c: b'L' },
    ];

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Vertex {
    len: u32,
    pos: Pos,
    prev: Pos,
    edge: u8,
}

struct Helper {
    dists: Vec<Vec<u32>>,
    start_id: usize,
    points_on_line: Vec<Vec<usize>>,
    n_lines: usize,
    by_vertex: Vec<Vec<Vec<Option<Vertex>>>>,
    points: Vec<Pos>,
    sz: usize,
    start_time: Instant,
}

impl Helper {
    fn build_path(&self, from: usize, to: usize) -> Vec<u8> {
        let mut path = vec![];
        let mut cur_pos = self.points[to];
        let from_pos = self.points[from];
        while from_pos != cur_pos {
            let v = self.by_vertex[from][cur_pos.x][cur_pos.y].unwrap();
            path.push(v.edge);
            cur_pos = v.prev;
        }
        path.reverse();
        path
    }
}

fn solve_helper(h: &Helper, rnd: &mut Random) -> Answer {
    let n = h.dists.len();
    let mut was_on_line = vec![false; h.n_lines];
    let start_id = h.start_id;
    for &line in h.points_on_line[start_id].iter() {
        was_on_line[line] = true;
    }
    let mut more_lines_to_visit = 0;
    for l in 0..h.n_lines {
        if !was_on_line[l] {
            more_lines_to_visit += 1;
        }
    }
    let mut cur_pos = start_id;
    let mut path_ids = vec![start_id];
    while more_lines_to_visit > 0 {
        let mut bdist = std::u32::MAX;
        let mut bv = 0;
        for v in 0..n {
            let mut smth_new = false;
            for &l in h.points_on_line[v].iter() {
                if !was_on_line[l] {
                    smth_new = true;
                    break;
                }
            }
            if smth_new && h.dists[cur_pos][v] < bdist {
                bdist = h.dists[cur_pos][v];
                bv = v;
            }
        }
        assert_ne!(bdist, std::u32::MAX);
        path_ids.push(bv);
        for &l in h.points_on_line[bv].iter() {
            if !was_on_line[l] {
                was_on_line[l] = true;
                more_lines_to_visit -= 1;
            }
        }
        cur_pos = bv;
    }

    path_ids = local_optimize(h, path_ids, rnd);

    let mut char_path = vec![];
    let mut total_time = 0;
    for i in 0..path_ids.len() {
        let v1 = path_ids[i];
        let v2 = path_ids[(i + 1) % path_ids.len()];
        total_time += h.dists[v1][v2] as usize;
        let mut subpath = h.build_path(v1, v2);
        char_path.append(&mut subpath);
    }
    let path = String::from_utf8(char_path).unwrap();
    let score = 10000 + 10_000_000 * h.sz / total_time;
    Answer { score, path }
}

// bigger - better
fn calc_optimize(now_dist: u32, next_dist: u32) -> f64 {
    (now_dist as f64 - next_dist as f64) as f64 / (now_dist as f64)
}

fn local_optimize(h: &Helper, mut path_ids: Vec<usize>, rnd: &mut Random) -> Vec<usize> {
    let n = path_ids.len();
    let mut whole_dist = 0;
    for i in 0..n {
        let v1 = path_ids[i];
        let v2 = path_ids[(i + 1) % n];
        whole_dist += h.dists[v1][v2];
    }
    let before_dist = whole_dist;
    let d = &h.dists;

    const START_TEMP: f64 = 0.01;
    const FINAL_TEMP: f64 = 0.001;

    let mut best_path = path_ids.clone();
    let mut best_whole_dist = whole_dist;
    dbg!(before_dist);

    let mut iters = 0;
    while h.start_time.elapsed().as_millis() < MAX_TIME_MS {
        iters += 1;
        let offset = rnd.next_in_range(0, n);
        // TODO: change constant?
        let cnt = rnd.next_in_range(2, 5);
        if offset + cnt >= n {
            continue;
        }
        let mut cur_dist = 0;
        for i in 0..=cnt {
            cur_dist += d[path_ids[offset + i]][path_ids[(offset + i + 1) % n]];
        }
        let last = (offset + cnt + 1) % n;
        let mut next_dist = d[path_ids[offset]][path_ids[offset + cnt]] + d[path_ids[offset + 1]][path_ids[last]];
        for i in 0..(cnt - 1) {
            next_dist += d[path_ids[offset + i + 2]][path_ids[offset + i + 1]];
        }


        let time_passed = h.start_time.elapsed().as_millis() as f64 / (MAX_TIME_MS as f64);


        let cur_temp = START_TEMP * (FINAL_TEMP / START_TEMP).powf(time_passed);

        let optimize = calc_optimize(whole_dist, whole_dist + next_dist - cur_dist);
        if iters % 1000 == 0 {
            dbg!(iters, optimize, whole_dist, whole_dist + next_dist - cur_dist, cur_temp, f64::exp(optimize / cur_temp));
        }
        let change = optimize >= 0.0 || rnd.next_double() < f64::exp(optimize / cur_temp);
        if change {
            path_ids[offset + 1..offset + cnt + 1].reverse();
            whole_dist += next_dist;
            whole_dist -= cur_dist;
        }
        if whole_dist < best_whole_dist {
            best_whole_dist = whole_dist;
            best_path = path_ids.clone();
        }
    }
    let optimized = (before_dist - best_whole_dist) as f64 / (before_dist as f64);
    dbg!(before_dist, best_whole_dist, optimized, iters);
    best_path
}

const MAX_TIME_MS: u128 = 1000;

fn solve(t: &Task, rnd: &mut Random) -> Answer {
    let start_time = Instant::now();
    let sz = t.field.len();
    let mut lines = vec![];
    for i in 0..sz {
        for j in 0..sz {
            if t.field[i][j] != 0 && (j == 0 || t.field[i][j - 1] == 0) {
                let mut nj = j;
                while nj != sz && t.field[i][nj] != 0 {
                    nj += 1;
                }
                if nj - j > 1 {
                    lines.push(Line { top_left: Pos { x: i, y: j }, bottom_right: Pos { x: i, y: nj - 1 } });
                }
            }
        }
    }
    for j in 0..sz {
        for i in 0..sz {
            if t.field[i][j] != 0 && (i == 0 || t.field[i - 1][j] == 0) {
                let mut ni = i;
                while ni != sz && t.field[ni][j] != 0 {
                    ni += 1;
                }
                if ni - i > 1 {
                    lines.push(Line { top_left: Pos { x: i, y: j }, bottom_right: Pos { x: ni, y: j } });
                }
            }
        }
    }
    let mut on_line = vec![vec![vec![]; sz]; sz];
    for (i1, l1) in lines.iter().enumerate() {
        for (i2, l2) in lines.iter().enumerate() {
            if i2 >= i1 {
                break;
            }
            if let Some(center) = intersect(l1, l2) {
                on_line[center.x][center.y].push(i1);
                on_line[center.x][center.y].push(i2);
            }
        }
    }
    for (i, l) in lines.iter().enumerate() {
        if t.start.x >= l.top_left.x && t.start.x <= l.bottom_right.x {
            if t.start.y >= l.top_left.y && t.start.y <= l.bottom_right.y {
                on_line[t.start.x][t.start.y].push(i);
            }
        }
    }
    let mut points = vec![];
    let mut points_on_line = vec![];
    let mut start_id = std::usize::MAX;
    let mut point_ids = vec![vec![None; sz]; sz];
    for x in 0..sz {
        for y in 0..sz {
            if !on_line[x][y].is_empty() {
                if x == t.start.x && y == t.start.y {
                    start_id = points.len();
                }
                point_ids[x][y] = Some(points.len());
                points.push(Pos { x, y });
                points_on_line.push(on_line[x][y].clone());
            }
        }
    }
    assert_ne!(start_id, std::usize::MAX);
    let n = points.len();
    let mut by_vertex = vec![];
    let mut dists = vec![vec![0; n]; n];
    for i in 0..n {
        let sp = points[i];
        let mut dist = vec![vec![std::u32::MAX; sz]; sz];
        let mut seen = vec![vec![false; sz]; sz];
        let mut vertices = vec![vec![None; sz]; sz];
        dist[sp.x][sp.y] = 0;
        let v = Vertex { len: 0, pos: sp, prev: sp, edge: 0 };
        let mut pq = BTreeSet::new();
        pq.insert(v);
        while !pq.is_empty() {
            let v = pq.iter().next().unwrap().clone();
            pq.remove(&v);
            let p = v.pos;
            if seen[p.x][p.y] {
                continue;
            }
            vertices[p.x][p.y] = Some(v);
            if let Some(real_id) = point_ids[p.x][p.y] {
                dists[i][real_id] = v.len;
            }
            seen[p.x][p.y] = true;
            for e in edges.iter() {
                let nx = p.x as i32 + e.dx;
                let ny = p.y as i32 + e.dy;
                if nx >= 0 && (nx as usize) < sz {
                    if ny >= 0 && (ny as usize) < sz {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if t.field[nx][ny] != 0 {
                            let ndist = t.field[nx][ny] + dist[p.x][p.y];
                            if ndist < dist[nx][ny] {
                                dist[nx][ny] = ndist;
                                pq.insert(Vertex { len: ndist, pos: Pos { x: nx, y: ny }, prev: p, edge: e.c });
                            }
                        }
                    }
                }
            }
        }
        by_vertex.push(vertices);
    }
    let helper = Helper { dists, start_id, points_on_line, n_lines: lines.len(), by_vertex, points, sz, start_time };
    solve_helper(&helper, rnd)
}

fn solve_one_scanner(sc: &mut Scanner) -> Answer {
    let n = sc.usize();
    let sx = sc.usize();
    let sy = sc.usize();
    let start = Pos { x: sx, y: sy };
    let mut field = vec![vec![0; n]; n];
    for i in 0..n {
        let s = sc.string();
        for j in 0..n {
            let c = s[j];
            if c == b'#' {
                field[i][j] = 0;
            } else {
                field[i][j] = (c - b'0') as u32;
            }
        }
    }
    let t = Task { field, start };
    let mut rnd = Random::new(787788);
    solve(&t, &mut rnd)
}

pub fn main_local() {
    let mut sum_scores = 0;
    let mut tests_seen = 0;
    const TESTS: usize = 1;
    const FIRST_TEST: usize = 0;
    for test in FIRST_TEST..FIRST_TEST + TESTS {
        let filename = format!("inputs/{:04}.txt", test);
        let mut sc = Scanner::new_file(&filename);
        let ans = solve_one_scanner(&mut sc);
        println!("{}", ans.path);
        sum_scores += ans.score as i64;
        tests_seen += 1;
        dbg!(test, (sum_scores / tests_seen));
    }
    dbg!(sum_scores / tests_seen);
}


pub fn main() {
    if std::env::args().len() == 1 {
        let mut sc = Scanner::new();
        let ans = solve_one_scanner(&mut sc);
        println!("{}", ans.path);
    } else {
        main_local();
    }
}