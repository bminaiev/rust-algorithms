mod modulo {
    const MODULO: i32 = 1_000_000_007;

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mod(i32);

    impl Mod {
        fn rev_rec(a: i32, m: i32) -> i32 {
            if a == 1 {
                return a;
            }
            return ((1 - Self::rev_rec(m % a, a) as i64 * m as i64) / a as i64 + m as i64) as i32;
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

    impl std::ops::Mul for Mod {
        type Output = Mod;

        fn mul(self, rhs: Self) -> Self::Output {
            let res = (self.0 as i64) * (rhs.0 as i64) % (MODULO as i64);
            Mod(res as i32)
        }
    }

    impl std::ops::Div for Mod {
        type Output = Mod;

        fn div(self, rhs: Self) -> Self::Output {
            let rhs_inv = rhs.inv();
            self * rhs_inv
        }
    }
}

#[cfg(test)]
mod tests {
    use super::modulo::*;

    #[test]
    fn add() {
        let x = Mod::new(1);
        let y = Mod::new(2);
        assert_eq!(format!("{}", x + y), "3");
    }

    #[test]
    fn sub() {
        let x = Mod::new(1);
        let y = Mod::new(2);
        assert_eq!(format!("{}", x - y), "1000000006");
        assert_eq!(format!("{:?}", x - y), "-1");
    }

    #[test]
    fn mul() {
        let x = Mod::new(3);
        let y = Mod::new(5);
        assert_eq!(format!("{}", x * y), "15");
    }

    #[test]
    fn div() {
        let x = Mod::new(3);
        let y = Mod::new(5);
        assert_eq!(format!("{}", x / y), "200000002");
        assert_eq!(format!("{:?}", x / y), "3/5");
    }

    #[test]
    fn dbg_format() {
        let x = Mod::new(1) / Mod::new(2);
        let y = Mod::new(1) / Mod::new(3);
        assert_eq!(format!("{}", x + y), "833333340");
        assert_eq!(format!("{:?}", x + y), "5/6");
    }

    #[test]
    fn dbg_format_big() {
        let x = Mod::new(123) / Mod::new(457);
        assert_eq!(format!("{:?}", x), "(?? 262582059 ??)");
    }

    #[test]
    fn dbg_format_more() {
        assert_eq!(format!("{:?}", Mod::new(1)), "1");
        assert_eq!(format!("{:?}", Mod::new(3)), "3");
        assert_eq!(format!("{:?}", Mod::new(-5)), "-5");
    }
}