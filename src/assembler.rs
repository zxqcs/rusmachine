pub mod assembler {
    use crate::machine::basic_machine::BasicMachine;
    use crate::memory::memory::Memory;
    use crate::parserfordev::parser::{exp_to_str, str_to_exp};
    use crate::primitives::primitives::{cadr, cddr, is_tagged_list};
    use crate::representation::type_system::Object;
    use crate::scheme_list;
    use crate::tpfordev::type_system::{
        append, car, cdr, scheme_assoc, scheme_cons, scheme_map_clousre, set_cdr, Exp, Pair,
    };
    /*
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
    */
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
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let symbol = car(&inst).unwrap();
        let assign = Exp::Symbol("assign".to_string());
        let test = Exp::Symbol("test".to_string());
        let branch = Exp::Symbol("branch".to_string());
        let goto = Exp::Symbol("goto".to_string());
        let save = Exp::Symbol("save".to_string());
        let restore = Exp::Symbol("restore".to_string());
        let perform = Exp::Symbol("perform".to_string());
        match symbol {
            x if x == assign => make_assign(inst, machine, memory, labels),
            x if x == test => make_test(inst, machine, memory, labels),
            x if x == branch => make_branch(inst, machine, memory, labels),
            x if x == goto => make_goto(inst, machine, memory, labels),
            x if x == save => make_save(inst, machine, memory, labels),
            x if x == restore => make_restore(inst, machine, memory, labels),
            x if x == perform => make_perform(inst, machine, memory, labels),
            _ => {
                println!("inst=> {:?}", inst);
                panic!("Unknown instruction type: ASSEMBLE")
            }
        }
    }

    #[allow(dead_code)]
    pub fn make_save(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let reg_name = exp_to_str(stack_inst_reg_name(&inst));
        let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
            let data = reg_name;
            let contents = machine.get_register_contents(data).unwrap();
            machine.stack.push(contents);
            machine.advance_pc();
            Exp::Quote("ok".to_string())
        };
        Box::new(lambda)
    }

    #[allow(dead_code)]
    fn stack_inst_reg_name(stack_instruction: &Exp) -> Exp {
        cadr(stack_instruction).unwrap()
    }

    #[allow(dead_code)]
    pub fn make_restore(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let reg_name = exp_to_str(stack_inst_reg_name(&inst));
        let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
            let data = reg_name;
            let contents = machine.stack.pop().unwrap();
            machine.set_register_contents(data, contents);
            Exp::Quote("ok".to_string())
        };
        Box::new(lambda)
    }

    #[allow(dead_code)]
    pub fn make_perform(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let action = perform_action(&inst);
        if is_operation_exp(&action) {
            let action_proc = make_operation_exp(action, machine, memory, labels);
            let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                let r = consume_box_closure(action_proc, machine, memory);
                machine.advance_pc();
                Exp::Quote("ok".to_string())
            };
            Box::new(lambda)
        } else {
            panic!("Error: Bad PERFORM instruction: ASSEMBLE, {:?}", inst);
        }
    }

    #[allow(dead_code)]
    fn perform_action(inst: &Exp) -> Exp {
        cdr(inst).unwrap()
    }

    #[allow(dead_code)]
    pub fn make_goto(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let dest = goto_dest(&inst);
        match dest {
            x if is_label_exp(&x) => {
                let insts = lookup_label(labels, &label_exp_label(&x)).unwrap();
                let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                    let data = insts;
                    machine.set_register_contents("pc".to_string(), data.exp_to_object());
                    Exp::Quote("ok".to_string())
                };
                Box::new(lambda)
            }
            x if is_register_exp(&x) => {
                let reg_name = exp_to_str(register_exp_reg(&x));
                let contents = machine.get_register_contents(reg_name).unwrap();
                let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                    let data = contents;
                    machine.set_register_contents("pc".to_string(), data);
                    Exp::Quote("ok".to_string())
                };
                Box::new(lambda)
            }
            _ => {
                println!("{:?}", inst);
                panic!("Error: Bad GOTO instruction: ASSEMBLE");
            }
        }
    }

    // (goto (reg continue))
    #[allow(dead_code)]
    pub fn goto_dest(goto_instruction: &Exp) -> Exp {
        cadr(&goto_instruction).unwrap()
    }

    #[allow(dead_code)]
    pub fn make_branch(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let dest = branch_dest(&inst);
        if is_label_exp(&dest) {
            let insts = exp_to_str(lookup_label(labels, &label_exp_label(&dest)).unwrap());
            let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                let data = insts;
                let r = machine.get_register_contents("flag".to_string()).unwrap();
                if r == Object::Bool(true) {
                    machine.set_register_contents_as_in_memory("pc".to_string(), data, memory);
                } else {
                    machine.advance_pc();
                }
                Exp::Quote("ok".to_string())
            };
            Box::new(lambda)
        } else {
            println!("{:?}", inst);
            panic!("Error: Bad BRANCH instruction: ASSEMBLE");
        }
    }

    // (branch (label base-case))
    #[allow(dead_code)]
    fn branch_dest(branch_instruction: &Exp) -> Exp {
        cadr(branch_instruction).unwrap()
    }

    #[allow(dead_code)]
    pub fn make_test(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let condition = test_condition(&inst);
        if is_operation_exp(&condition) {
            let condition_proc = make_operation_exp(condition, machine, memory, labels);
            let value = consume_box_closure(condition_proc, machine, memory);
            let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                let data = value;
                machine.set_register_contents("flag".to_string(), data.exp_to_object());
                Exp::Quote("ok".to_string())
            };
            Box::new(lambda)
        } else {
            println!("{:?}", inst);
            panic!("Error: BAD TEST instruction: ASSEMBLE");
        }
    }

    // (test (op =) (reg val) (const 0))
    #[allow(dead_code)]
    pub fn test_condition(test_instruction: &Exp) -> Exp {
        cdr(test_instruction).unwrap()
    }

    #[allow(dead_code)]
    pub fn make_assign(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let reg_name = assign_reg_name(&inst);
        let value_exp = assign_value_exp(&inst);
        let mut lambda: Option<Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp>> = None;
        if is_operation_exp(&value_exp) {
            lambda = Some(make_operation_exp(value_exp, machine, memory, labels));
        } else {
            lambda = Some(make_primitive_exp(value_exp, machine, memory, labels));
        }
        let value = consume_box_closure(lambda.unwrap(), machine, memory);
        let assign_lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
            let name = exp_to_str(reg_name);
            let data = value;
            match data {
                Exp::List(ref x) => {
                    machine.set_register_contents_as_in_memory(name, exp_to_str(data), memory);
                    Exp::Quote("ok".to_string())
                }
                _ => {
                    machine.set_register_contents(name, data.exp_to_object());
                    Exp::Quote("ok".to_string())
                }
            }
        };
        Box::new(assign_lambda)
    }

    // (assgin t (op rem) (reg a) (reg b))
    // assign_reg_name: t
    #[allow(dead_code)]
    pub fn assign_reg_name(assign_instruction: &Exp) -> Exp {
        cadr(assign_instruction).unwrap()
    }

    // (assgin t (op rem) (reg a) (reg b))
    // assign_value_exp: ((op rem) (reg a) (reg b))
    #[allow(dead_code)]
    pub fn assign_value_exp(assign_instruction: &Exp) -> Exp {
        cddr(assign_instruction).unwrap()
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
        let arg = scheme_list!((*exp).clone(), Exp::Symbol("reg".to_string()));
        let r = is_tagged_list(&arg);
        match r {
            Exp::Bool(true) => true,
            _ => false,
        }
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
        let arg = scheme_list!((*exp).clone(), Exp::Symbol("const".to_string()));
        let r = is_tagged_list(&arg);
        match r {
            Exp::Bool(true) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    fn label_exp_label(exp: &Exp) -> Exp {
        cadr(exp).unwrap()
    }

    #[allow(dead_code)]
    fn is_label_exp(exp: &Exp) -> bool {
        let arg = scheme_list!((*exp).clone(), Exp::Symbol("label".to_string()));
        let r = is_tagged_list(&arg);
        match r {
            Exp::Bool(true) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn make_operation_exp(
        exp: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
        labels: &Exp,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let op_name = exp_to_str(operation_exp_op(&exp));
        let operands = operation_exp_oprands(&exp);
        let evaluated_operands = eval_operands_iter(operands, machine, memory, labels);
        println!("{:?}", evaluated_operands);
        let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
            let operands = evaluated_operands;
            let result = machine.call_op(op_name, &operands);
            result
        };
        Box::new(lambda)
    }

    // note that here the operands is organized as (arg1, arg2 ....)
    // such that the operands can be sent to machine.call_op directly
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
        if exp.is_pair() {
            let arg = scheme_list!(car(exp).unwrap(), Exp::Symbol("op".to_string()));
            let r = is_tagged_list(&arg);
            match r {
                Exp::Bool(true) => true,
                _ => false,
            }
        } else {
            false
        }
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
        primitives::primitives::{eq, is_self_evaluating, multiply},
        representation::type_system::Object,
        tpfordev::type_system::{car, cdr, Exp, Pair},
    };

    use super::assembler::{
        assign_reg_name, assign_value_exp, extract_labels, lookup_label, make_assign, make_branch,
        make_operation_exp, make_primitive_exp, make_test,
    };

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

    #[test]
    fn make_operation_exp_works() {
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        let labels = Exp::List(Pair::Nil);
        machine.initilize_registers();
        machine.add_op("is_self_evaluating".to_string(), is_self_evaluating);
        let s = "winter is coming!";
        machine.set_register_contents("root".to_string(), Object::LispString(s.to_string()));
        let exp = str_to_exp("((op is_self_evaluating) (reg root))".to_string());
        let cb = make_operation_exp(exp, &mut machine, &mut memory, &labels);
        let result = consume_box_closure(cb, &mut machine, &mut memory);
        assert_eq!(result, Exp::Bool(true));
    }

    #[test]
    fn assign_reg_name_works() {
        let inst = "(assgin t (op rem) (reg a) (reg b))".to_string();
        let assign_inst = str_to_exp(inst);
        let reg_name = assign_reg_name(&assign_inst);
        let reg_name_chekcout = str_to_exp("t".to_string());
        assert_eq!(reg_name, reg_name_chekcout);
    }

    #[test]
    fn assign_value_exp_works() {
        let inst = "(assgin t (op rem) (reg a) (reg b))".to_string();
        let assign_inst = str_to_exp(inst);
        let assign_value_exp = assign_value_exp(&assign_inst);
        let assign_value_exp_checkout = str_to_exp("((op rem) (reg a) (reg b))".to_string());
        assert_eq!(assign_value_exp, assign_value_exp_checkout);
    }

    #[test]
    fn make_assign_works() {
        let mut inst = "(assgin root (op *) (reg val) (reg exp))".to_string();
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        let labels = Exp::List(Pair::Nil);
        machine.set_register_contents("val".to_string(), Object::Number(3.14));
        machine.set_register_contents("exp".to_string(), Object::Integer(3));
        machine.add_op("*".to_string(), multiply);
        let cb = make_assign(str_to_exp(inst), &mut machine, &mut memory, &labels);
        let result = consume_box_closure(cb, &mut machine, &mut memory);
        let value = machine.get_register_contents("root".to_string()).unwrap();
        assert_eq!(value, Object::Number(9.42));
    }

    #[test]
    fn make_test_works() {
        let mut inst = str_to_exp("(test  (op =) (reg val) (const 1))".to_string());
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        machine.add_op("=".to_string(), eq);
        machine.initilize_registers();
        let labels = Exp::List(Pair::Nil);
        machine.set_register_contents("val".to_string(), Object::Integer(1));
        let cb = make_test(inst, &mut machine, &mut memory, &labels);
        let mut result = consume_box_closure(cb, &mut machine, &mut memory);
        assert_eq!(
            machine.get_register_contents("flag".to_string()).unwrap(),
            Object::Bool(true)
        );

        inst = str_to_exp("(test  (op =) (reg val) (const 3.14))".to_string());
        let cb = make_test(inst, &mut machine, &mut memory, &labels);
        result = consume_box_closure(cb, &mut machine, &mut memory);

        assert_eq!(
            machine.get_register_contents("flag".to_string()).unwrap(),
            Object::Bool(false)
        );
    }

    #[test]
    fn make_branch_works() {
        let mut inst = str_to_exp("(branch (label base-case))".to_string());
        let mut memory = Memory::new(100);
        let mut machine = BasicMachine::new();
        let text = MachineCase::new().controller_text.to_string();
        let result = extract_labels(text);
        let insts = car(&result).unwrap();
        let labels = cdr(&result).unwrap();
        machine.initilize_registers();
        machine.set_register_contents("flag".to_string(), Object::Bool(true));
        let checkout = str_to_exp(
            "(((assgin val (const 1))) 
                                       ((goto (reg continue))))"
                .to_string(),
        );
        let cb = make_branch(inst, &mut machine, &mut memory, &labels);
        let r = consume_box_closure(cb, &mut machine, &mut memory);
        let contents =
            str_to_exp(machine.get_register_contents_as_in_memory("pc".to_string(), &memory));
        assert_eq!(contents, checkout);
    }

    #[test]
    fn make_goto_works() {}

    #[test]
    fn make_save_works() {}

    #[test]
    fn make_restore_works() {}

    #[test]
    fn make_perform_works() {}
}
