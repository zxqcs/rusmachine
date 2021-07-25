pub mod assembler {
    use crate::machine::basic_machine::BasicMachine;
    use crate::memory::memory::Memory;
    use crate::parserfordev::parser::{exp_to_str, str_to_exp};
    use crate::primitives::primitives::{cadr, is_tagged_list};
    use crate::representation::type_system::Object;
    use crate::tpfordev::type_system::{
        car, cdr, scheme_assoc, scheme_cons, scheme_map_clousre, set_cdr, Exp, Pair,
    };

    #[allow(dead_code)]
    fn assemble(controller_text: String, machine: &mut BasicMachine, memory: &mut Memory) -> Exp {
        let result = extract_labels(controller_text);
        let insts = car(&result).unwrap();
        let labels = cdr(&result).unwrap();
        let set_instruction_execution_proc = |inst| {
            let proc = make_execution_procedure(instruction_text(&inst), &labels, machine, memory);
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
    pub fn make_execution_procedure(
        inst: Exp,
        labels: &Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
    ) -> Exp {
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

    #[allow(dead_code)]
    pub fn make_primitive_exp(
        exp: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
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
                        }
                        _ => {
                            return content.object_to_exp();
                        }
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
        is_tagged_list(&[(*exp).clone(), Exp::Symbol("reg".to_string())])
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
        is_tagged_list(&[(*exp).clone(), Exp::Symbol("const".to_string())])
    }

    #[allow(dead_code)]
    fn label_exp_label(exp: &Exp) -> Exp {
        cadr(exp).unwrap()
    }

    #[allow(dead_code)]
    fn is_label_exp(exp: &Exp) -> bool {
        is_tagged_list(&[(*exp).clone(), Exp::Symbol("label".to_string())])
    }

    #[allow(dead_code)]
    pub fn make_operation_exp(
        exp: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    )
    {
        let op_name = exp_to_str(operation_exp_op(&exp));
        let operands = operation_exp_oprands(&exp);
        let evaluated_operands = eval_operands_iter(operands, machine, memory, labels);
    }

    fn eval_operands_iter(
        operands: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Exp {
        if operands.is_null() {
            Exp::List(Pair::Nil)
        } else {
            let item = car(&operands).unwrap();
            let r = make_primitive_exp(item, machine, memory, labels);
            let exp = consume_box_closure(r, machine, memory);
            scheme_cons(
                exp,
                eval_operands_iter(cdr(&operands).unwrap(), machine, memory, labels),
            )
        }
    }

    #[allow(dead_code)]
    pub fn is_operation_exp(exp: &Exp) -> bool {
        exp.is_pair() && is_tagged_list(&[car(exp).unwrap(), Exp::Symbol("op".to_string())])
    }

    #[allow(dead_code)]
    pub fn operation_exp_op(operation_exp: &Exp) -> Exp {
        let temp = car(operation_exp).unwrap();
        cadr(&temp).unwrap()
    }

    #[allow(dead_code)]
    pub fn operation_exp_oprands(operation_exp: &Exp) -> Exp {
        cdr(operation_exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn consume_box_closure(
        x: Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp>,
        machine: &mut BasicMachine,
        memory: &mut Memory,
    ) -> Exp {
        x(machine, memory)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        assembler::assembler::{
            consume_box_closure, is_operation_exp, operation_exp_op, operation_exp_oprands,
        },
        machine::basic_machine::BasicMachine,
        machine_cases::MachineCase::MachineCase,
        memory::memory::Memory,
        parserfordev::parser::str_to_exp,
        tpfordev::type_system::{cdr, Exp, Pair},
    };

    use super::assembler::{extract_labels, lookup_label, make_primitive_exp};

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
                                       ((goto (reg continue))))"
                .to_string(),
        );
        assert_eq!(insts, checkout);
    }

    #[test]
    fn make_primitive_exp_works() {
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        let labels = Exp::List(Pair::Nil);
        machine.initilize_registers();
        let s = "(define x '(+ 1 2))";
        machine.set_register_contents_as_in_memory("root".to_string(), s.to_string(), &mut memory);
        let mut exp = "(reg root)".to_string();
        let r1 = make_primitive_exp(str_to_exp(exp), &mut machine, &mut memory, &labels);
        let mut result = consume_box_closure(r1, &mut machine, &mut memory);
        assert_eq!(result, str_to_exp(s.to_string()));
        exp = "(const 1)".to_string();
        let r2 = make_primitive_exp(str_to_exp(exp), &mut machine, &mut memory, &labels);
        result = consume_box_closure(r2, &mut machine, &mut memory);
        assert_eq!(result, str_to_exp("1".to_string()));
    }

    #[test]
    fn is_operation_exp_works() {
        let exp = str_to_exp("((op =) (reg n) (const 1))".to_string());
        assert_eq!(is_operation_exp(&exp), true);
    }

    #[test]
    fn operation_exp_op_works() {
        let exp = str_to_exp("((op =) (reg n) (const 1))".to_string());
        assert_eq!(operation_exp_op(&exp), str_to_exp("=".to_string()));
    }

    #[test]
    fn operation_exp_operands_works() {
        let exp = str_to_exp("((op =) (reg n) (const 1))".to_string());
        assert_eq!(
            operation_exp_oprands(&exp),
            str_to_exp("((reg n) (const 1))".to_string())
        );
    }
}
