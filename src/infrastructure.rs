pub mod register {
    use crate::memory::memory::Memory;
    use crate::representation::type_system::Object;
    use std::{fmt, usize};
    pub struct Register {
        pub name: &'static str,
        pub contents: Object,
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
                contents: Object::Quote("unsigned".to_string()),
            }
        }
        pub fn get(&self) -> &Object {
            &self.contents
        }

        pub fn set(&mut self, val: Object) {
            self.contents = val;
        }

        pub fn get_memory_index(&self) -> usize {
            if let &Object::Index(x) = &self.contents {
                x
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

        pub fn get_list_frome_memory_as_str(&self, memory: &Memory) -> String {
            let mut s = "".to_string();
            s.push('(');
            let i = self.get_memory_index();
            let car_item = memory.car(i);
            let cdr_item = memory.cdr(i);

            get_list_from_memory_as_str_iter(&mut s, &car_item, &cdr_item, memory);
            s
        }

        // thest two API may possibly be cancelled later, since the meaning of
        // car and cdr is vague.
        pub fn car(&self, memory: &Memory) -> Object {
            let i = self.get_memory_index();
            memory.car(i)
        }

        pub fn cdr(&self, memory: &Memory) -> Object {
            let i = self.get_memory_index();
            memory.cdr(i)
        }
    }

    fn get_list_from_memory_as_str_iter(
        s: &mut String,
        car_item: &Object,
        cdr_item: &Object,
        memory: &Memory,
    ) {
        match car_item {
            Object::Bool(x) => {
                s.push(' ');
                s.push_str(&x.to_string());
            }
            Object::Integer(x) => {
                s.push(' ');
                s.push_str(&x.to_string());
            }
            Object::LispString(x) => {
                s.push(' ');
                s.push('"');
                s.push_str(&x);
                s.push('"');
            }
            Object::Nummber(x) => {
                s.push(' ');
                s.push_str(&x.to_string());
            }
            Object::Quote(x) => {
                s.push(' ');
                s.push('\'');
                s.push_str(&x);
            }
            Object::Symbol(x) => {
                s.push(' ');
                s.push_str(&x);
            }
            Object::Pair(x) => {
                s.push('(');
                let car_item = &memory.car(*x);
                let cdr_item = &memory.cdr(*x);
                get_list_from_memory_as_str_iter(s, car_item, cdr_item, memory);
            }
            _ => {
                panic!("not a proper object in car position of A Lisp List!");
            }
        }

        match cdr_item {
            Object::Nil => {
                s.push(')');
            }
            Object::Pair(x) => {
                let car_item = &memory.car(*x);
                let cdr_item = &memory.cdr(*x);
                get_list_from_memory_as_str_iter(s, car_item, cdr_item, memory);
            }
            _ => {
                panic!("not a proper object in cdr position of A Lisp List!");
            }
        }
    }

    fn print_list_iter(car_item: &Object, cdr_item: &Object, memory: &Memory) {
        match car_item {
            Object::Bool(x) => print!("{} ", *x),
            Object::Integer(x) => print!("{} ", *x),
            Object::LispString(x) => print!("{} ", *x),
            Object::Nummber(x) => print!("{} ", *x),
            Object::Quote(x) => print!("{} ", *x),
            Object::Symbol(x) => print!("{} ", *x),
            Object::Pair(x) => {
                print!("(");
                let car_item = &memory.car(*x);
                let cdr_item = &memory.cdr(*x);
                print_list_iter(car_item, cdr_item, memory);
            }
            _ => {
                panic!("not a proper object in car position of A Lisp List!");
            }
        }

        match cdr_item {
            Object::Nil => {
                print!(")");
            }
            Object::Pair(x) => {
                let car_item = &memory.car(*x);
                let cdr_item = &memory.cdr(*x);
                print_list_iter(car_item, cdr_item, memory);
            }
            _ => {
                panic!("not a proper object in cdr position of A Lisp List!");
            }
        }
    }
}

pub mod stack {
    use crate::representation::type_system::Object;
    use std::fmt;
    pub struct Stack {
        capacity: usize,
        container: Vec<Object>,
    }

    impl Stack {
        pub fn new() -> Self {
            Stack {
                capacity: 1000,
                container: Vec::new(),
            }
        }

        pub fn push(&mut self, item: Object) {
            if self.container.len() < self.capacity {
                self.container.push(item);
            } else {
                panic!("Maximum depth violated!");
            }
        }

        pub fn pop(&mut self) -> Option<Object> {
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
    use crate::machine::basic_machine::BasicMachine;
    use crate::memory::memory::Memory;
    use crate::parser::parser::build_syntax_tree_into_memeory;
    use crate::parser::parser::tokenizer;
    use crate::representation::type_system::Object;

    use super::{register::Register, stack::Stack};

    #[test]
    fn register_get_works() {
        let r = Register::new("Alpha");
        assert_eq!(Object::Quote("unsigned".to_string()), *r.get());
    }

    #[test]
    fn register_set_works() {
        let mut r = Register::new("Alpha");
        r.set(Object::Quote("apple".to_string()));
        assert_eq!(Object::Quote("apple".to_string()), *r.get());
    }

    #[test]
    fn stack_push_pop() {
        let mut s = Stack::new();
        s.push(Object::Quote("Winter".to_string()));
        let item = s.pop().unwrap();
        assert_eq!(item, Object::Quote("Winter".to_string()));
    }

    #[test]
    fn get_list_from_memory_as_str_works() {
        let mut memory = Memory::new(20);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        let s = "(( 1  2 )
                           (3 
                               (4  
                                  5)))";
        let mut tokens = tokenizer(s.to_string());
        let root = build_syntax_tree_into_memeory(&mut tokens, &mut memory, &mut machine);
        machine.set_register_contents("root".to_string(), Object::Index(root));
        let reg = machine.get_register("root".to_string()).unwrap();
        let s = String::from("(( 1 2)( 3( 4 5)))");
        assert_eq!(s, reg.get_list_frome_memory_as_str(&memory));
        let ss = "(( 7 8) 9)";
        let mut ttokens = tokenizer(ss.to_string());
        let another_root = build_syntax_tree_into_memeory(&mut ttokens, &mut memory, &mut machine);
        machine.set_register_contents("root".to_string(), Object::Index(root));
        let s = String::from("(( 7 8) 9)");
        assert_eq!(
            ss,
            machine
                .get_register("root".to_string())
                .unwrap()
                .get_list_frome_memory_as_str(&memory)
        );
    }
}
