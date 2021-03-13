use std::hash::{Hasher, BuildHasherDefault};

#[derive(Default)]
#[allow(dead_code)]
struct IntHasher {
    value: u64
}


#[allow(dead_code)]
impl Hasher for IntHasher {
    fn finish(&self) -> u64 {
        self.value
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes.iter() {
            self.value = self.value.wrapping_mul(934_475_254_358_498_081);
            self.value = self.value.wrapping_add(b as u64);
        }
    }
}

#[allow(dead_code)]
type IntBuildHasher = BuildHasherDefault<IntHasher>;


#[cfg(test)]
mod tests {
    use rand::prelude::StdRng;
    use rand::{SeedableRng, RngCore, Rng};
    use std::collections::HashSet;
    use super::*;

    const MAX: usize = 3_000_000;

    #[test]
    fn simple_hashmap() {
        let mut rnd = StdRng::seed_from_u64(787788);
        let vals = (0..MAX).map(|_| rnd.next_u64()).collect::<Vec<_>>();
        let start_time = std::time::Instant::now();
        let mut map = HashSet::new();
        for _ in 0..MAX {
            let random_val = vals[rnd.gen_range(0..vals.len())];
            map.insert(random_val);
        }
        let mut found_total = 0;
        for _ in 0..MAX {
            let random_val = vals[rnd.gen_range(0..vals.len())];
            if map.contains(&random_val) {
                found_total += 1;
            }
        }
        println!("found total: {}, consumed: {}ms", found_total, start_time.elapsed().as_millis());
    }

    #[test]
    fn redefine_hasher_hashmap() {
        let mut rnd = StdRng::seed_from_u64(787788);
        let vals = (0..MAX).map(|_| rnd.next_u64()).collect::<Vec<_>>();
        let start_time = std::time::Instant::now();
        let mut map = HashSet::with_hasher(IntBuildHasher::default());
        for _ in 0..MAX {
            let random_val = vals[rnd.gen_range(0..vals.len())];
            map.insert(random_val);
        }
        let mut found_total = 0;
        for _ in 0..MAX {
            let random_val = vals[rnd.gen_range(0..vals.len())];
            if map.contains(&random_val) {
                found_total += 1;
            }
        }
        println!("found total: {}, consumed: {}ms", found_total, start_time.elapsed().as_millis());
    }
}