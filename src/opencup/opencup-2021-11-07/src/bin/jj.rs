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


struct BitRevIterator {
    a: usize,
    n: usize,
}

impl BitRevIterator {
    fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());
        Self { a: 2 * n - 1, n }
    }
}

impl Iterator for BitRevIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.a == 2 * self.n - 2 {
            return None;
        }
        let mut mask = self.n;
        while self.a & mask > 0 {
            self.a ^= mask;
            mask /= 2;
        }
        self.a |= mask;
        Some(self.a / 2)
    }
}

#[allow(clippy::upper_case_acronyms)]
pub trait FFT: Sized + Copy {
    type F: Sized + Copy + From<Self> + Neg + Add<Output=Self::F> + Div<Output=Self::F> + Mul<Output=Self::F> + Sub<Output=Self::F>;

    const ZERO: Self;

    fn get_roots(n: usize, inverse: bool) -> Vec<Self::F>;
    fn get_factor(n: usize, inverse: bool) -> Self::F;
    fn extract(f: Self::F) -> Self;
}


pub use std::f64::consts::PI;
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

/// Fast iterative version of Euclid's GCD algorithm
pub fn fast_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}

/// Represents a fraction reduced to lowest terms
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Rational {
    pub num: i64,
    pub den: i64,
}

impl Rational {
    pub fn new(num: i64, den: i64) -> Self {
        let g = fast_gcd(num, den) * den.signum();
        Self {
            num: num / g,
            den: den / g,
        }
    }
    pub fn abs(self) -> Self {
        Self {
            num: self.num.abs(),
            den: self.den,
        }
    }
    pub fn recip(self) -> Self {
        let g = self.num.signum();
        Self {
            num: self.den / g,
            den: self.num / g,
        }
    }
}

impl From<i64> for Rational {
    fn from(num: i64) -> Self {
        Self { num, den: 1 }
    }
}

impl Neg for Rational {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            num: -self.num,
            den: self.den,
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add for Rational {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(
            self.num * other.den + self.den * other.num,
            self.den * other.den,
        )
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub for Rational {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.num * other.den - self.den * other.num,
            self.den * other.den,
        )
    }
}

impl Mul for Rational {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.num * other.num, self.den * other.den)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Rational {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.num * other.den).cmp(&(self.den * other.num))
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Represents a complex number using floating-point arithmetic
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    pub fn from_polar(r: f64, th: f64) -> Self {
        Self::new(r * th.cos(), r * th.sin())
    }
    pub fn abs_square(self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    pub fn argument(self) -> f64 {
        self.imag.atan2(self.real)
    }
    pub fn conjugate(self) -> Self {
        Self::new(self.real, -self.imag)
    }
    pub fn recip(self) -> Self {
        let denom = self.abs_square();
        Self::new(self.real / denom, -self.imag / denom)
    }
}

impl From<f64> for Complex {
    fn from(real: f64) -> Self {
        Self::new(real, 0.0)
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.real, -self.imag)
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imag + other.imag)
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.real - other.real, self.imag - other.imag)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.imag * other.real + self.real * other.imag;
        Self::new(real, imag)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}

/// Represents an element of the finite (Galois) field of prime order M, where
/// 1 <= M < 2^31.5. If M is not prime, ring operations are still valid
/// but recip() and division are not. Note that the latter operations are also
/// the slowest, so precompute any inverses that you intend to use frequently.
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Modulo<const M: i64> {
    pub val: i64,
}

impl<const M: i64> Modulo<M> {
    /// Computes self^n in O(log n) time
    pub fn pow(mut self, mut n: u64) -> Self {
        let mut result = Self::from_small(1);
        while n > 0 {
            if n % 2 == 1 {
                result = result * self;
            }
            self = self * self;
            n /= 2;
        }
        result
    }
    /// Computes inverses of 1 to n in O(n) time
    pub fn vec_of_recips(n: i64) -> Vec<Self> {
        let mut recips = vec![Self::from(0), Self::from(1)];
        for i in 2..=n {
            let (md, dv) = (M % i, M / i);
            recips.push(recips[md as usize] * Self::from_small(-dv));
        }
        recips
    }
    /// Computes self^-1 in O(log M) time
    pub fn recip(self) -> Self {
        self.pow(M as u64 - 2)
    }


    fn from_small(s: i64) -> Self {
        let val = if s < 0 { s + M } else { s };
        Self { val }
    }
}

impl<const M: i64> From<i64> for Modulo<M> {
    fn from(val: i64) -> Self {
        // Self { val: val.rem_euclid(M) }
        Self::from_small(val % M)
    }
}

impl<const M: i64> Neg for Modulo<M> {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from_small(-self.val)
    }
}

impl<const M: i64> Add for Modulo<M> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from_small(self.val + other.val - M)
    }
}

impl<const M: i64> Sub for Modulo<M> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from_small(self.val - other.val)
    }
}

impl<const M: i64> Mul for Modulo<M> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::from(self.val * other.val)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl<const M: i64> Div for Modulo<M> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.recip()
    }
}

/// Prime modulus that's commonly used in programming competitions
pub const COMMON_PRIME: i64 = 998_244_353;

// 2^23 * 7 * 17 + 1;
pub type CommonField = Modulo<COMMON_PRIME>;

#[derive(Clone, PartialEq, Debug)]
pub struct Matrix {
    cols: usize,
    inner: Box<[f64]>,
}

impl Matrix {
    pub fn zero(rows: usize, cols: usize) -> Self {
        let inner = vec![0.0; rows * cols].into_boxed_slice();
        Self { cols, inner }
    }
    pub fn one(cols: usize) -> Self {
        let mut matrix = Self::zero(cols, cols);
        for i in 0..cols {
            matrix[i][i] = 1.0;
        }
        matrix
    }
    pub fn vector(vec: &[f64], as_row: bool) -> Self {
        let cols = if as_row { vec.len() } else { 1 };
        let inner = vec.to_vec().into_boxed_slice();
        Self { cols, inner }
    }
    pub fn pow(&self, mut n: u64) -> Self {
        let mut base = self.clone();
        let mut result = Self::one(self.cols);
        while n > 0 {
            if n % 2 == 1 {
                result = &result * &base;
            }
            base = &base * &base;
            n /= 2;
        }
        result
    }
    pub fn rows(&self) -> usize {
        self.inner.len() / self.cols
    }
    pub fn transpose(&self) -> Self {
        let mut matrix = Matrix::zero(self.cols, self.rows());
        for i in 0..self.rows() {
            for j in 0..self.cols {
                matrix[j][i] = self[i][j];
            }
        }
        matrix
    }
    pub fn recip(&self) -> Self {
        unimplemented!();
    }
}

impl Index<usize> for Matrix {
    type Output = [f64];
    fn index(&self, row: usize) -> &Self::Output {
        let start = self.cols * row;
        &self.inner[start..start + self.cols]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = self.cols * row;
        &mut self.inner[start..start + self.cols]
    }
}

impl Neg for &Matrix {
    type Output = Matrix;
    fn neg(self) -> Matrix {
        let inner = self.inner.iter().map(|&v| -v).collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}

impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, other: Self) -> Matrix {
        let self_iter = self.inner.iter();
        let inner = self_iter.zip(other.inner.iter()).map(|(&u, &v)| u + v).collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}

impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, other: Self) -> Matrix {
        let self_iter = self.inner.iter();
        let inner = self_iter.zip(other.inner.iter()).map(|(&u, &v)| u - v).collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;
    fn mul(self, scalar: f64) -> Matrix {
        let inner = self.inner.iter().map(|&v| v * scalar).collect();
        Matrix {
            cols: self.cols,
            inner,
        }
    }
}

impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, other: Self) -> Matrix {
        assert_eq!(self.cols, other.rows());
        let mut matrix = Matrix::zero(self.rows(), other.cols);
        for i in 0..self.rows() {
            for k in 0..self.cols {
                for j in 0..other.cols {
                    matrix[i][j] += self[i][k] * other[k][j];
                }
            }
        }
        matrix
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rational() {
        let three = Rational::from(3);
        let six = Rational::from(6);
        let three_and_half = three + three / six;

        assert_eq!(three_and_half.num, 7);
        assert_eq!(three_and_half.den, 2);
        assert_eq!(three_and_half, Rational::new(-35, -10));
        assert!(three_and_half > Rational::from(3));
        assert!(three_and_half < Rational::from(4));

        let minus_three_and_half = six - three_and_half + three / (-three / six);
        let zero = three_and_half + minus_three_and_half;

        assert_eq!(minus_three_and_half.num, -7);
        assert_eq!(minus_three_and_half.den, 2);
        assert_eq!(three_and_half, -minus_three_and_half);
        assert_eq!(zero.num, 0);
        assert_eq!(zero.den, 1);
    }

    #[test]
    fn test_complex() {
        let four = Complex::new(4.0, 0.0);
        let two_i = Complex::new(0.0, 2.0);

        assert_eq!(four / two_i, -two_i);
        assert_eq!(two_i * -two_i, four);
        assert_eq!(two_i - two_i, Complex::from(0.0));
        assert_eq!(four.abs_square(), 16.0);
        assert_eq!(two_i.abs_square(), 4.0);
        assert_eq!((-four).argument(), -PI);
        assert_eq!((-two_i).argument(), -PI / 2.0);
        assert_eq!(four.argument(), 0.0);
        assert_eq!(two_i.argument(), PI / 2.0);
    }

    #[test]
    fn test_field() {
        let base = CommonField::from(1234);
        let zero = base - base;
        let one = base.recip() * base;
        let two = CommonField::from(2 - 5 * COMMON_PRIME);

        assert_eq!(zero.val, 0);
        assert_eq!(one.val, 1);
        assert_eq!(one + one, two);
        assert_eq!(one / base * (base * base) - base / one, zero);
    }

    #[test]
    fn test_vec_of_recips() {
        let recips = CommonField::vec_of_recips(20);

        assert_eq!(recips.len(), 21);
        for i in 1..recips.len() {
            assert_eq!(recips[i], CommonField::from(i as i64).recip());
        }
    }
}


impl FFT for f64 {
    type F = Complex;

    const ZERO: f64 = 0.0;

    fn get_roots(n: usize, inverse: bool) -> Vec<Self::F> {
        let step = if inverse { -2.0 } else { 2.0 } * PI / n as f64;
        (0..n / 2).map(|i| Complex::from_polar(1.0, step * i as f64)).collect()
    }

    fn get_factor(n: usize, inverse: bool) -> Self::F {
        Self::F::from(if inverse { (n as f64).recip() } else { 1.0 })
    }

    fn extract(f: Self::F) -> f64 {
        f.real
    }
}

impl FFT for i64 {
    type F = CommonField;

    const ZERO: Self = 0;

    fn get_roots(n: usize, inverse: bool) -> Vec<Self::F> {
        assert!(n <= 1 << 23);
        let mut prim_root = Self::F::from(15_311_432);
        if inverse {
            prim_root = prim_root.recip();
        }
        for _ in (0..).take_while(|&i| n < 1 << (23 - i)) {
            prim_root = prim_root * prim_root;
        }

        let mut roots = Vec::with_capacity(n / 2);
        let mut root = Self::F::from(1);
        for _ in 0..roots.capacity() {
            roots.push(root);
            root = root * prim_root;
        }
        roots
    }

    fn get_factor(n: usize, inverse: bool) -> Self::F {
        Self::F::from(if inverse { n as Self } else { 1 }).recip()
    }

    fn extract(f: Self::F) -> Self {
        f.val
    }
}

/// Computes the discrete fourier transform of v, whose length is a power of 2.
/// Forward transform: polynomial coefficients -> evaluate at roots of unity
/// Inverse transform: values at roots of unity -> interpolated coefficients
pub fn fft<T: FFT>(v: &[T::F], inverse: bool) -> Vec<T::F> {
    let n = v.len();
    assert!(n.is_power_of_two());

    let factor = T::get_factor(n, inverse);
    let roots_of_unity = T::get_roots(n, inverse);
    let mut dft = BitRevIterator::new(n).map(|i| v[i] * factor).collect::<Vec<_>>();

    for m in (0..).map(|s| 1 << s).take_while(|&m| m < n) {
        for k in (0..n).step_by(2 * m) {
            for j in 0..m {
                let u = dft[k + j].clone();
                let t = dft[k + j + m].clone() * roots_of_unity[n / 2 / m * j];
                dft[k + j] = u + t;
                dft[k + j + m] = u - t;
            }
        }
    }
    dft
}

/// From a slice of reals (f64 or i64), computes DFT of size at least desired_len
pub fn dft_from_reals<T: FFT>(v: &[T], desired_len: usize) -> Vec<T::F> {
    assert!(v.len() <= desired_len);

    let complex_v = v.iter().cloned().chain(std::iter::repeat(T::ZERO)).take(desired_len.next_power_of_two()).map(T::F::from).collect::<Vec<_>>();
    fft::<T>(&complex_v, false)
}

/// The inverse of dft_from_reals()
pub fn idft_to_reals<T: FFT>(dft_v: &[T::F], desired_len: usize) -> Vec<T> {
    assert!(dft_v.len() >= desired_len);

    let complex_v = fft::<T>(dft_v, true);
    complex_v.into_iter().take(desired_len).map(T::extract).collect()
}

/// Given two polynomials (vectors) sum_i a[i] x^i and sum_i b[i] x^i,
/// computes their product (convolution) c[k] = sum_(i+j=k) a[i]*b[j].
/// Uses complex FFT if inputs are f64, or modular NTT if inputs are i64.
pub fn convolution<T: FFT>(a: &[T], b: &[T]) -> Vec<T> {
    let len_c = a.len() + b.len() - 1;
    let dft_a = dft_from_reals(a, len_c).into_iter();
    let dft_b = dft_from_reals(b, len_c).into_iter();
    let dft_c = dft_a.zip(dft_b).map(|(a, b)| a * b).collect::<Vec<_>>();
    idft_to_reals(&dft_c, len_c)
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/


const WALL: u8 = b'#';
const EMPTY: u8 = b'.';

type Field = Vec<Vec<u8>>;

fn calc_bad(s: &Field, t: &Field, first: u8, second: u8) -> Vec<f64> {
    let n = s.len();
    let m = s[0].len();

    let size = 2 * n * m;

    let mut poly_a = vec![0.0; size];

    for i in 0..n {
        for j in 0..m {
            if s[i][j] == first {
                let pos = 2 * m * i + j;
                poly_a[pos] = 1.0;
            }
        }
    }

    let mut poly_b = vec![0.0; size];

    for i in 0..t.len() {
        for j in 0..t[0].len() {
            if t[i][j] == second {
                let pos = 2 * m * (n - 1 - i) + (m - 1 - j);
                poly_b[pos] = 1.0;
            }
        }
    }

    convolution(&poly_a, &poly_b)
}

struct Pos {
    row: usize,
    col: usize,
}

pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let n = sc.usize();
    let m = sc.usize();
    let a = sc.usize();
    let b = sc.usize();
    let mut s = vec![];
    for _ in 0..n {
        s.push(sc.string());
    }
    let mut t = vec![];
    for _ in 0..a {
        t.push(sc.string());
    }

    let sum1 = calc_bad(&s, &t, WALL, EMPTY);
    let sum2 = calc_bad(&s, &t, EMPTY, WALL);

    let mut res = vec![];

    const EPS : f64 = 0.5;

    for i in 0..=(n - a) {
        for j in 0..=(m - b) {
            let pos = 2 * m * (n - 1 + i) + (m - 1 + j);
            if sum1[pos] + sum2[pos] <= EPS {
                res.push(Pos { row: i + 1, col: j + 1 });
            }
        }
    }
    writeln!(out, "{}", res.len()).unwrap();
    for cell in res.iter() {
        writeln!(out, "{} {}", cell.row, cell.col).unwrap();
    }
}
