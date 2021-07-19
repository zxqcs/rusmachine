pub mod assembler {
    use crate::machine::basic_machine::BasicMachine;
    use crate::parserfordev::parser::str_to_exp;
    use crate::scheme_list;
    use crate::tpfordev::type_system::{
        append, car, cdr, scheme_cons, set_car, set_cdr, Exp, Pair,
    };

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
    pub fn update_inst(insts: Exp, labels: Exp, machine: &mut BasicMachine) -> Exp {
        Exp::Integer(1)
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
    fn lookup_label(labels: Exp, label_name: Exp) -> Exp {
        Exp::Integer(1)
    }
}
