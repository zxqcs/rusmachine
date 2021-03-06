pub mod type_system {
    use std::usize;

    use crate::tpfordev::type_system::Exp;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Object {
        Number(f32),        // both
        Integer(i32),       // both
        Symbol(String),     // both
        Quote(String),      // both
        LispString(String), // both
        Index(usize),       // it can live in both memory and register
        Bool(bool),         // both
        Pair(usize),        // live in memory only
        Nil,                // live in memory only
        Empty,              // live in memory only
    }

    impl Object {
        pub fn object_to_exp(&self) -> Exp {
            match self {
                Object::Number(x) => Exp::FloatNumber(*x),
                Object::Integer(x) => Exp::Integer(*x),
                Object::Symbol(x) => Exp::Symbol((*x).clone()),
                Object::Quote(x) => Exp::Quote((*x).clone()),
                Object::LispString(x) => Exp::SchemeString((*x).clone()),
                Object::Bool(x) => Exp::Bool(*x),
                _ => {
                    panic!("Error, the type can't be converted to Exp!");
                }
            }
        }
    }
}
