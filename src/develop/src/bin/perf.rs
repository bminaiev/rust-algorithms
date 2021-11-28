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

fn calc(d: &mut [Vec<i32>]) {
    let n = d.len();
    for i in 0..n {
        assert_eq!(d[i].len(), n);
        for j in 0..n {
            assert_eq!(d[j].len(), n);
            for k in 0..d[j].len() {
                unsafe {
                    *d[j].get_unchecked_mut(k) = min(
                        *d[j].get_unchecked(k),
                        d[j].get_unchecked(i) + d[i].get_unchecked(k),
                    )
                }
                // d[j][k] = min(d[j][k], d[j][i] + d[i][k]);
            }
        }
    }
}

pub fn main() {
    let n = 750;
    let mut d = vec![vec![0; n]; n];

    for iter in 1.. {
        let start = Instant::now();
        let mut rnd = Random::new(iter);
        for x in 0..n {
            for y in 0..n {
                d[x][y] = rnd.next_in_range(0, 1_000_000) as i32;
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
