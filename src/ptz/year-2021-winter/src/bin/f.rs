use std::io;
use std::io::Write;
use std::cmp::min;
use std::collections::{BTreeSet, VecDeque};

fn fmin(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}

fn fmax(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
}

fn combine_answers(n: usize, from_left: &[Answer], from_right: &[Answer], from_mid: &[Answer]) -> Vec<f64> {
    let mut res = vec![0.0; n];
    let mut min_x = 0.0;
    let mut it_left = 0;
    let mut it_right = 0;
    let mut it_mid = 0;
    while it_left != from_left.len() && it_right != from_right.len() {
        let left = &from_left[it_left];
        let right = &from_right[it_right];
        let mut mid = if it_mid == from_mid.len() { None } else { Some(&from_mid[it_mid]) };
        if left.to_x <= min_x {
            it_left += 1;
            continue;
        }
        if right.to_x <= min_x {
            it_right += 1;
            continue;
        }
        if let Some(mid) = mid {
            if mid.to_x <= min_x {
                it_mid += 1;
                continue;
            }
        }
        let mut to_x = std::f64::MAX;
        if let Some(mid) = mid {
            to_x = fmin(to_x, mid.to_x as f64);
        }
        to_x = fmin(to_x, left.to_x);
        to_x = fmin(to_x, right.to_x);
        if let Some(mid2) = mid {
            if mid2.fr_x as f64 > min_x {
                to_x = fmin(to_x, mid2.fr_x);
                mid = None;
            }
        }
        combine_three_answers(left, right, mid, min_x, to_x, &mut res);
        min_x = to_x;
    }
    return res;
}

fn combine_three_answers(left: &Answer, right: &Answer, mid: Option<&Answer>, fr_x: f64, to_x: f64, res: &mut [f64]) {
    let mut better_point = if left.p == right.p { fr_x } else { get_mid_point(&left.p, &right.p) };
    better_point = fmax(better_point, fr_x);
    better_point = fmin(better_point, to_x);
    match mid {
        | None => {
            add_answer(fr_x, better_point, &left.p, res);
            add_answer(better_point, to_x, &right.p, res);
        }
        | Some(mid) => {
            combine_left_and_mid(left, mid, fr_x, better_point, res);
            combine_mid_and_right(mid, right, better_point, to_x, res);
        }
    }
}

const MAX_ITERS: usize = 50;

fn combine_left_and_mid(left: &Answer, mid: &Answer, fr_x: f64, to_x: f64, res: &mut [f64]) {
    let mut l = fr_x;
    let mut r = to_x;
    let h2 = (mid.p.y as f64).powi(2);
    for _ in 0..MAX_ITERS {
        let m = (l + r) / 2.0;
        let dist = left.p.get_d2_to(m);
        if dist < h2 {
            l = m;
        } else {
            r = m;
        }
    }
    let change = (l + r) / 2.0;
    add_answer(fr_x, change, &left.p, res);
    add_answer(change, to_x, &mid.p, res);
}

fn add_answer(fr_x: f64, to_x: f64, p: &Point, res: &mut [f64]) {
    assert!(fr_x <= to_x);
    res[p.id / 2] += to_x - fr_x;
}

fn combine_mid_and_right(mid: &Answer, right: &Answer, fr_x: f64, to_x: f64, res: &mut [f64]) {
    let mut l = fr_x;
    let mut r = to_x;
    let h2 = (mid.p.y as f64).powi(2);
    for _ in 0..MAX_ITERS {
        let m = (l + r) / 2.0;
        let dist = right.p.get_d2_to(m);
        if dist < h2 {
            r = m;
        } else {
            l = m;
        }
    }
    let change = (l + r) / 2.0;
    add_answer(fr_x, change, &mid.p, res);
    add_answer(change, to_x, &right.p, res);
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    id: usize,
}

impl Point {
    fn get_d2_to(&self, another_x: f64) -> f64 {
        let y = self.y as f64;
        let x = self.x as f64;
        y * y + (another_x - x) * (another_x - x)
    }
}

#[derive(Debug)]
struct Answer {
    fr_x: f64,
    to_x: f64,
    p: Point,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct ActiveSegment {
    y: i32,
    id: usize,
}

fn gen_mid_answer(a: &[Point], initial_points: &[Point]) -> Vec<Answer> {
// a should be sorted
    let mut answers = vec![];
    let mut active_segments: BTreeSet<ActiveSegment> = BTreeSet::new();
    let mut last_x = 0;
    for p in a.iter() {
        if last_x != p.x {
            if !active_segments.is_empty() {
                let best = active_segments.iter().next().unwrap().clone();
                answers.push(Answer { fr_x: last_x as f64, to_x: p.x as f64, p: initial_points[best.id] });
            }
            last_x = p.x;
        }
        if p.id % 2 == 0 {
            active_segments.insert(ActiveSegment { y: p.y, id: p.id });
        } else {
            active_segments.remove(&ActiveSegment { y: p.y, id: p.id - 1 });
        }
    }
    return answers;
}

fn get_mid_point(a: &Point, b: &Point) -> f64 {
    let dx = (b.x - a.x) as f64;
    let dy = (b.y - a.y) as f64;
    let xm = (a.x + b.x) as f64 / 2.0;
    let ym = (a.y + b.y) as f64 / 2.0;
    return xm + ym / dx * dy;
}

fn add_segment(mut fr_x: f64, to_x: f64, answers: &mut Vec<Answer>, stack: &mut VecDeque<Point>) {
    assert!(stack.len() > 0);
    loop {
        if stack.len() == 1 {
            answers.push(Answer { fr_x, to_x, p: stack[0] });
            return;
        } else {
            let p1 = &stack[0];
            let p2 = &stack[1];
            let p2_is_better_after = get_mid_point(p1, p2);
            if p2_is_better_after <= fr_x {
                stack.pop_front();
                continue;
            }
            if p2_is_better_after >= to_x {
                answers.push(Answer { fr_x, to_x, p: stack[0] });
                return;
            }
            answers.push(Answer { fr_x, to_x: p2_is_better_after, p: stack[0] });
            fr_x = p2_is_better_after;
            stack.pop_front();
        }
    }
}

fn add_point(stack: &mut VecDeque<Point>, p: &Point) {
    if stack.is_empty() {
        stack.push_back(p.clone());
    } else {
        if stack.back().unwrap().x == p.x {
            return;
        }
        while stack.len() >= 2 {
            let p2 = &stack[stack.len() - 2];
            let p1 = &stack[stack.len() - 1];
            if get_mid_point(p2, p1) > get_mid_point(p1, p) {
                stack.pop_back();
            } else {
                break;
            }
        }
        if stack.back().unwrap().y >= p.y {
            stack.pop_back();
        }
        stack.push_back(p.clone());
    }
}

fn solve_one_part(a: &mut Vec<Point>, mut min_x: i32, max_x: i32) -> Vec<Answer> {
    let mut answers = vec![];
    a.sort();
    let mut stack = VecDeque::new();
    add_point(&mut stack, &a[0]);

    for i in 1..a.len() {
        add_segment(min_x as f64, min(max_x, a[i].x) as f64, &mut answers, &mut stack);
        if a[i].x >= max_x {
            break;
        }
        min_x = a[i].x;
        add_point(&mut stack, &a[i]);
    }
    if min_x != max_x {
        add_segment(min_x as f64, max_x as f64, &mut answers, &mut stack);
    }
    return answers;
}

pub fn main() {
    let mut sc = Scanner::default();
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());


    let n: usize = sc.next();
    let max_x: i32 = sc.next();
    let mut a = vec![];
    for i in 0..n {
        let x: i32 = sc.next();
        let y: i32 = sc.next();
        let w: i32 = sc.next();
        let _h: i32 = sc.next();

        a.push(Point { x, y, id: i * 2 });
        a.push(Point { x: x + w, y, id: i * 2 + 1 });
    }
    let initial_pts = a.clone();
    let from_left = solve_one_part(&mut a, 0, max_x);
    let from_mid = gen_mid_answer(&a, &initial_pts);
    reverse_x(&mut a);
    let mut from_right = solve_one_part(&mut a, -max_x, 0);
    for an in from_right.iter_mut() {
        let new_fr_x = an.to_x * -1.0;
        let new_to_x = an.fr_x * -1.0;
        an.fr_x = new_fr_x;
        an.to_x = new_to_x;
        an.p.x *= -1;
    }
    reverse_x(&mut a);
    from_right.reverse();
    let res = combine_answers(n, &from_left, &from_right, &from_mid);
    for &r in res.iter() {
        let to_print = (r as f64) * 100.0 / (max_x as f64);
        writeln!(out, "{} ", to_print).unwrap();
    }
}

fn reverse_x(a: &mut [Point]) {
    for p in a.iter_mut() {
        p.x *= -1;
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