pub mod type_system {
    use std::usize;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Object {
        Nummber(f32),
        Integer(i32),
        Symbol(String),
        Quote(String),
        LispString(String),
        Index(usize),
        Bool(bool),
        Pair(usize),
        Nil,
        Empty,
    }
}
