#[allow(dead_code)]
struct Random {
    state: usize
}

impl Random {
    fn next(&mut self) -> usize {
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
        from + self.next() % (to - from)
    }

    #[allow(dead_code)]
    fn new(seed: usize) -> Self {
        Random { state: seed }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::utils::random::Random;

    #[test]
    fn distribution() {
        const LEN: usize = 10;
        const FROM: usize = 15;
        const NUMBERS: usize = 1000000;
        let mut counts = vec![0; LEN];
        let mut rnd = Random::new(787788);
        for _ in 0..NUMBERS {
            let val = rnd.next_in_range(FROM, FROM + LEN);
            assert!(val >= FROM);
            assert!(val < FROM + LEN);
            counts[val - FROM] += 1;
        }
        let min = *counts.iter().min().unwrap();
        let max = *counts.iter().max().unwrap();
        assert!((max - min) as f64 / (max as f64) < 0.05);
    }
}