pub mod register {
    use crate::representation::type_system::Object;
    use std::fmt;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Item {
        index(usize), // to indicate the position in memory vectors
        object(Object),
    }
    
    pub struct Register {
        pub name: &'static str,
        pub contents: Item,
    }

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self.contents {
                Item::index(x) => {
                    write!(f, "name: {}, memory index: {}", self.name, x)
                }
                Item::object(x) => {
                    write!(f, "name: {}, objects: {:?}", self.name, x)
                }
            }
        }
    }

    impl Register {
        pub fn new(s: &'static str) -> Self {
            Register {
                name: s,
                contents: Item::object(Object::Quote("unsigned")),
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

pub mod stack {
    use super::register::Item;
    use std::fmt;
    pub struct Stack {
        capacity: usize,
        container: Vec<Item>,
    }

    impl Stack {
        pub fn new() -> Self {
            Stack {
                capacity: 1000,
                container: Vec::new(),
            }
        }

        pub fn push(&mut self, item: Item) {
            if self.container.len() < self.capacity {
                self.container.push(item);
            } else {
                panic!("Maximum depth violated!");
            }
        }

        pub fn pop(&mut self) -> Option<Item> {
            self.container.pop()
        }
    }

    impl fmt::Display for Stack {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Depth: {}, top: {:?}",
                self.container.len(),
                self.container[self.container.len() - 1]
            )
        }
    }
}

#[cfg(test)]
mod test {
    use crate::representation::type_system::Object;

    use super::{
        register::{Item, Register},
        stack::Stack,
    };

    #[test]
    fn register_get_works() {
        let r = Register::new("Alpha");
        assert_eq!(Item::object(Object::Quote("unsigned")), *r.get());
    }

    #[test]
    fn register_set_works() {
        let mut r = Register::new("Alpha");
        r.set(Item::object(Object::Quote("apple")));
        assert_eq!(Item::object(Object::Quote("apple")), *r.get());
    }

    #[test]
    fn stack_push_pop() {
        let mut s = Stack::new();
        s.push(Item::object(Object::Quote("Winter")));
        let item = s.pop().unwrap();
        assert_eq!(item, Item::object(Object::Quote("Winter")));
    }
}
