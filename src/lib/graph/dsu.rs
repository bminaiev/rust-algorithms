struct Dsu {
    p: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        Self { p }
    }

    fn get(&mut self, v: usize) -> usize {
        if self.p[v] == v {
            return v;
        } else {
            self.p[v] = self.get(self.p[v]);
            return self.p[v];
        }
    }

    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        self.p[x] = y;
    }
}