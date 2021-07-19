pub mod type_system {
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub enum Pair {
        Cons(Box<Exp>, Box<Pair>),
        Nil,
    }

    impl PartialEq for Pair {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Pair::Nil => match other {
                    Pair::Nil => true,
                    _ => false,
                },
                Pair::Cons(x, y) => match other {
                    Pair::Nil => false,
                    Pair::Cons(x1, y1) => x == x1 && y == y1,
                },
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Exp {
        FloatNumber(f32),
        Integer(i32),
        List(Pair),
        Symbol(String),
        Quote(String),
        SchemeString(String),
        Bool(bool),
    }

    impl Exp {
        pub fn is_pair(&self) -> bool {
            match self {
                Exp::List(x) => match x {
                    Pair::Nil => false,
                    _ => true,
                },
                _ => false,
            }
        }

        pub fn is_null(&self) -> bool {
            match self {
                Exp::List(Pair::Nil) => true,
                _ => false,
            }
        }
    }

    #[macro_export]
    macro_rules! scheme_list {
    ( $( $x:expr ),* ) => {
        {
            let null = Exp::List(Pair::Nil);
            let mut temp_list = null.clone();
            $(
                temp_list = append(temp_list, scheme_cons($x, null.clone()));
            )*
            temp_list
        }
    }
}

    #[allow(dead_code)]
    pub fn scheme_cons(lhs: Exp, rhs: Exp) -> Exp {
        match rhs {
            Exp::List(x) => {
                let s1 = Box::new(lhs);
                let s2 = Box::new(x);
                let s3 = Pair::Cons(s1, s2);
                Exp::List(s3)
            }
            _ => {
                let s1 = Box::new(Pair::Nil);
                let s2 = Box::new(rhs);
                let s3 = Pair::Cons(s2, s1);
                let s4 = Box::new(s3);
                let s5 = Box::new(lhs);
                Exp::List(Pair::Cons(s5, s4))
            }
        }
    }

    #[allow(dead_code)]
    pub fn append(lhs: Exp, rhs: Exp) -> Exp {
        let null = Exp::List(Pair::Nil);
        if lhs == null {
            rhs
        } else {
            scheme_cons(
                car(&lhs).unwrap(),
                append(cdr(&lhs).unwrap(), rhs),
            )
        }
    }

    #[allow(dead_code)]
    pub fn set_car(x: Exp, y: Exp) -> Result<Exp, &'static str> {
        if let Exp::List(Pair::Cons(lhs, rhs)) = x {
            Ok(Exp::List(Pair::Cons(Box::new(y), rhs)))
        } else {
            Err("error happens!")
        }
    }

    #[allow(dead_code)]
    pub fn set_cdr(x: Exp, y: Exp) -> Result<Exp, &'static str> {
        if let Exp::List(Pair::Cons(lhs, rhs)) = x {
            Ok(scheme_cons(*lhs, y))
        } else {
            Err("error happens!")
        }
    }
    #[allow(dead_code)]
    pub fn list_length(exp: Exp) -> i32 {
        if exp == Exp::List(Pair::Nil) {
            0
        } else {
            1 + list_length(cdr(&exp).unwrap())
        }
    }

    #[allow(dead_code)]
    pub fn car(exp: &Exp) -> Result<Exp, &'static str> {
        match exp {
            Exp::List(_x) => {
                if exp.is_pair() {
                    if let Exp::List(Pair::Cons(x, _y)) = exp {
                        Ok((**x).clone())
                    } else {
                        Err("error happens!")
                    }
                } else {
                    Err("not a pair!")
                }
            }
            _ => Err("type mismatch, not even a List!"),
        }
    }

    #[allow(dead_code)]
    pub fn cdr(exp: &Exp) -> Result<Exp, &'static str> {
        match exp {
            Exp::List(_x) => {
                if exp.is_pair() {
                    if let Exp::List(Pair::Cons(_x, y)) = exp {
                        let z = Exp::List((**y).clone());
                        Ok(z)
                    } else {
                        Err("error happens!")
                    }
                } else {
                    Err("not a pair!")
                }
            }
            _ => Err("type mismatch, not even a List!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::type_system::{car, cdr};
    use crate::parserfordev::parser::str_to_exp;
    #[test]
    fn car_works() {
        let exp = str_to_exp("((1 2) (3 (4 5)))");
        let exp1 = car(&exp).unwrap();
        let exp2 = car(&exp1).unwrap();
        assert_eq!(exp1, str_to_exp("(1 2)"));
        assert_eq!(exp2, str_to_exp("1"));
    }

    #[test]
    fn cdr_works() {
        let exp = str_to_exp("((1 2) (3 (4 5)))");
        let exp1 = cdr(&exp).unwrap();
        let exp2 = cdr(&exp1).unwrap();
        let exp3 = car(&exp1).unwrap();
        let exp4 = cdr(&exp3).unwrap();

        assert_eq!(exp1, str_to_exp("((3 ( 4 5)))"));
        assert_eq!(exp2, str_to_exp("()"));
        assert_eq!(exp3, str_to_exp("(3 ( 4 5))"));
        assert_eq!(exp4, str_to_exp("(( 4 5))"));
    }
}
