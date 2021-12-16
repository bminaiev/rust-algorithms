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

#[derive(Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn shift(&self, shift: Shift) -> Self {
        Self {
            x: (self.x as i32 + shift.dx) as usize,
            y: (self.y as i32 + shift.dy) as usize,
        }
    }
}

#[derive(Copy, Clone)]
struct Shift {
    dx: i32,
    dy: i32,
}

const DIRS: [u8; 4] = [b'N', b'W', b'S', b'E'];
const SHIFTS: [Shift; 4] = [
    Shift { dx: -1, dy: 0 },
    Shift { dx: 0, dy: -1 },
    Shift { dx: 1, dy: 0 },
    Shift { dx: 0, dy: 1 },
];

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let tc = sc.usize();
    for _ in 0..tc {
        let n = sc.usize();
        let m = sc.usize();
        let q = sc.usize();
        let mut s = vec![];
        for _ in 0..n {
            s.push(sc.string());
        }

        const M: usize = 12;

        let mut ok_shift = vec![vec![true; M * 2 + 1]; M * 2 + 1];

        for x1 in 0..n {
            for y1 in 0..m {
                if s[x1][y1] == b'B' {
                    for x2 in 0..n {
                        for y2 in 0..m {
                            if s[x2][y2] == b'A' {
                                ok_shift[x2 + M - x1][y2 + M - y1] = false;
                            }
                        }
                    }
                }
            }
        }
        assert!(ok_shift[M][M]);
        let mut cur = Pos { x: M, y: M };
        let moves = sc.string();

        let on_border = |p: Pos| -> bool { p.x == 0 || p.y == 0 || p.x == M * 2 || p.y == M * 2 };

        let mut ok = false;
        for &c in moves.iter() {
            let shift_pos = DIRS.iter().position(|&z| z == c).unwrap();
            let my_shift = SHIFTS[shift_pos];
            loop {
                let next = cur.shift(my_shift);
                if !ok_shift[next.x][next.y] {
                    break;
                }
                cur = next;
                if on_border(cur) {
                    ok = true;
                    break;
                }
            }
            if ok {
                break;
            }
        }
        if ok {
            writeln!(out, "TAK").unwrap();
        } else {
            writeln!(out, "NIE").unwrap();
        }
    }
}
