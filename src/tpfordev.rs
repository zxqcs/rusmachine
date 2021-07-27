pub mod type_system {
    use crate::representation::type_system::Object;

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

        pub fn is_symbol(&self) -> bool {
            match self {
                Exp::Symbol(_x) => true,
                _ => false,
            }
        }

        pub fn exp_to_object(&self) -> Object {
            match self {
                Exp::FloatNumber(x) => Object::Number(*x),
                Exp::Integer(x) => Object::Integer(*x),
                Exp::Symbol(x) => Object::Symbol((*x).to_string()),
                Exp::Quote(x) => Object::Symbol((*x).to_string()),
                Exp::Bool(x) => Object::Bool(*x),
                Exp::SchemeString(x) => Object::LispString((*x).to_string()),
                _ => panic!("Error: this type can't be converted to Object directly!"),
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
            scheme_cons(car(&lhs).unwrap(), append(cdr(&lhs).unwrap(), rhs))
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
    pub fn list_length(exp: &Exp) -> i32 {
        if exp == &Exp::List(Pair::Nil) {
            0
        } else {
            1 + list_length(&cdr(&exp).unwrap())
        }
    }

    #[allow(dead_code)]
    pub fn car(exp: &Exp) -> Result<Exp, &'static str> {
        if exp.is_pair() {
            if let Exp::List(Pair::Cons(x, _y)) = exp {
                Ok((**x).clone())
            } else {
                Err("error happens in running car!")
            }
        } else {
            Err("not a proper list!")
        }
    }

    #[allow(dead_code)]
    pub fn cdr(exp: &Exp) -> Result<Exp, &'static str> {
        if exp.is_pair() {
            if let Exp::List(Pair::Cons(_x, y)) = exp {
                let z = Exp::List((**y).clone());
                Ok(z)
            } else {
                Err("error happens in running cdr!")
            }
        } else {
            Err("not a proper lsit!")
        }
    }

    #[allow(dead_code)]
    pub fn scheme_map(proc: fn(Exp) -> Exp, items: Exp) -> Exp {
        if items.is_null() {
            Exp::List(Pair::Nil)
        } else {
            scheme_cons(
                proc(car(&items).unwrap()),
                scheme_map(proc, cdr(&items).unwrap()),
            )
        }
    }

    #[allow(dead_code)]
    pub fn scheme_map_clousre<F>(mut f: F, items: &Exp) -> Exp
    where
        F: FnMut(Exp) -> Exp,
    {
        if items.is_null() {
            Exp::List(Pair::Nil)
        } else {
            scheme_cons(
                f(car(items).unwrap()),
                scheme_map_clousre(f, &cdr(items).unwrap()),
            )
        }
    }

    #[allow(dead_code)]
    pub fn scheme_for_each<F>(mut f: F, items: &Exp)
    where
        F: FnMut(Exp),
    {
        if items.is_null() {
        } else {
            f(car(items).unwrap());
            scheme_for_each(f, &cdr(items).unwrap());
        }
    }

    #[allow(dead_code)]
    pub fn scheme_assoc(items: &Exp, key: &Exp) -> Option<Exp> {
        if items.is_pair() {
            let mut list = (*items).clone();
            while let Ok(x) = car(&list) {
                let item = car(&x).unwrap();
                match item {
                    y if y == *key => {
                        return Some(x);
                    }
                    _ => {}
                }
                list = cdr(&list).unwrap();
            }
            return None;
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod test {
    use super::type_system::{car, cdr, scheme_assoc, scheme_map, scheme_map_clousre};
    use crate::parserfordev::parser::str_to_exp;
    use crate::tpfordev::type_system::Exp;
    #[test]
    fn car_works() {
        let exp = str_to_exp("((1 2) (3 (4 5)))".to_string());
        let exp1 = car(&exp).unwrap();
        let exp2 = car(&exp1).unwrap();
        assert_eq!(exp1, str_to_exp("(1 2)".to_string()));
        assert_eq!(exp2, str_to_exp("1".to_string()));
    }

    #[test]
    fn cdr_works() {
        let exp = str_to_exp("((1 2) (3 (4 5)))".to_string());
        let exp1 = cdr(&exp).unwrap();
        let exp2 = cdr(&exp1).unwrap();
        let exp3 = car(&exp1).unwrap();
        let exp4 = cdr(&exp3).unwrap();

        assert_eq!(exp1, str_to_exp("((3 ( 4 5)))".to_string()));
        assert_eq!(exp2, str_to_exp("()".to_string()));
        assert_eq!(exp3, str_to_exp("(3 ( 4 5))".to_string()));
        assert_eq!(exp4, str_to_exp("(( 4 5))".to_string()));
    }

    #[test]
    fn scheme_map_works() {
        let items = str_to_exp("(1 2 3 4)".to_string());
        let result = scheme_map(square, items);
        assert_eq!(result, str_to_exp("(1 4 9 16)".to_string()));
    }

    #[test]
    fn scheme_map_clousre_works() {
        let square = |x: Exp| match x {
            Exp::Integer(i) => Exp::Integer(i * i),
            _ => {
                panic!("type mismatch!");
            }
        };
        let items = str_to_exp("(1 2 3 4)".to_string());
        let result = scheme_map_clousre(square, &items);
        assert_eq!(result, str_to_exp("(1 4 9 16)".to_string()));
    }

    fn square(x: Exp) -> Exp {
        match x {
            Exp::Integer(i) => Exp::Integer(i * i),
            _ => {
                panic!("type mismatch!");
            }
        }
    }

    #[test]
    fn scheme_assoc_works() {
        let items = str_to_exp("((spring 1) (summer 2) (autumn 3) (winter 4))".to_string());
        let mut key = Exp::Symbol("summer".to_string());
        let result_a = scheme_assoc(&items, &key).unwrap();
        let checkout_a = str_to_exp("(summer 2)".to_string());
        assert_eq!(result_a, checkout_a);
        key = Exp::Symbol("USA".to_string());
        let result_b = scheme_assoc(&items, &key);
        assert_eq!(result_b, None);
        let list = str_to_exp("(((bird)) ((egg)) ((cake)) ((pear)))".to_string());
        key = str_to_exp("(egg)".to_string());
        let result_c = scheme_assoc(&list, &key).unwrap();
        let checkout_c = str_to_exp("((egg))".to_string());
        assert_eq!(result_c, checkout_c);
    }
}
