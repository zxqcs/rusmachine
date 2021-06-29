pub mod register {
    use crate::representation::type_system::{Pair, Object};
    use crate::memory::memory::Memory;
    use std::{fmt, mem, usize};

    #[derive(Debug, Clone, PartialEq)]
    pub enum Item {
        Object(Object),
        Index(usize), // a pointe to the List Memory
    }

    pub struct Register {
        pub name: &'static str,
        pub contents: Item,
    }

    impl fmt::Display for Register {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "name: {}, Objects: {:?}", self.name, self.contents)
        }
    }

    impl Register {
        pub fn new(s: &'static str) -> Self {
            Register {
                name: s,
                contents: Item::Object(Object::Quote("unsigned")),
            }
        }
        pub fn get(&self) -> &Item {
            &self.contents
        }

        pub fn set(&mut self, val: Item) {
            self.contents = val;
        }

        pub fn get_inner_object(&self) -> Object {
            if let Item::Object(x) = &self.contents {
                (*x).clone()
            } else {
                panic!("not a porper Object in this register!");
            }
        }

        pub fn get_memory_index(&self) -> usize {
            if let Item::Index(x) = &self.contents {
                *x
            } else {
                panic!("not a proper Index for Memory in this register!");
            }
        }

        pub fn print_list(&self, memory: &Memory) {
            print!("(");
            let i = self.get_memory_index();
            let car_item = memory.car(i);
            let cdr_item = memory.cdr(i);

            print_list_iter(&car_item, &cdr_item, memory);
            print!("\n");
        }

        pub fn car(&self, memory: &Memory) -> Object {
            let  i = self.get_memory_index(); 
                memory.car(i)
        } 
       
        pub fn cdr(&self, memory: &Memory) -> Object {
            let  i = self.get_memory_index(); 
                memory.cdr(i)
        } 
    }

    fn print_list_iter(car_item: &Object, cdr_item: &Object, memory: &Memory) {
        match car_item {
            Object::Bool(x) => print!("{} ", *x), 
            Object::Integer(x) => print!("{} ", *x), 
            Object::LispString(x) => print!("{} ",*x),
            Object::Nummber(x) => print!("{} ", *x),
            Object::Quote(x) => print!("{} ", *x),
            Object::Symbol(x) => print!("{} ", *x),
            Object::Pair(x) => {
                print!("(");
                let car_item = &memory.car(x.index());
                let cdr_item = &memory.cdr(x.index());
                print_list_iter(car_item, cdr_item, memory);
            },
            _ => {
                panic!("not a proper object in car position of A Lisp List!");
            }, 
        }

        match cdr_item {
            Object::Nil => {
                print!(") ");
            }
            Object::Pair(x) => {
                let car_item = &memory.car(x.index());
                let cdr_item = &memory.cdr(x.index());
                print_list_iter(car_item, cdr_item, memory);
            },
            _ => {
                panic!("not a proper object in cdr position of A Lisp List!");
            },
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
    use crate::memory::memory::Memory;
    use crate::representation::type_system::Object;

    use super::{
        register::{Item, Register},
        stack::Stack,
    };

    #[test]
    fn register_get_works() {
        let r = Register::new("Alpha");
        assert_eq!(Item::Object(Object::Quote("unsigned")), *r.get());
    }

    #[test]
    fn register_set_works() {
        let mut r = Register::new("Alpha");
        r.set(Item::Object(Object::Quote("apple")));
        assert_eq!(Item::Object(Object::Quote("apple")), *r.get());
    }

    #[test]
    fn stack_push_pop() {
        let mut s = Stack::new();
        s.push(Item::Object(Object::Quote("Winter")));
        let item = s.pop().unwrap();
        assert_eq!(item, Item::Object(Object::Quote("Winter")));
    }
}
