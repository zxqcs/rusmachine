pub mod type_system {
    #[derive(Debug, Clone, PartialEq)]
    pub enum Object {
        Nummber(f32),
        Integer(i32),
        Symbol(&'static str),
        Quote(&'static str),
        Bool(bool),
        Pair(Pair),
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Pair {
        index(usize),
        Nil,
    }
}

