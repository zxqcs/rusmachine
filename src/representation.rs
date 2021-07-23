pub mod type_system {
    use std::usize;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Object {
        Nummber(f32), // both
        Integer(i32),  // both
        Symbol(String), // both
        Quote(String), // both
        LispString(String), // both
        Index(usize), // it can live in both memory and register
        Bool(bool), // both
        Pair(usize),  // live in memory only 
        Nil,   // live in memory only
        Empty,
    }
}
