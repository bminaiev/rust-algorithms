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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn double_mid() {
        let mut rnd = Random::new(787788);
        const CNT: usize = 10_000;
        let mut sum = 0.0;
        for _ in 0..CNT {
            let val = rnd.next_double();
            assert!(val >= 0.0);
            assert!(val <= 1.0);
            sum += val;
        }
        let average = sum / (CNT as f64);
        assert!((average - 0.5).abs() < 0.05);
    }
}
