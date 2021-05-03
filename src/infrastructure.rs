
pub mod register {
    use std::fmt;
    #[derive(Debug)]
    pub enum Item {
        RegisterNumber(f64),
        RegisterStr(&'static str),
    }
    
    impl PartialEq for Item {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Item::RegisterNumber(x) => match other {
                    Item::RegisterNumber(y) => {
                        x == y 
                    },
                    _ => false,
                },
                Item::RegisterStr(x) => match other {
                    Item::RegisterStr(y) => {
                        x == y
                    },
                    _ => false,
                },
            }
        }
    }

    #[derive(Debug)]
    pub struct Register {
        pub name: &'static str,
        pub contents: Item,
    }

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.contents {
                Item::RegisterNumber(x) => {
                    write!(f, "name: {}, contents: {}", self.name, x)
                },
                Item::RegisterStr(x) => {
                    write!(f, "name: {}, contents: {}", self.name, x)
                },
            }
        }
    }
    
    impl Register {
        pub fn new(s: &'static str) -> Self {
            Register { 
                name: s,
                contents: Item::RegisterStr("unsigned"),
            }
        }
        pub fn get(&self) -> &Item {
            &self.contents
        }

        pub fn set(&mut self, val: Item) {
            self.contents = val;
        }
    }
}

#[cfg(test)]
mod test {
    use super::register::{Item, Register};

    #[test]
    fn register_get_works() {
        let r = Register::new("Alpha"); 
        assert_eq!(Item::RegisterStr("unsigned"), *r.get());
    }

    #[test]
    fn register_set_works() {
        let mut r = Register::new("Alpha"); 
        r.set(Item::RegisterStr("apple"));
        assert_eq!(Item::RegisterStr("apple"), *r.get());
    }
}