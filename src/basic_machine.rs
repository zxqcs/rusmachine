mod basic_machine {
    use crate::infrastructure::register::Register;
    use crate::infrastructure::stack::Stack;
    use std::{collections::HashMap, hash::Hash};

    pub struct BasicMachine<T> {
        registers: HashMap<&'static str, Register>,
        stack: Stack,
        instruction_sequence: Vec<Box<T>>,
    }

    impl<T> BasicMachine<T> {
        fn initilize_stack(&mut self) {
            self.stack = Stack::new();
        }

        fn initilize_registers(&mut self) {
            self.registers = HashMap::new();
            self.registers.insert("pc", Register::new("pc"));
            self.registers.insert("flag", Register::new("flag"));
        }

        fn initialize_instruction_seq(&mut self) {}

        pub fn initilize_basic_machine(&mut self) {
            self.initilize_registers();
            self.initilize_stack();
            self.initialize_instruction_seq();
        }
    }
}
