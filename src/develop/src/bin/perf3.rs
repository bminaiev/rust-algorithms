use std::cmp::min;

use rand::{thread_rng, Rng};
use std::time::Instant;

/*
  [dist] should contain [n] vectors each of length [n]

  dist[i][j] = initial distance between [i] and [j]

  After function returns, dist[i][j] contains shortest
  distance between [i] and [j], which could visit other
  vertices
*/
fn floyd_warshall_slow(dist: &mut [Vec<i32>]) {
    let n = dist.len();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (dist_j, dist_i) = if j < i {
                let (lo, hi) = dist.split_at_mut(i);
                (&mut lo[j][..n], &mut hi[0][..n])
            } else {
                let (lo, hi) = dist.split_at_mut(j);
                (&mut hi[0][..n], &mut lo[i][..n])
            };
            let dist_ji = dist_j[i];
            for k in 0..n {
                dist_j[k] = min(dist_j[k], dist_ji + dist_i[k]);
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
    mut init_dists: Vec<Vec<i32>>,
    algo: fn(&mut [Vec<i32>]) -> (),
    name: &str,
) -> i32 {
    let start = Instant::now();
    algo(&mut init_dists);
    let elapsed = start.elapsed().as_millis();
    println!("Algo {} finished in {}ms", name, elapsed);
    let mut hash_sum = 0;
    for row in init_dists.iter() {
        for val in row.iter() {
            hash_sum ^= *val;
        }
    }
    hash_sum
}

pub fn main() {
    let n = 750;
    let mut d = vec![vec![0; n]; n];
    let mut rng = thread_rng();

    for _ in 1.. {
        for x in 0..n {
            for y in 0..n {
                d[x][y] = rng.gen_range(0..1_000_000);
            }
            d[x][x] = 0;
        }
        let hash_unsafe = test_one_algorithm(d.clone(), floyd_warshall_unsafe, "unsafe");
        let hash_slow = test_one_algorithm(d.clone(), floyd_warshall_slow, "slow");
        assert_eq!(hash_slow, hash_unsafe);
        println!("{}", hash_unsafe);
    }
}
