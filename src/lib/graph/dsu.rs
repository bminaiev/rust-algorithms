struct Dsu {
    p: Vec<usize>,
}

impl Dsu {
    #[allow(unused)]
    fn new(n: usize) -> Self {
        let p = (0..n).collect();
        Self { p }
    }

    fn get(&mut self, v: usize) -> usize {
        return if self.p[v] == v {
            v
        } else {
            self.p[v] = self.get(self.p[v]);
            self.p[v]
        }
    }

    #[allow(unused)]
    fn unite(&mut self, mut x: usize, mut y: usize) {
        x = self.get(x);
        y = self.get(y);
        self.p[x] = y;
    }
}
