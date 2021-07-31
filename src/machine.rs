pub mod basic_machine {
    use crate::infrastructure::register::Register;
    use crate::infrastructure::stack::Stack;
    use crate::memory::memory::Memory;
    use crate::parser::parser::{build_syntax_tree_into_memeory, tokenizer};
    use crate::representation::type_system::Object;
    use crate::tpfordev::type_system::Exp;
    use std::collections::HashMap;

    pub struct BasicMachine {
        pub registers: HashMap<String, Register>,
        pub stack: Stack,
        pub ops: HashMap<String, CallbackExp>,
        pub instruction_sequence: Vec<Box<dyn FnOnce(&mut BasicMachine, &mut Memory)>>,
    }

    type CallbackExp = fn(&Exp) -> Exp;

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
            self.registers.insert("pc".to_string(), Register::new("PC"));
            self.registers
                .insert("flag".to_string(), Register::new("FLAG"));
            self.registers
                .insert("root".to_string(), Register::new("ROOT"));
            self.registers
                .insert("free".to_string(), Register::new("FREE"));
            self.registers
                .insert("scan".to_string(), Register::new("SCAN"));
            self.registers
                .insert("old".to_string(), Register::new("OLD"));
            self.registers
                .insert("oldcr".to_string(), Register::new("OLDCR"));
            self.registers
                .insert("new".to_string(), Register::new("NEW"));
            self.registers
                .insert("exp".to_string(), Register::new("EXP"));
            self.registers
                .insert("env".to_string(), Register::new("ENV"));
            self.registers
                .insert("unev".to_string(), Register::new("UENV"));
            self.registers
                .insert("continue".to_string(), Register::new("CONTINUE"));
            self.registers
                .insert("val".to_string(), Register::new("VAL"));
            self.registers
                .insert("argl".to_string(), Register::new("ARGL"));
            self.registers
                .insert("proc".to_string(), Register::new("PROC"));
            self.registers.insert(
                "relocate_continue".to_string(),
                Register::new("RELOCATE_CONTINUE"),
            );
            self.set_register_contents(&"free".to_string(), Object::Index(0));
        }

        pub fn add_op(&mut self, fn_name: String, func: CallbackExp) {
            self.ops.insert(fn_name, func);
        }

        pub fn call_op(&mut self, fn_name: String, argv: &Exp) -> Exp {
            self.ops[&fn_name](argv)
        }

        pub fn new() -> Self {
            let machine = BasicMachine {
                registers: HashMap::new(),
                stack: Stack::new(),
                ops: HashMap::new(),
                instruction_sequence: Vec::new(),
            };
            machine
        }

        pub fn get_register(&self, name: &String) -> Option<&Register> {
            self.registers.get(name)
        }

        pub fn get_register_contents_ref(&self, name: String) -> Option<&Object> {
            let item = self.registers.get(&name).clone();
            match item {
                Some(x) => Some(x.get()),
                None => None,
            }
        }

        #[allow(dead_code)]
        pub fn get_register_contents(&self, name: &String) -> Option<Object> {
            let register = self.registers.get(name);
            match register {
                Some(x) => Some((*x.get()).clone()),
                None => None,
            }
        }

        // in this case, a memory address is stored in machine's register
        // a list can be printed by calling this fn
        #[allow(dead_code)]
        pub fn print_register_contents(&self, name: &String, memory: &Memory) {
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
        pub fn set_register_contents(&mut self, name: &String, item: Object) {
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
        #[allow(dead_code)]
        pub fn set_register_contents_as_in_memory(
            &mut self,
            name: &String,
            object: String,
            memory: &mut Memory,
        ) {
            let item = memory.write(object, self);
            self.set_register_contents(name, Object::Index(item));
        }

        #[allow(dead_code)]
        pub fn assign_from_one_register_to_another(&mut self, to: &String, from: &String) {
            let from = self.get_register_contents(&from);
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
        pub fn get_register_contents_as_in_memory(&self, name: &String, memory: &Memory) -> String {
            let reg = self.get_register(name);
            match reg {
                Some(r) => r.get_list_frome_memory_as_str(memory),
                None => {
                    panic!("No such registers exists!");
                }
            }
        }

        #[allow(dead_code)]
        pub fn advance_pc(&mut self) {
            /*
            let reg = "pc".to_string();
            self.register_increment_by_one(&reg);
            */
        }

        #[allow(dead_code)]
        pub fn advance_free(&mut self) {
            let reg = "free".to_string();
            self.register_increment_by_one(&reg);
        }

        #[allow(dead_code)]
        pub fn advance_scan(&mut self) {
            let reg = "scan".to_string();
            self.register_increment_by_one(&reg);
        }

        pub fn register_increment_by_one(&mut self, name: &String) {
            let item = self.get_register_contents_ref(name.clone()).unwrap();
            match item {
                &Object::Index(i) => {
                    let item = Object::Index(i + 1);
                    self.set_register_contents(name, item);
                }
                _ => {
                    panic!("Not a proper index, panic when running register_increment_by_one!")
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
    use crate::tpfordev::type_system::{append, scheme_cons};
    use crate::{
        memory::memory::Memory,
        parserfordev::parser::str_to_exp,
        primitives::primitives::{
            assignment_variable, caddr, cadr, is_self_evaluating, is_variable, make_procedure,
        },
        scheme_list,
        tpfordev::type_system::{Exp, Pair},
    };

    use super::basic_machine::BasicMachine;

    #[test]
    fn set_register_contents_as_in_memory_works() {
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        let s = "(define x '(+ 1 2))";
        machine.set_register_contents_as_in_memory(&"root".to_string(), s.to_string(), &mut memory);
        let ss = machine.get_register_contents_as_in_memory(&"root".to_string(), &memory);
        assert_eq!(ss, String::from("( define x '( + 1 2))"));
    }

    #[test]
    fn machine_ops_bool_works() {
        let mut machine = BasicMachine::new();
        machine.add_op("self_evaluating".to_string(), is_self_evaluating);
        let exp1 = scheme_list!(Exp::SchemeString("winter is coming".to_string()));
        let exp2 = scheme_list!(Exp::Symbol("x".to_string()));
        assert_eq!(
            machine.call_op("self_evaluating".to_string(), &exp1.clone()),
            Exp::Bool(true)
        );
        assert_eq!(
            machine.call_op("self_evaluating".to_string(), &exp2.clone()),
            Exp::Bool(false)
        );
        machine.add_op("is_variable".to_string(), is_variable);
        assert_eq!(
            machine.call_op("is_variable".to_string(), &exp2),
            Exp::Bool(true)
        );
        assert_eq!(
            machine.call_op("is_variable".to_string(), &exp1),
            Exp::Bool(false)
        );
    }

    #[test]
    fn machine_ops_exp_works() {
        let mut machine = BasicMachine::new();
        machine.add_op("assignment_variable".to_string(), assignment_variable);
        let assisgn_exp = "(assign a (reg b ))".to_string();
        let assign = str_to_exp(assisgn_exp);
        let var = machine.call_op("assignment_variable".to_string(), &assign);
        assert_eq!(var, Exp::Symbol("a".to_string()));
        machine.add_op("make_procedure".to_string(), make_procedure);
        let lambda_exp = "(lambda (x) (* x x))".to_string();
        let lambda = str_to_exp(lambda_exp);
        let parameters = cadr(&lambda).unwrap();
        let body = caddr(&lambda).unwrap();
        let env = Exp::List(Pair::Nil);
        let args = scheme_list!(parameters.clone(), body.clone(), env.clone());
        let proc = machine.call_op("make_procedure".to_string(), &args);
        let tag = Exp::Symbol("procedure".to_string());
        assert_eq!(proc, scheme_list!(tag, parameters, body, env));
    }
}
