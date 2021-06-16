pub mod basic_machine {
    use crate::infrastructure::register::{Item, Register};
    use crate::infrastructure::stack::Stack;
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

        pub fn get_register_contents(&self, name: &'static str) -> Option<&Item> {
            let item = self.registers.get(name).clone();
            match item {
                Some(x) => Some(x.get()),
                None => None,
            }
        }

        pub fn get_register_inner_object(&self, name: &'static str) -> Option<Object> {
            let register = self.registers.get(name);
            match register {
                Some(x) => Some(x.get_inner_object()),
                None => None,
            }
        }

        pub fn set_register_contents(&mut self, name: &'static str, item: Item) {
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

        pub fn assign_from_one_register_to_another(
            &mut self,
            to: &'static str,
            from: &'static str,
        ) {
            let from = self.get_register_contents(from);
            match from {
                Some(x) => {
                    let item = (*x).clone();
                    self.set_register_contents(to, item);
                }
                None => {
                    panic!("No such registers in this Machie or nothing in this register now!");
                }
            }
        }

        pub fn register_increment_by_one(&mut self, name: &'static str) {
            match name {
                "free" | "scan" | "pc" => {
                    let item = self.get_register_contents(name).unwrap();
                    match item {
                        &Item::Object(Object::Index(i)) => {
                            let item = Item::Object(Object::Index(i + 1));
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
    }
}
