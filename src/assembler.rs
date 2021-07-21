pub mod assembler {
    use crate::machine::basic_machine::BasicMachine;
    use crate::parserfordev::parser::str_to_exp;
    use crate::scheme_list;
    use crate::tpfordev::type_system::{
        append, car, cdr, scheme_assoc, scheme_cons, scheme_map, scheme_map_clousre, set_car,
        set_cdr, Exp, Pair,
    };

    #[allow(dead_code)]
    fn assemble(controller_text: &'static str, machine: &mut BasicMachine) -> Exp {
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
    pub fn extract_labels(text: &'static str) -> Exp {
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
        let result = extract_labels(text);
        let labels = cdr(&result).unwrap();
        let label_key = str_to_exp("base-case");
        let insts = lookup_label(&labels, &label_key).unwrap();
        let checkout = str_to_exp(
            "(((assgin val (const 1))) 
                                       ((goto (reg continue))))",
        );
        assert_eq!(insts, checkout);
    }
}
