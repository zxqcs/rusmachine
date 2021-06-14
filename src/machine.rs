pub mod basic_machine {
    use crate::infrastructure::register::{Item, Register};
    use crate::infrastructure::stack::Stack;
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
    }
}
