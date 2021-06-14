pub mod type_system {
    use std::usize;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Object {
        Nummber(f32),
        Integer(i32),
        Index(usize),
        Symbol(&'static str),
        Quote(&'static str),
        LispString(String),
        Bool(bool),
        Pair(Pair),
        Nil,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Pair {
        index: usize,
    }
}
