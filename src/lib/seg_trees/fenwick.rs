pub struct Fenwick {
    pub values: Vec<i32>
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

    fn add(&mut self, mut pos: usize, change: i32) {
        while pos < self.values.len() {
            self.values[pos] += change;
            pos |= pos + 1;
        }
    }

    pub fn new(n: usize) -> Self {
        let values = vec![0; n];
        Fenwick { values }
    }
}
