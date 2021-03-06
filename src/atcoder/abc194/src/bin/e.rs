#[derive(Default)]
struct Scanner {
    buffer: Vec<String>,
}

// Scanner code is copied from Russell Emerine's solution
// http://codeforces.com/contest/1477/submission/105755265
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    #[allow(dead_code)]
    fn next_string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}

pub fn main() {
    let mut scanner = Scanner::default();
    let n: usize = scanner.next();
    let m: usize = scanner.next();
    let a: Vec<usize> = (0..n).map(|_| scanner.next()).collect();
    let mut positions = vec![Vec::<usize>::new(); n];
    for i in 0..n {
        positions[i].push(0);
    }
    for (pos, &val) in a.iter().enumerate() {
        positions[val].push(pos + 1);
    }
    for i in 0..n {
        positions[i].push(n + 1);
    }
    for val in 0..n {
        let mut ok = true;
        for pos in positions[val].windows(2) {
            if pos[1] - pos[0] > m {
                ok = false;
                break;
            }
        }
        if !ok {
            println!("{}", val);
            return;
        }
    }
    println!("{}", n);
}