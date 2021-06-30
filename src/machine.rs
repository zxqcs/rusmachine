pub mod basic_machine {
    use crate::infrastructure::register::Register;
    use crate::infrastructure::stack::Stack;
    use crate::memory::memory::Memory;
    use crate::representation::type_system::Object;
    use std::collections::HashMap;

    pub struct BasicMachine {
        registers: HashMap<&'static str, Register>,
        stack: Stack,
        // instruction_sequence: Vec<Box<T>>,
    }

    impl BasicMachine {
        pub fn initilize_registers(&mut self) {
            self.registers.insert("pc", Register::new("pc"));
            self.registers.insert("flag", Register::new("flag"));
            // these registers below doesn't need to be stored in the process of GC
            self.registers.insert("root", Register::new("ROOT"));
            self.registers.insert("free", Register::new("FREE"));
            self.registers.insert("scan", Register::new("SCAN"));
            self.registers.insert("old", Register::new("OLD"));
            self.registers.insert("oldcr", Register::new("OLDCR"));
            self.registers.insert("new", Register::new("NEW"));
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

        pub fn get_register_contents(&self, name: &'static str) -> Option<Object> {
            let register = self.registers.get(name);
            match register {
                Some(x) => Some((*x.get()).clone()),
                None => None,
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

        // set a list in memory from a str and return a index to some register
        // for example, let s = "(1 (2 3))";
        // set s in memory and return the beginning index, such as, 3 to Register root
        pub fn set_register_as_in_memory(&mut self, name: &'static str, item: &'static str) {

        }

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
        pub fn assign_car(&mut self, x: &'static str, y: &'static str, memory: &mut Memory) {

        }

        // assign the cdr part of list of register y to register x
        pub fn assign_cdr(&mut self, x: &'static str, y: &'static str, memory: &mut Memory) {

        }

        // change the car part of list of register x to register y
        pub fn set_car(&mut self, x: &'static str, y: &'static str, memory: &mut Memory) {

        }

        // change the cdr part of list of register x to register y
        pub fn set_cdr(&mut self, x: &'static str, y: &'static str, memory: &mut Memory) {
            
        }

        // build a new list from register y and register z, then assign this new list to register x
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
