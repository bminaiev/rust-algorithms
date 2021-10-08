use std::io;
use std::io::Write;

/**************************************************

    START OF TEMPLATE CODE

 *************************************************/
#[allow(unused_macros)]
macro_rules! dbg {
    ($first_val:expr, $($val:expr),+ $(,)?) => {
        eprint!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($first_val), &$first_val);
        ($(eprint!(", {} = {:?}", stringify!($val), &$val)),+,);
        eprintln!();
    };
    ($first_val:expr) => {
        eprintln!("[{}:{}] {} = {:?}",
                    file!(), line!(), stringify!($first_val), &$first_val);
    };
}

enum InputSource {
    Stdin,
    FromFile(Vec<String>),
}

struct Scanner {
    buffer: Vec<String>,
    input_source: InputSource,
}


impl Scanner {
    #[allow(dead_code)]
    fn new() -> Self {
        Self { buffer: vec![], input_source: InputSource::Stdin }
    }

    #[allow(dead_code)]
    fn new_file(filename: &str) -> Self {
        let file = std::fs::read_to_string(filename).unwrap();
        let mut lines: Vec<String> = file.lines().map(|line| String::from(line)).collect();
        lines.reverse();
        Self { buffer: vec![], input_source: InputSource::FromFile(lines) }
    }


    #[allow(dead_code)]
    fn i64(&mut self) -> i64 {
        self.next::<i64>()
    }

    #[allow(dead_code)]
    fn i32(&mut self) -> i32 {
        self.next::<i32>()
    }

    #[allow(dead_code)]
    fn usize(&mut self) -> usize {
        self.next::<usize>()
    }

    #[allow(dead_code)]
    fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

    fn parse_next_line(&mut self) -> bool {
        let mut input = String::new();
        match &mut self.input_source {
            | InputSource::Stdin => {
                if std::io::stdin().read_line(&mut input).expect("Failed read") == 0 {
                    return false;
                }
            }
            | InputSource::FromFile(lines) => {
                match lines.pop() {
                    Some(line) => input = line,
                    None => return false,
                }
            }
        }

        self.buffer = input.split_whitespace().rev().map(String::from).collect();
        return true;
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }

            self.parse_next_line();
        }
    }

    #[allow(dead_code)]
    fn has_more_elements(&mut self) -> bool {
        loop {
            if !self.buffer.is_empty() {
                return true;
            }
            if !self.parse_next_line() {
                return false;
            }
        }
    }


    #[allow(dead_code)]
    fn string(&mut self) -> Vec<u8> {
        self.next::<String>().into_bytes()
    }
}

/**************************************************

    END OF TEMPLATE CODE

 *************************************************/


#[allow(dead_code)]
struct Random {
    state: usize,
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
    fn next_double(&mut self) -> f64 {
        (self.next() as f64) / (std::usize::MAX as f64)
    }

    #[allow(dead_code)]
    fn new(seed: usize) -> Self {
        assert_ne!(seed, 0);
        Self {
            state: seed,
        }
    }
}

fn count(g: &Vec<Vec<bool>>) -> usize {
    let n = g.len();
    // [from][to][mask]
    let mut dp = vec![vec![vec![false; 1 << n]; n]; n];
    for i in 0..n {
        dp[i][i][1 << i] = true;
    }
    for mask in 0..1 << n {
        for fr in 0..n {
            for to in 0..n {
                if ((1 << fr) & mask) != 0 {
                    if ((1 << to) & mask) != 0 {
                        if !dp[fr][to][mask] {
                            continue;
                        }
                        for go in 0..n {
                            if ((1 << go) & mask) == 0 && g[to][go] {
                                let nmask = mask | (1 << go);
                                dp[fr][go][nmask] = true;
                            }
                        }
                    }
                }
            }
        }
    }
    let mut cnt = 0;
    let full_mask = (1 << n) - 1;
    for i in 0..n {
        for j in i + 1..n {
            if dp[i][j][full_mask] {
                cnt += 1;
            }
        }
    }
    cnt
}


const MAX_K: usize = 60;
const precalc_seeds: [usize; MAX_K + 1] = [0, 13670721290488359750, 873076716418189950, 1174356152888052838, 744745430862514248, 2837667632778463652, 4354440581333875080, 4710554660166565845, 17012798532317222089, 534363483160844598, 11569279363167720448, 10432370810517487955, 13960082585383542849, 3420512756345608293, 18221674579461622774, 12662350636124761167, 18244563871591599327, 11163898317203062252, 2354555596730232721, 4400354661170695304, 7319763634354985574, 5536363701401595025, 7490754587575785255, 15300992393390547857, 9720216808043834935, 14000914875863123289, 7322313776398596994, 6523303905471949679, 2635925964672488495, 901196658747401080, 1812000731500895940, 15025945828088780482, 517373355575899411, 13110746547873348997, 17047206703974379636, 3462008825611853469, 909676994216277395, 16038752794297909509, 6474048706180744437, 14715536034483035874, 4730873063985049, 6894544683956743939, 5011047471537060743, 3541377440463742834, 7466833936155651911, 14094711162051263440, 12533067178555286422, 7062121012018115775, 933962161937548751, 10985786939639541088, 2673925637179329225, 12825824853535109291, 16861151458310501484, 4588133604979481946, 1984811539814218491, 852431271939654, 10396777398295830079, 5538876657603877557, 12921465296898898399, 1390224205672505744, 2219573541827841205,];

fn gen_graph(seed: usize) -> Vec<Vec<bool>> {
    let mut rnd = Random::new(seed);
    let n = rnd.next_in_range(1, 15);
    let mut g = vec![vec![false; n]; n];
    let buben = rnd.next_double();
    for i in 0..n {
        for j in i + 1..n {
            if rnd.next_double() < buben {
                g[i][j] = true;
                g[j][i] = true;
            }
        }
    }
    g
}


pub fn main3() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let mut rnd_global = Random::new(787788);
    let mut seen = vec![false; MAX_K + 1];
    let mut total_seen = 0;
    let mut seeds = vec![0; MAX_K + 1];
    for it in 0.. {
        let seed = rnd_global.next();
        let g = gen_graph(seed);
        let cnt = count(&g);
        if cnt > 0 && cnt <= MAX_K && !seen[cnt] {
            total_seen += 1;
            dbg!("seen", cnt, total_seen);
            seeds[cnt] = seed;
            seen[cnt] = true;
            if total_seen == MAX_K {
                break;
            }
        }
    }
    for i in 0..=MAX_K {
        write!(out, "{}, ", seeds[i]).unwrap();
    }
    writeln!(out).unwrap();
}


pub fn main() {
    let stdout = io::stdout();
    let mut out = std::io::BufWriter::new(stdout.lock());
    let mut sc = Scanner::new();

    let k = sc.usize();
    let g = gen_graph(precalc_seeds[k]);
    let n = g.len();
    let mut m = 0;
    for i in 0..n {
        for j in i + 1..n {
            if g[i][j] {
                m += 1;
            }
        }
    }
    writeln!(out, "{} {}", n, m).unwrap();
    for i in 0..n {
        for j in i + 1..n {
            if g[i][j] {
                writeln!(out, "{} {}", i + 1, j + 1).unwrap();
            }
        }
    }
}
