pub mod register {
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
        pub contents: Item,
    }

    impl Register {
        pub fn new() -> Self {
            Register { contents: Item::RegisterStr("unsigned") }
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
        let r = Register {
            contents: Item::RegisterStr("unsigned")
        };
        assert_eq!(Item::RegisterStr("unsigned"), *r.get());
    }

    #[test]
    fn register_set_works() {
        let mut r = Register {
            contents: Item::RegisterStr("unsigned"),
        };
        r.set(Item::RegisterStr("apple"));
        assert_eq!(Item::RegisterStr("apple"), *r.get());
    }
}