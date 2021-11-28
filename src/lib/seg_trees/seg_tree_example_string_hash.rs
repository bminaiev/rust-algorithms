use super::lazy_seg_tree3::*;

#[test]
fn string_hash() {
    mod modulo {
        const MODULO: i32 = 1_000_000_123;

        #[derive(Copy, Clone, Eq, PartialEq, Default)]
        pub struct Mod(i32);

        impl Mod {
            #[allow(unused)]
            pub const ZERO: Self = Self(0);

            #[allow(unused)]
            pub const ONE: Self = Self(1);

            fn rev_rec(a: i32, m: i32) -> i32 {
                if a == 1 {
                    return a;
                }
                return ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64)
                    as i32;
            }

            #[allow(dead_code)]
            fn inv(self) -> Mod {
                Mod(Self::rev_rec(self.0, MODULO))
            }

            #[allow(dead_code)]
            pub fn new(mut x: i32) -> Self {
                if x < 0 {
                    x += MODULO;
                } else if x >= MODULO {
                    x -= MODULO;
                }
                assert!(0 <= x && x < MODULO);
                Self(x)
            }
        }

        impl std::fmt::Display for Mod {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::fmt::Debug for Mod {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                const MAX: usize = 100;
                if self.0 <= MAX as i32 {
                    write!(f, "{}", self.0)
                } else if self.0 >= MODULO - MAX as i32 {
                    write!(f, "-{}", MODULO - self.0)
                } else {
                    for denum in 1..MAX {
                        for num in 1..MAX {
                            if Mod(num as i32) / Mod(denum as i32) == *self {
                                return write!(f, "{}/{}", num, denum);
                            }
                        }
                    }
                    write!(f, "(?? {} ??)", self.0)
                }
            }
        }

        impl std::ops::Add for Mod {
            type Output = Mod;

            fn add(self, rhs: Self) -> Self::Output {
                let res = self.0 + rhs.0;
                if res >= MODULO {
                    Mod(res - MODULO)
                } else {
                    Mod(res)
                }
            }
        }

        impl std::ops::AddAssign for Mod {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
                if self.0 >= MODULO {
                    self.0 -= MODULO;
                }
            }
        }

        impl std::ops::Sub for Mod {
            type Output = Mod;

            fn sub(self, rhs: Self) -> Self::Output {
                let res = self.0 - rhs.0;
                if res < 0 {
                    Mod(res + MODULO)
                } else {
                    Mod(res)
                }
            }
        }

        impl std::ops::SubAssign for Mod {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
                if self.0 < 0 {
                    self.0 += MODULO;
                }
            }
        }

        impl std::ops::Mul for Mod {
            type Output = Mod;

            fn mul(self, rhs: Self) -> Self::Output {
                let res = (self.0 as i64) * (rhs.0 as i64) % (MODULO as i64);
                Mod(res as i32)
            }
        }

        impl std::ops::MulAssign for Mod {
            fn mul_assign(&mut self, rhs: Self) {
                self.0 = ((self.0 as i64) * (rhs.0 as i64) % (MODULO as i64)) as i32;
            }
        }

        impl std::ops::Div for Mod {
            type Output = Mod;

            fn div(self, rhs: Self) -> Self::Output {
                let rhs_inv = rhs.inv();
                self * rhs_inv
            }
        }

        impl std::ops::DivAssign for Mod {
            fn div_assign(&mut self, rhs: Self) {
                *self *= rhs.inv();
            }
        }
    }

    use modulo::*;

    #[derive(Clone, Default, Copy, Debug)]
    struct HashNode {
        hash: Mod,
        len: usize,
    }

    struct Context {
        pow: Vec<Mod>,
    }

    impl LazySegTreeNodeSpec for HashNode {
        fn unite(l: &Self, r: &Self, context: &Context) -> Self {
            Self {
                hash: l.hash * context.pow[r.len] + r.hash,
                len: l.len + r.len,
            }
        }

        fn apply_update(node: &mut Self, update: &Self::Update) {
            node.hash = Mod::new(*update as i32);
        }

        #[allow(unused)]
        fn join_updates(current: &mut Self::Update, add: &Self::Update) {
            unreachable!()
        }

        type Update = u8;
        type Context = Context;
    }

    let s = "abacaba".to_owned().into_bytes();
    let mut powers = vec![Mod::ONE; s.len() + 1];
    for i in 1..powers.len() {
        powers[i] = powers[i - 1] * Mod::new(239);
    }
    let context = Context { pow: powers };
    let mut seg_tree = LazySegTree::new_f(
        s.len(),
        &|pos| HashNode {
            hash: Mod::new(s[pos] as i32),
            len: 1,
        },
        context,
    );
    let aba = seg_tree.get(0, 3);
    let aba2 = seg_tree.get(4, 7);
    println!("res = {:?}, res2 = {:?}", aba, aba2);
}
