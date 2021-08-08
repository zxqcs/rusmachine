pub mod assembler {
    use crate::machine::basic_machine::BasicMachine;
    use crate::memory::memory::Memory;
    use crate::parserfordev::parser::{exp_to_str, str_to_exp};
    use crate::primitives::primitives::{cadr, cddr, is_tagged_list};
    use crate::representation::type_system::Object;
    use crate::scheme_list;
    use crate::tpfordev::type_system::{
        append, car, cdr, scheme_cons, scheme_for_each, set_cdr, Exp, Pair,
    };
    #[allow(dead_code)]
    pub fn assemble(controller_text: String, machine: &mut BasicMachine, memory: &mut Memory) {
        let insts = extract_labels_alternative(controller_text, machine);
        machine.install_raw_instructions(&insts);
        let set_instruction_execution_proc = |inst| {
            let proc = make_execution_procedure(inst, machine, memory);
            machine.instruction_sequence.push(Some(proc));
        };
        update_inst(&insts, set_instruction_execution_proc);
    }

    #[allow(dead_code)]
    pub fn extract_labels(text: String) -> Exp {
        let text = str_to_exp(text);
        extract_labels_iter(text)
    }

    #[allow(dead_code)]
    pub fn extract_labels_alternative(text: String, machine: &mut BasicMachine) -> Exp {
        let text = str_to_exp(text);
        let mut offset: usize = 0;
        extract_labels_iter_alternative(text, machine, &mut offset)
    }

    #[allow(dead_code)]
    pub fn extract_labels_iter_alternative(
        text: Exp,
        machine: &mut BasicMachine,
        offset: &mut usize,
    ) -> Exp {
        if text.is_null() {
            Exp::List(Pair::Nil)
        } else {
            let next_inst = car(&text).unwrap();
            if next_inst.is_symbol() {
                machine.labels.insert(exp_to_str(next_inst), *offset);
                extract_labels_iter_alternative(cdr(&text).unwrap(), machine, offset)
            } else {
                *offset = (*offset + 1) as usize;
                scheme_cons(
                    next_inst,
                    extract_labels_iter_alternative(cdr(&text).unwrap(), machine, offset),
                )
            }
        }
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
    pub fn update_inst<F>(insts: &Exp, f: F)
    where
        F: FnMut(Exp),
    {
        scheme_for_each(f, insts)
    }

    #[allow(dead_code)]
    fn make_instruction(text: Exp) -> Exp {
        let null = Exp::List(Pair::Nil);
        scheme_cons(text, null)
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
    pub fn lookup_label(machine: &mut BasicMachine, label_name: &String) -> Option<usize> {
        let index = machine.labels.get(label_name);
        match index {
            Some(x) => Some(*x),
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn make_execution_procedure(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        println!("inst to be assembled => {}", exp_to_str(inst.clone()));
        let symbol = car(&inst).unwrap();
        let assign = Exp::Symbol("assign".to_string());
        let test = Exp::Symbol("test".to_string());
        let branch = Exp::Symbol("branch".to_string());
        let goto = Exp::Symbol("goto".to_string());
        let save = Exp::Symbol("save".to_string());
        let restore = Exp::Symbol("restore".to_string());
        let perform = Exp::Symbol("perform".to_string());
        match symbol {
            x if x == assign => make_assign(inst, machine, memory),
            x if x == test => make_test(inst, machine, memory),
            x if x == branch => make_branch(inst, machine, memory),
            x if x == goto => make_goto(inst, machine, memory),
            x if x == save => make_save(inst, machine, memory),
            x if x == restore => make_restore(inst, machine, memory),
            x if x == perform => make_perform(inst, machine, memory),
            _ => {
                panic!("Unknown instruction type => {}: ASSEMBLE", exp_to_str(inst));
            }
        }
    }

    // note that for each of the following make_* procedures, operations that are done outside of
    // lambda(closure) are done in assembly time while other operations that are done in lambda(closure)
    // when consumed, are done in running time(simulated time)
    #[allow(dead_code)]
    pub fn make_save(
        inst: Exp,
        _machine: &mut BasicMachine,
        _memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let reg_name = exp_to_str(stack_inst_reg_name(&inst));
        let lambda = |machine: &mut BasicMachine, _memory: &mut Memory| {
            let data = reg_name;
            let contents = machine.get_register_contents(&data).unwrap();
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
        _machine: &mut BasicMachine,
        _memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let reg_name = exp_to_str(stack_inst_reg_name(&inst));
        let lambda = |machine: &mut BasicMachine, _memory: &mut Memory| {
            let data = reg_name;
            let contents = machine.stack.pop().unwrap();
            machine.set_register_contents(&data, contents);
            machine.advance_pc();
            Exp::Quote("ok".to_string())
        };
        Box::new(lambda)
    }

    #[allow(dead_code)]
    pub fn make_perform(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let action = perform_action(&inst);
        let op_name = operation_exp_op(&action);
        println!("op_name=>{}", exp_to_str(op_name.clone()));
        let set_varialbe_value = Exp::Symbol("set-variable-value".to_string());
        let define_variable = Exp::Symbol("define-variable".to_string());
        let flag;
        match op_name {
            x if x == set_varialbe_value => flag = true,
            x if x == define_variable => flag = true,
            _ => flag = false,
        }
        println!("flag=>{}", flag);
        if is_operation_exp(&action) {
            let action_proc = make_operation_exp(action, machine, memory);
            let lambda = move |machine: &mut BasicMachine, memory: &mut Memory| {
                let data = flag;
                let r = consume_box_closure(action_proc, machine, memory);
                if data {
                    machine.set_register_contents_as_in_memory(
                        &"env".to_string(),
                        exp_to_str(r),
                        memory,
                    );
                }
                machine.advance_pc();
                Exp::Quote("ok".to_string())
            };
            Box::new(lambda)
        } else {
            panic!("Error: Bad PERFORM instruction: ASSEMBLE, {:?}", inst);
        }
    }

    // (perform (op set-variable-value) (reg unev) (reg val) (reg env))
    #[allow(dead_code)]
    fn perform_action(inst: &Exp) -> Exp {
        cdr(inst).unwrap()
    }

    #[allow(dead_code)]
    pub fn make_goto(
        inst: Exp,
        machine: &mut BasicMachine,
        _memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let dest = goto_dest(&inst);
        match dest {
            x if is_label_exp(&x) => {
                let index = lookup_label(machine, &exp_to_str(label_exp_label(&x))).unwrap();
                let lambda = move |machine: &mut BasicMachine, _memory: &mut Memory| {
                    let data = index;
                    machine.set_register_contents(&"pc".to_string(), Object::Index(data));
                    Exp::Quote("ok".to_string())
                };
                Box::new(lambda)
            }
            x if is_register_exp(&x) => {
                let reg_name = exp_to_str(register_exp_reg(&x));
                let lambda = |machine: &mut BasicMachine, _memory: &mut Memory| {
                    let data = reg_name;
                    machine.assign_from_one_register_to_another(&"pc".to_string(), &data);
                    Exp::Quote("ok".to_string())
                };
                Box::new(lambda)
            }
            _ => {
                panic!(
                    "Error: Bad GOTO instruction {:?}: ASSEMBLE",
                    exp_to_str(inst)
                );
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
        _memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let dest = branch_dest(&inst);
        if is_label_exp(&dest) {
            let index = lookup_label(machine, &exp_to_str(label_exp_label(&dest))).unwrap();
            let lambda = move |machine: &mut BasicMachine, _memory: &mut Memory| {
                let data = index;
                let r = machine.get_register_contents(&"flag".to_string()).unwrap();
                if r == Object::Bool(true) {
                    machine.set_register_contents(&"pc".to_string(), Object::Index(data));
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
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let condition = test_condition(&inst);
        if is_operation_exp(&condition) {
            let condition_proc = make_operation_exp(condition, machine, memory);
            let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                let data = condition_proc;
                let value = consume_box_closure(data, machine, memory);
                machine.set_register_contents(&"flag".to_string(), value.exp_to_object());
                machine.advance_pc();
                Exp::Quote("ok".to_string())
            };
            Box::new(lambda)
        } else {
            panic!("Error: BAD TEST instruction {}: ASSEMBLE", exp_to_str(inst));
        }
    }

    // (test (op =) (reg val) (const 0))
    #[allow(dead_code)]
    pub fn test_condition(test_instruction: &Exp) -> Exp {
        cdr(test_instruction).unwrap()
    }

    // ( assign continue ( label fact-done))
    #[allow(dead_code)]
    pub fn make_assign(
        inst: Exp,
        machine: &mut BasicMachine,
        memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let reg_name = assign_reg_name(&inst);
        let value_exp = assign_value_exp(&inst);
        let lambda;
        if is_operation_exp(&value_exp) {
            lambda = Some(make_operation_exp(value_exp, machine, memory));
        } else {
            lambda = Some(make_primitive_exp(
                car(&value_exp).unwrap(),
                machine,
                memory,
            ));
        }
        let assign_lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
            let name = exp_to_str(reg_name);
            let data = lambda;
            let value = consume_box_closure(data.unwrap(), machine, memory);
            match value {
                Exp::List(ref _x) => {
                    machine.set_register_contents_as_in_memory(&name, exp_to_str(value), memory);
                }
                _ => {
                    machine.set_register_contents(&name, value.exp_to_object());
                }
            }
            machine.advance_pc();
            Exp::Quote("ok".to_string())
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
        _memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        match exp {
            x if is_constant_exp(&x) => {
                let c = constant_exp_value(&x);
                let lambda = |_machine: &mut BasicMachine, _memory: &mut Memory| {
                    let data = c;
                    data
                };
                Box::new(lambda)
            }
            x if is_label_exp(&x) => {
                let index = lookup_label(machine, &exp_to_str(label_exp_label(&x))).unwrap();
                let lambda =
                    move |_machine: &mut BasicMachine, _memory: &mut Memory| Exp::Index(index);
                Box::new(lambda)
            }
            x if is_register_exp(&x) => {
                let name = exp_to_str(register_exp_reg(&x));
                let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
                    let data = name;
                    let content = machine.get_register_contents(&data).unwrap();
                    let mem = &(*memory);
                    match content {
                        Object::Index(_x) => {
                            let result = machine.get_register_contents_as_in_memory(&data, mem);
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
                panic!(
                    "error: Unknow expression type {}: ASSEMBLE",
                    exp_to_str(exp)
                );
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

    // (label fact-done)
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
        _machine: &mut BasicMachine,
        _memory: &mut Memory,
    ) -> Box<dyn FnOnce(&mut BasicMachine, &mut Memory) -> Exp> {
        let op_name = exp_to_str(operation_exp_op(&exp));
        let operands = operation_exp_oprands(&exp);
        let lambda = |machine: &mut BasicMachine, memory: &mut Memory| {
            let data = operands;
            let evaluated_operands = eval_operands_iter(data, machine, memory);
            let result = machine.call_semantic_op(op_name, &evaluated_operands);
            result
        };
        Box::new(lambda)
    }

    // note that here the operands is organized as (arg1, arg2 ....)
    // such that the operands can be sent to machine.call_semantic_op directly
    fn eval_operands_iter(operands: Exp, machine: &mut BasicMachine, memory: &mut Memory) -> Exp {
        if operands.is_null() {
            Exp::List(Pair::Nil)
        } else {
            let item = car(&operands).unwrap();
            let r = make_primitive_exp(item, machine, memory);
            let exp = consume_box_closure(r, machine, memory);
            scheme_cons(
                exp,
                eval_operands_iter(cdr(&operands).unwrap(), machine, memory),
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
            consume_box_closure, is_operation_exp, make_goto, operation_exp_op,
            operation_exp_oprands,
        },
        machine::basic_machine::BasicMachine,
        machine_cases::machine_case::MachineCase,
        memory::memory::Memory,
        parserfordev::parser::str_to_exp,
        primitives::primitives::{define_variable, is_eq, is_self_evaluating, multiply},
        representation::type_system::Object,
        scheme_list,
        tpfordev::type_system::{Exp, Pair},
    };

    use super::assembler::{
        assign_reg_name, assign_value_exp, extract_labels_alternative, lookup_label, make_assign,
        make_branch, make_operation_exp, make_perform, make_primitive_exp, make_restore, make_save,
        make_test,
    };
    use crate::tpfordev::type_system::{append, scheme_cons};
    #[test]
    fn lookup_label_works() {
        let mut machine = BasicMachine::new();
        let factorial = MachineCase::test_case();
        let text = factorial.controller_text;
        let _result = extract_labels_alternative(text.to_string(), &mut machine);
        let index = lookup_label(&mut machine, &"base-case".to_string()).unwrap();
        assert_eq!(index, 12 as usize);
    }

    #[test]
    fn make_primitive_exp_works() {
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        let s = "(define x '(+ 1 2))";
        machine.set_register_contents_as_in_memory(&"root".to_string(), s.to_string(), &mut memory);
        let mut exp = "(reg root)".to_string();
        let r1 = make_primitive_exp(str_to_exp(exp), &mut machine, &mut memory);
        let mut result = consume_box_closure(r1, &mut machine, &mut memory);
        assert_eq!(result, str_to_exp(s.to_string()));
        exp = "(const 1)".to_string();
        let r2 = make_primitive_exp(str_to_exp(exp), &mut machine, &mut memory);
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
        machine.initilize_registers();
        machine.add_semantic_op("is_self_evaluating".to_string(), is_self_evaluating);
        let s = "winter is coming!";
        machine.set_register_contents(&"root".to_string(), Object::LispString(s.to_string()));
        let exp = str_to_exp("((op is_self_evaluating) (reg root))".to_string());
        let cb = make_operation_exp(exp, &mut machine, &mut memory);
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
        let inst = "(assgin root (op *) (reg val) (reg exp))".to_string();
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        machine.set_register_contents(&"val".to_string(), Object::Number(3.14));
        machine.set_register_contents(&"exp".to_string(), Object::Integer(3));
        machine.add_semantic_op("*".to_string(), multiply);
        let cb = make_assign(str_to_exp(inst), &mut machine, &mut memory);
        let _result = consume_box_closure(cb, &mut machine, &mut memory);
        let value = machine.get_register_contents(&"root".to_string()).unwrap();
        assert_eq!(value, Object::Number(9.42));
    }

    #[test]
    fn make_test_works() {
        let mut inst = str_to_exp("(test  (op =) (reg val) (const 1))".to_string());
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        machine.add_semantic_op("=".to_string(), is_eq);
        machine.initilize_registers();
        machine.set_register_contents(&"val".to_string(), Object::Integer(1));
        let cb = make_test(inst, &mut machine, &mut memory);
        let mut _result = consume_box_closure(cb, &mut machine, &mut memory);
        assert_eq!(
            machine.get_register_contents(&"flag".to_string()).unwrap(),
            Object::Bool(true)
        );

        inst = str_to_exp("(test  (op =) (reg val) (const 3.14))".to_string());
        let cb = make_test(inst, &mut machine, &mut memory);
        _result = consume_box_closure(cb, &mut machine, &mut memory);

        assert_eq!(
            machine.get_register_contents(&"flag".to_string()).unwrap(),
            Object::Bool(false)
        );
    }

    #[test]
    fn make_branch_works() {
        let inst = str_to_exp("(branch (label base-case))".to_string());
        let mut memory = Memory::new(100);
        let mut machine = BasicMachine::new();
        let text = MachineCase::new().controller_text.to_string();
        let _result = extract_labels_alternative(text, &mut machine);
        machine.initilize_registers();
        machine.set_register_contents(&"flag".to_string(), Object::Bool(true));
        let cb = make_branch(inst, &mut machine, &mut memory);
        let _r = consume_box_closure(cb, &mut machine, &mut memory);
        let contents = machine.get_register_contents(&"pc".to_string()).unwrap();
        assert_eq!(contents, Object::Index(12));
    }

    #[test]
    fn make_goto_works() {
        let inst = str_to_exp("(goto (reg continue))".to_string());
        let mut memory = Memory::new(50);
        let mut machine = BasicMachine::new();
        let text = MachineCase::new().controller_text.to_string();
        let _insts = extract_labels_alternative(text, &mut machine);
        let base_case = lookup_label(&mut machine, &"base-case".to_string()).unwrap();
        machine.initilize_registers();
        machine.set_register_contents(&"continue".to_string(), Object::Index(base_case));
        let cb = make_goto(inst, &mut machine, &mut memory);
        let _r = consume_box_closure(cb, &mut machine, &mut memory);
        let result = machine.get_register_contents(&"pc".to_string()).unwrap();
        assert_eq!(result, Object::Index(12 as usize));
    }

    #[test]
    fn make_save_works() {
        let inst = str_to_exp("(save val)".to_string());
        let mut memory = Memory::new(20);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        machine.set_register_contents(&"val".to_string(), Object::Number(3.14));
        let cb = make_save(inst, &mut machine, &mut memory);
        let _r = consume_box_closure(cb, &mut machine, &mut memory);
        let item = (*machine.stack.peek().unwrap()).clone();
        assert_eq!(item, Object::Number(3.14));
    }

    #[test]
    fn make_restore_works() {
        let inst = str_to_exp("(restore val)".to_string());
        let mut memory = Memory::new(20);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        machine.stack.push(Object::Integer(9));
        let cb = make_restore(inst, &mut machine, &mut memory);
        let _r = consume_box_closure(cb, &mut machine, &mut memory);
        let item = machine.get_register_contents(&"val".to_string()).unwrap();
        assert_eq!(item, Object::Integer(9));
    }

    #[test]
    fn make_perform_works() {
        let inst =
            str_to_exp("(perform (op define-variable) (reg unev) (reg val) (reg env))".to_string());
        let mut memory = Memory::new(20);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        machine.add_semantic_op("define-variable".to_string(), define_variable);
        machine.set_register_contents(&"unev".to_string(), Object::Symbol("x".to_string()));
        machine.set_register_contents(&"val".to_string(), Object::Integer(3));
        machine.set_register_contents_as_in_memory(
            &"env".to_string(),
            "(((y) 1))".to_string(),
            &mut memory,
        );
        let cb = make_perform(inst, &mut machine, &mut memory);
        let _r = consume_box_closure(cb, &mut machine, &mut memory);
        let content = machine.get_register_contents_as_in_memory(&"env".to_string(), &memory);
        let checkout = scheme_list!(scheme_list!(
            scheme_list!(Exp::Symbol("y".to_string()), Exp::Symbol("x".to_string())),
            Exp::Integer(1),
            Exp::Integer(3)
        ));
        assert_eq!(str_to_exp(content), checkout);
    }

    #[test]
    fn assemble_works() {}
}
