use std::io;
use std::io::Write;
use std::ops::{Mul, Add, Sub, MulAssign};

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

#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    const ZERO: Self = Complex { real: 0.0, imag: 0.0 };
    const ONE: Self = Complex { real: 1.0, imag: 0.0 };
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

mod fft {
    use super::*;

    fn fft(a: &mut Vec<Complex>, invert: bool) {
        let n = a.len();
        assert!(n.is_power_of_two());
        let shift = usize::BITS - n.trailing_zeros();
        for i in 1..n {
            let j = (i << shift).reverse_bits();
            assert!(j < n);
            if i < j {
                a.swap(i, j);
            }
        }
        for len in (1..).map(|x| 1 << x).take_while(|s| *s <= n) {
            let half = len / 2;
            let alpha = std::f64::consts::PI * 2.0 / (len as f64);
            let cos = f64::cos(alpha);
            let sin = f64::sin(alpha) * (if invert { -1.0 } else { 1.0 });
            let complex_angle = Complex { real: cos, imag: sin };
            for start in (0..n).step_by(len) {
                let mut mult = Complex::ONE;
                for j in 0..half {
                    let u = a[start + j];
                    let v = a[start + half + j] * mult;
                    a[start + j] = u + v;
                    a[start + j + half] = u - v;
                    mult *= complex_angle;
                }
            }
        }
        if invert {
            for i in 0..n {
                let n = n as f64;
                a[i].imag /= n;
                a[i].real /= n;
            }
        }
    }

    #[allow(unused)]
    pub(crate) fn multiply_raw(mut a: Vec<Complex>, mut b: Vec<Complex>) -> Vec<Complex> {
        assert!(a.len().is_power_of_two());
        assert!(b.len().is_power_of_two());
        assert_eq!(a.len(), b.len());
        fft(&mut a, false);
        fft(&mut b, false);
        for (x, y) in a.iter_mut().zip(b.iter()) {
            *x *= *y;
        }
        fft(&mut a, true);
        a
    }

    #[allow(unused)]
    pub(crate) fn multiply(mut a: Vec<Complex>, mut b: Vec<Complex>) -> Vec<Complex> {
        let expected_size = (a.len() + b.len() - 1).next_power_of_two();
        a.resize(expected_size, Complex::ZERO);
        b.resize(expected_size, Complex::ZERO);
        multiply_raw(a, b)
    }
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/

const WALL: u8 = b'#';
const EMPTY: u8 = b'.';

const REAL: Complex = Complex { real: 1.0, imag: 0.0 };
const IMG: Complex = Complex { real: 0.0, imag: 1.0 };
const IMG2: Complex = Complex { real: 0.0, imag: -1.0 };

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let a = sc.usize();
    let b = sc.usize();

    const M: usize = 2048;
    let size = 2 * M * M;

    let mut poly_a = vec![Complex::ZERO; size];
    let mut poly_b = vec![Complex::ZERO; size];

    for i in 0..n {
        let s = sc.string();
        for j in 0..m {
            let pos = (2 * m * i + j) % size;
            if s[j] == WALL {
                poly_a[pos] = REAL;
            } else if s[j] == EMPTY {
                poly_a[pos] = IMG;
            }
        }
    }

    for i in 0..a {
        let s = sc.string();
        for j in 0..b {
            let pos = (2 * m * (n - 1 - i) + (m - 1 - j)) % size;
            if s[j] == EMPTY {
                poly_b[pos] = REAL;
            } else if s[j] == WALL {
                poly_b[pos] = IMG2;
            }
        }
    }

    let sum1 = fft::multiply_raw(poly_a, poly_b);

    const EPS: f64 = 0.5;

    let mut cnt = 0;
    for i in 0..=(n - a) {
        for j in 0..=(m - b) {
            let pos = (2 * m * (n - 1 + i) + (m - 1 + j)) % size;
            if sum1[pos].real <= EPS {
                cnt += 1;
            }
        }
    }
    writeln!(out, "{}", cnt).unwrap();
    for i in 0..=(n - a) {
        for j in 0..=(m - b) {
            let pos = (2 * m * (n - 1 + i) + (m - 1 + j)) % size;
            if sum1[pos].real <= EPS {
                writeln!(out, "{} {}", i + 1, j + 1).unwrap();
            }
        }
    }
}
