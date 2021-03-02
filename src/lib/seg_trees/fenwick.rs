pub struct Fenwick {
    values: Vec<i64>
}

impl Fenwick {
    fn get_sum(&self, mut pos: usize) -> i64 {
        let mut res = 0i64;
        loop {
            res += self.values[pos] as i64;
            pos = pos & (pos + 1);
            if pos == 0 {
                return res;
            }
            pos -= 1;
        }
    }

    fn add(&mut self, mut pos: usize, change: i64) {
        while pos < self.values.len() {
            self.values[pos] += change;
            pos |= pos + 1;
        }
    }

    pub(crate) fn new(n: usize) -> Self {
        let values = vec![0; n];
        Fenwick { values }
    }
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use crate::lib::seg_trees::fenwick::Fenwick;

    #[test]
    fn stress() {
        let mut rnd = StdRng::from_rng(thread_rng()).unwrap();
        const MAX_N: usize = 100;
        const MAX_VAL: i32 = std::i32::MAX;
        const TESTS_N: usize = 100;

        for _ in 0..TESTS_N {
            let n: usize = rnd.gen_range(1..=MAX_N);
            let mut fenw = Fenwick::new(n);
            let mut slow_vec = vec![0i64; n];
            for _ in 0..TESTS_N {
                let pos = rnd.gen_range(0..n);
                if rnd.gen_bool(0.5) {
                    let sum_from_fenw = fenw.get_sum(pos);
                    let sum_slow = slow_vec[0..=pos].iter().sum();
                    assert_eq!(sum_from_fenw, sum_slow);
                } else {
                    let change = rnd.gen_range(0..MAX_VAL) as i64;
                    fenw.add(pos, change);
                    slow_vec[pos] += change;
                }
            }
        }
    }
}