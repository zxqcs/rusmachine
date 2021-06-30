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
        Pair(Pair),
        Nil,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Pair {
        index: usize,
    }

    impl Pair {
        pub fn index(&self) -> usize {
            self.index
        }

        pub fn new(i: usize) -> Self {
            Pair { index: i }
        }
    }
}
