use std::cmp::min;
use std::thread;
use std::time::{Duration, Instant};

#[allow(dead_code)]
struct Random {
    state: u64,
}

impl Random {
    fn next(&mut self) -> u64 {
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
        (from as u64 + self.next() % ((to - from) as u64)) as usize
    }

    #[allow(dead_code)]
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }

    #[allow(dead_code)]
    fn new(seed: u64) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }
}
use std::ops::{Index, IndexMut};

struct Array2D<T> {
    m: usize,
    v: Vec<T>,
}

impl<T> Array2D<T>
where
    T: Clone,
{
    fn new(empty: T, n: usize, m: usize) -> Self {
        Self {
            m,
            v: vec![empty; n * m],
        }
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
       unsafe {
           &self.v.get_unchecked((index) * self.m..(index + 1) * self.m)
       }
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[(index) * self.m..(index + 1) * self.m]
    }
}

fn calc(d: &mut Array2D<usize>) {
    let n = d.m;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                d[j][k] = min(d[j][k], d[j][i] + d[i][k]);
            }
        }
    }
}

pub fn main() {
    let n = 750;
    let mut d = Array2D::new(0, n, n);

    for iter in 1.. {
        let start = Instant::now();
        let mut rnd = Random::new(iter);
        for x in 0..n {
            for y in 0..n {
                d[x][y] = rnd.next_in_range(0, 1_000_000);
            }
        }
        calc(&mut d);
        let mut hash_sum = 0;
        for x in 0..n {
            for y in 0..n {
                hash_sum ^= d[x][y];
            }
        }
        println!(
            "iter: {}, debug: {}, time: {}ms",
            iter,
            hash_sum,
            start.elapsed().as_millis()
        );
        // assert!(start.elapsed().as_millis() < 500);
        // thread::sleep(Duration::from_millis(100));
    }
}
