use std::cmp::min;

use array2d::Array2D;
use rand::{thread_rng, Rng};
use std::time::Instant;

/*
  [dist] should contain [n] vectors each of length [n]

  dist[i][j] = initial distance between [i] and [j]

  After function returns, dist[i][j] contains shortest
  distance between [i] and [j], which could visit other
  vertices
*/
fn floyd_warshall_slow(dist: &mut Array2D<i32>) {
    let n = dist.row_len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                dist.set(
                    j,
                    k,
                    min(
                        *dist.get(j, k).unwrap(),
                        dist.get(j, i).unwrap() + dist.get(i, k).unwrap(),
                    ),
                ).unwrap();
            }
        }
    }
}

fn floyd_warshall_unsafe(dist: &mut [Vec<i32>]) {
    let n = dist.len();
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                unsafe {
                    *dist[j].get_unchecked_mut(k) = min(
                        *dist[j].get_unchecked(k),
                        dist[j].get_unchecked(i) + dist[i].get_unchecked(k),
                    )
                }
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
    for val in init_dists.elements_row_major_iter() {
        hash_sum ^= *val;
    }
    hash_sum
}

pub fn main() {
    let n = 750;
    let mut d = Array2D::filled_with(0, n, n);
    let mut rng = thread_rng();

    for _ in 1.. {
        for x in 0..n {
            for y in 0..n {
                d.set(x, y, rng.gen_range(0..1_000_000)).unwrap();
            }
            d.set(x, x, 0).unwrap();
        }
        // let hash_unsafe = test_one_algorithm(d.clone(), floyd_warshall_unsafe, "unsafe");
        let hash_slow = test_one_algorithm(d.clone(), floyd_warshall_slow, "slow");
        // assert_eq!(hash_slow, hash_unsafe);
        println!("{}", hash_slow);
    }
}
