pub mod basic_machine {
    use crate::infrastructure::register::Register;
    use crate::infrastructure::stack::Stack;
    use crate::memory::memory::Memory;
    use crate::parser::parser::{build_syntax_tree_into_memeory, tokenizer};
    use crate::representation::type_system::Object;
    use std::collections::HashMap;

    pub struct BasicMachine {
        registers: HashMap<&'static str, Register>,
        stack: Stack,
        // instruction_sequence: Vec<Box<T>>,
    }

    impl BasicMachine {
        // Register exp is used to hold the expression to be evaluated
        // Register env contains the environment
        // Register val contains the value obtained by evaluating the expression in
        // designated environment
        // Register continue is used to implement recursion
        // The registers proc, argl, and unev are used in evaluating combinations
        // The registers root, free, scan, old, oldcr, new are used in gc, which shall not
        // be stored in the gc process
        // The registers pc, flag is brought by basic machine
        pub fn initilize_registers(&mut self) {
            self.registers.insert("pc", Register::new("pc"));
            self.registers.insert("flag", Register::new("flag"));
            self.registers.insert("root", Register::new("ROOT"));
            self.registers.insert("free", Register::new("FREE"));
            self.registers.insert("scan", Register::new("SCAN"));
            self.registers.insert("old", Register::new("OLD"));
            self.registers.insert("oldcr", Register::new("OLDCR"));
            self.registers.insert("new", Register::new("NEW"));
            self.registers.insert("exp", Register::new("EXP"));
            self.registers.insert("env", Register::new("ENV"));
            self.registers.insert("unev", Register::new("UENV"));
            self.registers.insert("continue", Register::new("CONTINUE"));
            self.registers.insert("val", Register::new("VAL"));
            self.registers.insert("argl", Register::new("ARGL"));
            self.registers.insert("proc", Register::new("PROC"));
            self.registers
                .insert("relocate_continue", Register::new("RELOCATE_CONTINUE"));
        }

        // fn initialize_instruction_seq(&mut self) {}

        pub fn new() -> Self {
            let machine = BasicMachine {
                registers: HashMap::new(),
                stack: Stack::new(),
            };
            machine
        }

        pub fn get_register(&self, name: &'static str) -> Option<&Register> {
            self.registers.get(name)
        }

        pub fn get_register_contents_ref(&self, name: &'static str) -> Option<&Object> {
            let item = self.registers.get(name).clone();
            match item {
                Some(x) => Some(x.get()),
                None => None,
            }
        }

        #[allow(dead_code)]
        pub fn get_register_contents(&self, name: &'static str) -> Option<Object> {
            let register = self.registers.get(name);
            match register {
                Some(x) => Some((*x.get()).clone()),
                None => None,
            }
        }

        // in this case, a memory address is stored in machine's register
        // a list can be printed by calling this fn
        #[allow(dead_code)]
        pub fn print_register_contents(&self, name: &'static str, memory: &Memory) {
            let reg = self.get_register(name);
            match reg {
                Some(r) => {
                    r.print_list(memory);
                }
                None => {
                    panic!("No such register exists!");
                }
            }
        }

        // set a Oject directly in some Register
        pub fn set_register_contents(&mut self, name: &'static str, item: Object) {
            let register = self.registers.get_mut(name);

            match register {
                Some(x) => {
                    x.set(item);
                }
                None => {
                    panic!("No such register in this Machine!");
                }
            }
        }

        // in this case, a list object is wriiten into memory and the beginning address
        // is returned and stored in some register, for specific,
        // set a list in memory from a str and return a index to some register
        // for example, let s = "(1 (2 3))";
        // set s in memory and return the beginning index, such as, 3 to Register root
        pub fn set_register_contents_as_in_memory(
            &mut self,
            name: &'static str,
            object: &'static str,
            memory: &mut Memory,
        ) {
            let mut tokens = tokenizer(object);
            let index = build_syntax_tree_into_memeory(&mut tokens, memory, self);
            self.set_register_contents(name, Object::Index(index));
        }

        #[allow(dead_code)]
        pub fn assign_from_one_register_to_another(
            &mut self,
            to: &'static str,
            from: &'static str,
        ) {
            let from = self.get_register_contents(from);
            match from {
                Some(x) => {
                    self.set_register_contents(to, x);
                }
                None => {
                    panic!("No such registers in this Machie or nothing in this register now!");
                }
            }
        }

        #[allow(dead_code)]
        pub fn get_register_contents_as_in_memory(
            &self,
            name: &'static str,
            memory: &Memory,
        ) -> String {
            let reg = self.get_register(name);
            match reg {
                Some(r) => r.get_list_frome_memory_as_str(memory),
                None => {
                    panic!("No such registers exists!");
                }
            }
        }

        pub fn register_increment_by_one(&mut self, name: &'static str) {
            match name {
                "free" | "scan" | "pc" => {
                    let item = self.get_register_contents_ref(name).unwrap();
                    match item {
                        &Object::Index(i) => {
                            let item = Object::Index(i + 1);
                            self.set_register_contents(name, item);
                        }
                        _ => {
                            panic!(
                                "Not a proper index, panic when running register_increment_by_one!"
                            );
                        }
                    }
                }
                _ => {
                    panic!("Wrong type of Register got incremented!");
                }
            }
        }

        // assign the car part of list of register y to register x
        #[allow(dead_code)]
        pub fn assign_car(&mut self, x: &'static str, y: &'static str, memory: &mut Memory) {}

        // assign the cdr part of list of register y to register x
        #[allow(dead_code)]
        pub fn assign_cdr(&mut self, x: &'static str, y: &'static str, memory: &mut Memory) {}

        // change the car part of list of register x to register y
        #[allow(dead_code)]
        pub fn set_car(&mut self, x: &'static str, y: &'static str, memory: &mut Memory) {}

        // change the cdr part of list of register x to register y
        #[allow(dead_code)]
        pub fn set_cdr(&mut self, x: &'static str, y: &'static str, memory: &mut Memory) {}

        // build a new list from register y and register z, then assign this new list to register x
        #[allow(dead_code)]
        pub fn cons(
            &mut self,
            x: &'static str,
            y: &'static str,
            z: &'static str,
            memory: &mut Memory,
        ) {
        }
    }
}

#[cfg(test)]
mod test {
    use crate::memory::memory::Memory;

    use super::basic_machine::BasicMachine;

    #[test]
    fn set_register_contents_as_in_memory_works() {
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        let s = "(define x '(+ 1 2))";
        machine.set_register_contents_as_in_memory("root", s, &mut memory);
        let ss = machine.get_register_contents_as_in_memory("root", &memory);
        assert_eq!(ss, String::from("( define x '( + 1 2))"));
    }
}
