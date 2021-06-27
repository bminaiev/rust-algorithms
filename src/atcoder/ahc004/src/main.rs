use std::io;
use std::io::{Write, BufWriter, StdoutLock};
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

type Field = Vec<Vec<usize>>;

struct Task {
    n: usize,
    strings: Vec<Vec<usize>>,
}

struct Answer {
    field: Field,
    score: i64,
}


impl Answer {
    fn write(&self, out: &mut BufWriter<StdoutLock>) {
        for f in self.field.iter() {
            for &x in f.iter() {
                if x == 0 {
                    write!(out, ".").unwrap();
                } else {
                    let c = x as u8 + b'A' - 1;
                    out.write(&[c]).unwrap();
                }
            }
            writeln!(out).unwrap();
        }
    }

    fn ewrite(&self) {
        for f in self.field.iter() {
            for &x in f.iter() {
                if x == 0 {
                    eprint!(".");
                } else {
                    let c = x as u8 + b'A' - 1;
                    eprint!("{}", String::from_utf8(vec![c]).unwrap());
                }
            }
            eprintln!();
        }
        eprintln!();
        eprintln!();
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
struct Direction {
    dx: usize,
    dy: usize,
}

const DIRS: [Direction; 2] = [Direction { dx: 1, dy: 0 }, Direction { dx: 0, dy: 1 }];

fn exist_in_field(field: &Field, s: &Vec<usize>) -> bool {
    for x in 0..field.len() {
        for y in 0..field.len() {
            for dir in DIRS.iter() {
                let mut ok_word = true;
                for pos in 0..s.len() {
                    let nx = (x + dir.dx * pos) % field.len();
                    let ny = (y + dir.dy * pos) % field.len();
                    if field[nx][ny] != s[pos] {
                        ok_word = false;
                        break;
                    }
                }
                if ok_word {
                    return true;
                }
            }
        }
    }
    return false;
}

fn can_put(field: &Field, s: &Vec<usize>, x: usize, y: usize, dir: Direction) -> bool {
    for pos in 0..s.len() {
        let nx = (x + dir.dx * pos) % field.len();
        let ny = (y + dir.dy * pos) % field.len();
        if field[nx][ny] != s[pos] && field[nx][ny] != 0 {
            return false;
        }
    }
    return true;
}

fn count_score(field: &Field, t: &Task) -> i64 {
    let found: usize = t.strings.iter().map(|s| exist_in_field(field, s) as usize).sum();
    if found < t.strings.len() {
        return (1e8 * found as f64 / t.strings.len() as f64).round() as i64;
    } else {
        let n = field.len();
        let empty: usize = field.iter().map(|s| -> usize { s.iter().map(|x| (*x == 0) as usize).sum() }).sum();
        let n = n as f64;
        let empty = empty as f64;
        return (1e8 * 2.0 * n * n / (n * n * 2.0 - empty)).round() as i64;
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Place {
    x: usize,
    y: usize,
    dir: Direction,
    which: usize,
    idx: usize,
}

fn gen_all_places(t: &Task, field: &Field) -> Vec<Place> {
    let mut res = vec![];
    for x in 0..t.n {
        for y in 0..t.n {
            for dir in DIRS.iter() {
                for which in 0..t.strings.len() {
                    if !can_put(field, &t.strings[which], x, y, *dir) {
                        continue;
                    }
                    res.push(Place { x, y, dir: *dir, which, idx: res.len() });
                }
            }
        }
    }
    res
}

const MAX_TIMS_MS: u128 = 115_500;

fn solve_task_with_field(t: &Task, start_time: &Instant, rnd: &mut Random, mut field: Field) -> Answer {
    let m = t.strings.len();
    let all_places = gen_all_places(t, &field);
    let mut all_places_filtered: Vec<_> = all_places.iter().cloned().collect();
    let mut places_per_row = vec![vec![]; t.n];
    let mut places_per_col = vec![vec![]; t.n];
    for (i, p) in all_places.iter().enumerate() {
        if p.dir.dx == 0 {
            places_per_row[p.x].push(i);
        } else {
            places_per_col[p.y].push(i);
        }
    }
    let cnt_places = all_places.len();
    let mut same_chars = vec![0; cnt_places];
    let mut want = vec![vec![vec![0.0f64; 10]; t.n]; t.n];

    let add_want = |p: &Place, same_chars: &Vec<usize>, delta: i32, want: &mut Vec<Vec<Vec<f64>>>| {
        if same_chars[p.idx] == 0 {
            return;
        }
        for pos in 0..t.strings[p.which].len() {
            let nx = (p.x + p.dir.dx * pos) % t.n;
            let ny = (p.y + p.dir.dy * pos) % t.n;
            let expect = t.strings[p.which][pos];
            want[nx][ny][expect] += delta as f64 * 1.0;//(same_chars[p.idx] as f64 );
            assert!(want[nx][ny][expect] >= 0.0);
        }
    };

    for p in all_places.iter() {
        for pos in 0..t.strings[p.which].len() {
            let nx = (p.x + p.dir.dx * pos) % t.n;
            let ny = (p.y + p.dir.dy * pos) % t.n;
            if field[nx][ny] == t.strings[p.which][pos] {
                same_chars[p.idx] += 1;
            }
        }
        add_want(p, &same_chars, 1, &mut want);
    }

    let mut diff_chars = vec![0; cnt_places];
    let mut seen = vec![false; m];
    let mut found = 0;
    // let mut ops = 0i64;
    loop {
        // TODO: think about it?
        if start_time.elapsed().as_millis() > MAX_TIMS_MS {
            break;
        }
        if found % 100 == 99 {
            all_places_filtered = all_places_filtered.iter().filter(|p| diff_chars[p.idx] == 0).cloned().collect();
        }
        let f = |p: &Place| -> f64 {
            let mut res = same_chars[p.idx] as f64;
            res *= 1e9;
            for pos in 0..t.strings[p.which].len() {
                let x = (p.x + p.dir.dx * pos) % t.n;
                let y = (p.y + p.dir.dy * pos) % t.n;
                let w = want[x][y][t.strings[p.which][pos]];
                assert!(w >= 0.0);
                res += w;
            }
            res
        };

        let mut max_f = -100.0;
        for p in all_places_filtered.iter() {
            if !seen[p.which] && diff_chars[p.idx] == 0 {
                let f = f(p);
                if f > max_f {
                    max_f = f;
                }
            }
        }

        dbg!(max_f);

        let mut possible_places = vec![];

        for p in all_places_filtered.iter() {
            let f = f(p);
            if !seen[p.which] && diff_chars[p.idx] == 0 && (f - max_f).abs() < 1e-6 {
                possible_places.push(p);
            }
        }

        if possible_places.is_empty() {
            break;
        }
        let place = possible_places[rnd.next_in_range(0, possible_places.len())];
        let mut real_same = 0;

        seen[place.which] = true;
        // add_want(&place, &same_chars, -1, &mut want);
        dbg!("??");
        found += 1;

        let mut check_place = |p: &Place, ch_x: usize, ch_y: usize, ch_val: usize, diff_chars: &mut Vec<usize>| {
            if seen[p.which] {
                return;
            }
            if diff_chars[p.idx] == 0 {
                add_want(p, &same_chars, -1, &mut want);
            }
            if p.dir.dx == 0 {
                // row
                let dy = if ch_y >= p.y { ch_y - p.y } else { t.n + ch_y - p.y };
                if dy >= t.strings[p.which].len() {
                    return;
                }
                let expect = t.strings[p.which][dy];
                if expect == ch_val {
                    same_chars[p.idx] += 1;
                } else {
                    diff_chars[p.idx] += 1;
                }
            } else {
                // col
                let dx = if ch_x >= p.x { ch_x - p.x } else { t.n + ch_x - p.x };
                if dx >= t.strings[p.which].len() {
                    return;
                }
                let expect = t.strings[p.which][dx];
                if expect == ch_val {
                    same_chars[p.idx] += 1;
                } else {
                    diff_chars[p.idx] += 1;
                }
            }
            if diff_chars[p.idx] == 0 {
                add_want(p, &same_chars, 1, &mut want);
            }
        };

        for pos in 0..t.strings[place.which].len() {
            let nx = (place.x + place.dir.dx * pos) % field.len();
            let ny = (place.y + place.dir.dy * pos) % field.len();
            if field[nx][ny] != 0 {
                assert_eq!(field[nx][ny], t.strings[place.which][pos]);
                real_same += 1;
                continue;
            }
            field[nx][ny] = t.strings[place.which][pos];
            for &p_id in places_per_row[nx].iter() {
                check_place(&all_places[p_id], nx, ny, field[nx][ny], &mut diff_chars);
            }
            places_per_row[nx] = places_per_row[nx].iter().filter(|id| diff_chars[**id] == 0).cloned().collect();
            for &p_id in places_per_col[ny].iter() {
                check_place(&all_places[p_id], nx, ny, field[nx][ny], &mut diff_chars);
            }
            places_per_col[ny] = places_per_col[ny].iter().filter(|id| diff_chars[**id] == 0).cloned().collect();
        }
        // assert_eq!(real_same, max_same);

        let tmp_ans = Answer { field: field.clone(), score: 0 };
        tmp_ans.ewrite();
    }
    let mut total = vec![0; 20];
    let mut done = vec![0; 20];
    for i in 0..t.strings.len() {
        let len = t.strings[i].len();
        total[len] += 1;
        if seen[i] {
            done[len] += 1;
        }
    }
    for (idx, (&done, &total)) in done.iter().zip(total.iter()).enumerate() {
        if total == 0 {
            continue;
        }
        dbg!(idx, done, total);
    }
    dbg!(found, t.strings.len());
    let score = count_score(&field, t);
    Answer { field, score }
}

fn solve_task(t: &Task, start_time: &Instant, rnd: &mut Random) -> Answer {
    let mut field = vec![vec![0; t.n]; t.n];
    let mut ans = solve_task_with_field(t, start_time, rnd, field);
    // let prob = 1.0;
    // for x in 0..t.n {
    //     for y in 0..(t.n - 1) {
    //         if rnd.next_double() <= prob {
    //             ans.field[x][y] = 0;
    //         }
    //     }
    // }
    // ans.field[3][5] = 0;
    // ans.field[3][6] = 0;
    //
    // ans.field[0][5] = 0;
    // solve_task_with_field(t, start_time, rnd, ans.field)
    ans
}

fn solve_scanner(sc: &mut Scanner, repeat: bool) -> Answer {
    let n = sc.usize();
    let m = sc.usize();
    let mut strings = vec![];
    for _ in 0..m {
        let s = sc.string().iter().map(|x| (*x - b'A' + 1) as usize).collect();
        strings.push(s);
    }
    let task = Task { n, strings };


    let mut rnd = Random::new(787789);
    let mut best_solution = Answer { field: vec![], score: 0 };
    let start_time = Instant::now();
    loop {
        if start_time.elapsed().as_millis() > MAX_TIMS_MS {
            break;
        }
        let sol = solve_task(&task, &start_time, &mut rnd);
        if sol.score > best_solution.score {
            best_solution = sol;
        }
        if !repeat {
            break;
        }
    }
    best_solution
}

fn main_submit() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();
    let answer = solve_scanner(&mut sc, true);
    answer.write(&mut out);
}

fn main_test() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sum_scores = 0.0;
    eprintln!("hello?");
    let mut seen_tests = 0.0;

    for tc in 0..1 {
        let mut sc = Scanner::new_file(&format!("inputs/{:04}.txt", tc));
        let start_time = Instant::now();
        let answer = solve_scanner(&mut sc, false);
        sum_scores += answer.score as f64;
        seen_tests += 1.0;
        eprintln!("test = {}, current score: {}, av_score = {}, last_time = {}ms", tc, answer.score, sum_scores / seen_tests, start_time.elapsed().as_millis());
        // answer.write(&mut out);
    }
}

pub fn main() {
    main_test();
}
