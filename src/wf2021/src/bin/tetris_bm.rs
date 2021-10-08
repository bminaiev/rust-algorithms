use std::io;
use std::io::Write;
use std::cmp::min;

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

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

#[derive(Copy, Clone, Default, Eq, PartialOrd, PartialEq, Debug)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn apply_shift(&self, shift: Shift) -> Self {
        Self { row: self.row + shift.d_row, col: self.col + shift.d_col }
    }
}

#[derive(Copy, Clone, Debug)]
struct Shift {
    d_row: i32,
    d_col: i32,
}

const ALL_SHIFTS: [Shift; 4] = [
    Shift { d_row: 1, d_col: 0 },
    Shift { d_row: -1, d_col: 0 },
    Shift { d_row: 0, d_col: 1 },
    Shift { d_row: 0, d_col: -1 },
];

#[derive(Debug)]
struct Figure {
    top_left: Pos,
    shifts: Vec<Shift>,
}

impl Figure {
    fn change_field(&self, field: &mut Field, value: u8) {
        for &shift in self.shifts.iter() {
            let cell = self.top_left.apply_shift(shift);
            set_cell(field, cell, value);
        }
    }

    fn change_field_empty(&self, field: &mut Field) {
        self.change_field(field, EMPTY);
    }

    fn change_field_back(&self, field: &mut Field) {
        self.change_field(field, USED);
    }

    fn can_put(&self, field: &Field, top_left: Pos) -> bool {
        for &shift in self.shifts.iter() {
            let cell = top_left.apply_shift(shift);
            if !inside(field, cell) {
                return false;
            }
            if get_cell(field, cell) != EMPTY {
                return false;
            }
        }
        return true;
    }
}

const EMPTY: u8 = b'.';
const USED: u8 = b'X';

type Field = Vec<Vec<u8>>;

fn inside(field: &Field, pos: Pos) -> bool {
    if pos.row < 0 || pos.col < 0 || pos.row >= field.len() as i32 || pos.col >= field[0].len() as i32 {
        return false;
    }
    return true;
}

fn get_cell(field: &Field, pos: Pos) -> u8 {
    if !inside(field, pos) {
        return EMPTY;
    }
    field[pos.row as usize][pos.col as usize]
}

fn set_cell(field: &mut Field, pos: Pos, value: u8) {
    field[pos.row as usize][pos.col as usize] = value;
}

fn find_figure(pos: Pos, field: &mut Field, res: &mut Vec<Pos>, required_val: u8) {
    if get_cell(field, pos) != required_val {
        return;
    }
    res.push(pos);
    field[pos.row as usize][pos.col as usize] = EMPTY;
    for &shift in ALL_SHIFTS.iter() {
        find_figure(pos.apply_shift(shift), field, res, required_val);
    }
}

fn build_figure_from_cells(cells: Vec<Pos>) -> Figure {
    let mut top_row = i32::MAX;
    let mut left_col = i32::MAX;
    for cell in cells.iter() {
        top_row = min(top_row, cell.row);
        left_col = min(left_col, cell.col);
    }
    let shifts: Vec<_> = cells.iter().map(|c| Shift { d_row: c.row - top_row, d_col: c.col - left_col }).collect();
    Figure { top_left: Pos { row: top_row, col: left_col }, shifts }
}

#[derive(Debug)]
struct Path {
    first_col: i32,
    path: Vec<u8>,
}

#[derive(Copy, Clone, Default, Debug)]
struct Prev {
    char: u8,
    prev: Pos,
}

struct DfsState {
    cur_iter: i32,
    used_iter: Vec<Vec<i32>>,
    prev: Vec<Vec<Prev>>,
}

impl DfsState {
    fn new(rows: usize, cols: usize) -> Self {
        Self { cur_iter: 1, used_iter: vec![vec![0; cols]; rows], prev: vec![vec![Prev::default(); cols]; rows] }
    }

    fn was(&self, pos: Pos) -> bool {
        if pos.row < 0 || pos.col < 0 {
            return true;
        }
        if self.used_iter[pos.row as usize][pos.col as usize] == self.cur_iter {
            return true;
        }
        return false;
    }

    fn mark(&mut self, pos: Pos, prev: Prev) {
        self.used_iter[pos.row as usize][pos.col as usize] = self.cur_iter;
        self.prev[pos.row as usize][pos.col as usize] = prev;
    }
}

struct Move {
    shift: Shift,
    char: u8,
}

const MOVES: [Move; 3] = [
    Move { shift: Shift { d_col: 0, d_row: -1 }, char: b'D' },
    Move { shift: Shift { d_col: -1, d_row: 0 }, char: b'R' },
    Move { shift: Shift { d_col: 1, d_row: 0 }, char: b'L' },
];

fn dfs_remove(field: &Field, figure: &Figure, cur_pos: Pos, prev: Prev, dfs_state: &mut DfsState) -> Option<Pos> {
    if !figure.can_put(field, cur_pos) {
        return None;
    }
    if dfs_state.was(cur_pos) {
        return None;
    }
    dfs_state.mark(cur_pos, prev);
    if cur_pos.row == 0 {
        return Some(cur_pos);
    }
    for move_ in MOVES.iter() {
        let next_pos = cur_pos.apply_shift(move_.shift);
        if let Some(res) = dfs_remove(field, figure, next_pos, Prev { char: move_.char, prev: cur_pos }, dfs_state) {
            return Some(res);
        }
    }
    return None;
}

fn try_remove(field: &mut Field, figure: &Figure, dfs_state: &mut DfsState) -> Option<Path> {
    figure.change_field_empty(field);
    dfs_state.cur_iter += 1;
    if let Some(end_pos) = dfs_remove(field, figure, figure.top_left, Prev { char: EMPTY, prev: figure.top_left }, dfs_state) {
        let mut path = vec![];
        let mut cur_pos = end_pos;
        while cur_pos != figure.top_left {
            let prev = dfs_state.prev[cur_pos.row as usize][cur_pos.col as usize];
            path.push(prev.char);
            cur_pos = prev.prev;
        }
        let mut col_in_first_row = i32::MAX;
        for shift in figure.shifts.iter() {
            if shift.d_row == 0 {
                col_in_first_row = min(col_in_first_row, shift.d_col);
            }
        }
        return Some(Path { first_col: end_pos.col + col_in_first_row, path });
    }
    figure.change_field_back(field);
    None
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let mut field = vec![];
    for _ in 0..n {
        field.push(sc.string());
    }
    let mut figures = vec![];
    for row in 0..n {
        for col in 0..m {
            let pos = Pos { row: row as i32, col: col as i32 };
            let cell = get_cell(&field, pos);
            if cell == EMPTY {
                continue;
            }
            let mut figure_cells = vec![];
            find_figure(pos, &mut field, &mut figure_cells, cell);
            let figure = build_figure_from_cells(figure_cells);
            figures.push(figure);
        }
    }
    for figure in figures.iter() {
        figure.change_field_back(&mut field);
    }
    let mut removed = vec![false; figures.len()];
    let mut dfs_state = DfsState::new(n, m);
    let mut res = vec![];
    loop {
        let mut all_removed = true;
        let mut changed_smth = false;
        let mut top_row = i32::MAX;
        for iter in 0..removed.len() {
            if removed[iter] {
                continue;
            }
            all_removed = false;
            let figure = &figures[iter];
            top_row = min(top_row, figure.top_left.row);
            const BUBEN: i32 = 8;
            if changed_smth && figure.top_left.row > top_row + BUBEN {
                break;
            }
            if let Some(path) = try_remove(&mut field, figure, &mut dfs_state) {
                changed_smth = true;
                removed[iter] = true;
                res.push(path);
            }
        }
        if all_removed {
            writeln!(out, "{}", res.len()).unwrap();
            res.reverse();
            for path in res.into_iter() {
                write!(out, "{} ", path.first_col + 1).unwrap();
                let mut str = path.path;
                str.push(b'S');
                writeln!(out, "{}", String::from_utf8(str).unwrap()).unwrap();
            }
            return;
        }
        if !changed_smth {
            writeln!(out, "-1").unwrap();
            return;
        }
    }
}
