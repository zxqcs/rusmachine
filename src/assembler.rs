pub mod assembler {
    use crate::machine::basic_machine::BasicMachine;
    use crate::parserfordev::parser::{exp_to_str, str_to_exp};
    use crate::primitives::primitives::{cadr, is_tagged_list};
    use crate::representation::type_system::Object;
    use crate::memory::memory::Memory;
    use crate::tpfordev::type_system::{
        car, cdr, scheme_assoc, scheme_cons, scheme_map_clousre, set_cdr, Exp, Pair,
    };

    #[allow(dead_code)]
    fn assemble(controller_text: String, machine: &mut BasicMachine) -> Exp {
        let result = extract_labels(controller_text);
        let insts = car(&result).unwrap();
        let labels = cdr(&result).unwrap();
        let set_instruction_execution_proc = |inst| {
            let proc = make_execution_procedure(instruction_text(&inst), &labels, machine);
            let new_inst = set_instruction_execution_proc(inst, proc);
            new_inst
        };
        let insts = update_inst(&insts, set_instruction_execution_proc);
        insts
    }

    #[allow(dead_code)]
    pub fn extract_labels(text: String) -> Exp {
        let text = str_to_exp(text);
        extract_labels_iter(text)
    }

    #[allow(dead_code)]
    pub fn extract_labels_iter(text: Exp) -> Exp {
        let null = Exp::List(Pair::Nil);
        if text.is_null() {
            scheme_cons(null.clone(), null)
        } else {
            let result = extract_labels_iter(cdr(&text).unwrap());
            let insts = car(&result).unwrap();
            let labels = cdr(&result).unwrap();
            let next_inst = car(&text).unwrap();
            if next_inst.is_symbol() {
                scheme_cons(
                    insts.clone(),
                    scheme_cons(make_label_entry(next_inst, insts), labels),
                )
            } else {
                scheme_cons(scheme_cons(make_instruction(next_inst), insts), labels)
            }
        }
    }

    #[allow(dead_code)]
    pub fn update_inst<F>(insts: &Exp, f: F) -> Exp
    where
        F: FnMut(Exp) -> Exp,
    {
        scheme_map_clousre(f, insts)
    }

    #[allow(dead_code)]
    fn make_instruction(text: Exp) -> Exp {
        let null = Exp::List(Pair::Nil);
        scheme_cons(text, null)
    }

    #[allow(dead_code)]
    fn instruction_text(inst: &Exp) -> Exp {
        car(inst).unwrap()
    }

    #[allow(dead_code)]
    fn instruction_execution_proc(inst: &Exp) -> Exp {
        cdr(inst).unwrap()
    }

    #[allow(dead_code)]
    fn set_instruction_execution_proc(inst: Exp, proc: Exp) -> Exp {
        let result = set_cdr(inst, proc);
        match result {
            Ok(x) => x,
            Err(_x) => panic!("something is wrong in set_instruction_execution_pro!"),
        }
    }

    #[allow(dead_code)]
    fn make_label_entry(label_name: Exp, inst: Exp) -> Exp {
        scheme_cons(label_name, inst)
    }

    #[allow(dead_code)]
    pub fn lookup_label(labels: &Exp, label_name: &Exp) -> Option<Exp> {
        let val = scheme_assoc(labels, label_name);
        match val {
            Some(x) => {
                let result = cdr(&x).unwrap();
                Some(result)
            }
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn make_execution_procedure(inst: Exp, labels: &Exp, machine: &mut BasicMachine) -> Exp {
        /*
        let symbol = car(&inst).unwrap();
        match symbol {
            Exp::Symbol("assign") => {}
            Exp::Symbol("test") => {}
            Exp::Symbol("branch") => {}
            Exp::Symbol("goto") => {}
            Exp::Symbol("save") => {}
            Exp::Symbol("restore") => {}
            Exp::Symbol("perform") => {}
            _ => {
                println!("inst=> {}", inst);
                panic!("Unknown instruction type: ASSEMBLE")
            }
        }
        */
        Exp::Integer(1)
    }

    /*
    #[allow(dead_code)]
    fn make_assign(
        inst: Exp,
        labels: &Exp,
        machine: &mut BasicMachine,
    ) -> impl FnMut(&mut BasicMachine) {
    }
    */
    #[allow(dead_code)]
    fn make_primitive_exp(exp: Exp, machine: &mut BasicMachine, memory: &mut Memory, labels: &Exp) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        match exp {
            x if is_constant_exp(&x) => {
                let c = constant_exp_value(&x);
                let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                    let data = c;
                    data
                };
                Box::new(lambda)
            }
            x if is_label_exp(&x) => {
                let insts = lookup_label(labels, &label_exp_label(&x)).unwrap();
                let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                    let data = insts;
                    data
                };
                Box::new(lambda)
            }
            x if is_register_exp(&x) => {
                let name = exp_to_str(register_exp_reg(&x));
                let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                    let content = machine.get_register_contents(name.clone()).unwrap();
                    let mem = &(*memory);
                    match content {
                        Object::Index(x) => {
                            let result = machine.get_register_contents_as_in_memory(name, mem);
                            return str_to_exp(result);
                        },
                        _ => { return Exp::Integer(1) }
                    }
                };
                Box::new(lambda)
            }
            _ => {
                print!("{}=>", exp_to_str(exp));
                panic!("error: Unknow expression type: ASSEMBLE");
            }
        }
    }

    #[allow(dead_code)]
    fn is_register_exp(exp: &Exp) -> bool {
        is_tagged_list(exp, "reg")
    }

    #[allow(dead_code)]
    fn register_exp_reg(exp: &Exp) -> Exp {
        cadr(exp).unwrap()
    }

    #[allow(dead_code)]
    fn constant_exp_value(exp: &Exp) -> Exp {
        cadr(exp).unwrap()
    }

    #[allow(dead_code)]
    fn is_constant_exp(exp: &Exp) -> bool {
        is_tagged_list(exp, "const")
    }

    #[allow(dead_code)]
    fn label_exp_label(exp: &Exp) -> Exp {
        cadr(exp).unwrap()
    }

    #[allow(dead_code)]
    fn is_label_exp(exp: &Exp) -> bool {
        is_tagged_list(exp, "label")
    }

    #[allow(dead_code)]
    fn make_operation_exp(exp: Exp, machine: &mut BasicMachine, labels: &Exp) {}
}

#[cfg(test)]
mod test {
    use crate::{
        machine_cases::MachineCase::MachineCase, parserfordev::parser::str_to_exp,
        tpfordev::type_system::cdr,
    };

    use super::assembler::{extract_labels, lookup_label};

    #[test]
    fn lookup_label_works() {
        let factorial = MachineCase::new();
        let text = factorial.controller_text;
        let result = extract_labels(text.to_string());
        let labels = cdr(&result).unwrap();
        let label_key = str_to_exp("base-case".to_string());
        let insts = lookup_label(&labels, &label_key).unwrap();
        let checkout = str_to_exp(
            "(((assgin val (const 1))) 
                                       ((goto (reg continue))))".to_string(),
        );
        assert_eq!(insts, checkout);
    }
}
