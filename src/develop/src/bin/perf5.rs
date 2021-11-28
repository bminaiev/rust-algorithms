use std::cmp::min;

use rand::{thread_rng, Rng};
use std::time::Instant;

use std::ops::{Index, IndexMut};

#[derive(Clone)]
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
        &self.v[(index) * self.m..(index + 1) * self.m]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[(index) * self.m..(index + 1) * self.m]
    }
}

/*
  [dist] should contain [n] vectors each of length [n]

  dist[i][j] = initial distance between [i] and [j]

  After function returns, dist[i][j] contains shortest
  distance between [i] and [j], which could visit other
  vertices
*/
fn floyd_warshall(dist: &mut Array2D<i32>) {
    let n = dist.m;
    for i in 0..n {
        for j in 0..n {
            let dist_ji = dist[j][i];
            for k in 0..n {
                dist[j][k] = min(dist[j][k], dist_ji + dist[i][k]);
            }
        }
    }
}

// Returns hash of final dists
fn test_one_algorithm(
    mut init_dists: Array2D<i32>,
    algo: fn(&mut Array2D<i32>) -> (),
    name: &str,
) -> i32 {
    let start = Instant::now();
    algo(&mut init_dists);
    let elapsed = start.elapsed().as_millis();
    println!("Algo {} finished in {}ms", name, elapsed);
    let mut hash_sum = 0;
    for row in 0..init_dists.m {
        for val in init_dists[row].iter() {
            hash_sum ^= *val;
        }
    }
    hash_sum
}

pub fn main() {
    let n = 1000;
    let mut d = Array2D::new(0, n, n);
    let mut rng = thread_rng();

    for _ in 1.. {
        for x in 0..n {
            for y in 0..n {
                d[x][y] = rng.gen_range(0..1_000_000);
            }
            d[x][x] = 0;
        }
        let hash = test_one_algorithm(d.clone(), floyd_warshall, "slow");
        println!("hash: {}", hash);
    }
}
