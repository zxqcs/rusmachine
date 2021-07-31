mod assembler;
mod gc;
mod infrastructure;
mod machine;
mod machine_cases;
mod memory;
mod parser;
mod parserfordev;
mod primitives;
mod representation;
mod tpfordev;

use crate::assembler::assembler::{consume_box_closure, make_primitive_exp, make_test};
use crate::machine::basic_machine::BasicMachine;
use crate::parser::parser::{build_syntax_tree_into_memeory, tokenizer};
use crate::parserfordev::parser::{print, str_to_exp};
use crate::tpfordev::type_system::{append, scheme_cons, Exp, Pair};
use assembler::assembler::{extract_labels, make_operation_exp};
use machine_cases::MachineCase::MachineCase;
use memory::memory::Memory;
use parserfordev::parser::scheme_list_pretty_print;
use primitives::primitives::is_self_evaluating;
use representation::type_system::Object;
use tpfordev::type_system::{car, cdr};

fn main() {
    let text = MachineCase::new().controller_text.to_string();
    let result = extract_labels(text);
    let insts = car(&result).unwrap();
    let labels = cdr(&result).unwrap();
    scheme_list_pretty_print(&insts);
    scheme_list_pretty_print(&labels);
}

#[allow(dead_code)]
fn build_syntax_tree_into_memeory_works() {
    let mut memory = Memory::new(10);
    let mut machine = BasicMachine::new();
    machine.initilize_registers();
    let s = "(( 1  2 )
                       (3 
                           (4  
                              5)))";
    let mut tokens = tokenizer(s.to_string());
    let root = build_syntax_tree_into_memeory(&mut tokens, &mut memory, &mut machine);
    machine.set_register_contents(&"root".to_string(), Object::Index(root));
    let reg = machine.get_register(&"root".to_string()).unwrap();
    reg.print_list(&memory);
    println!("{}", memory);
}

#[allow(dead_code)]
fn set_register_contents_as_in_memory_works() {
    let mut memory = Memory::new(10);
    let mut machine = BasicMachine::new();
    machine.initilize_registers();
    let s = "(( 1  2 )
                       (3 
                           (4  
                              5)))";
    machine.set_register_contents_as_in_memory(&"root".to_string(), s.to_string(), &mut memory);
    machine
        .get_register(&"root".to_string())
        .unwrap()
        .print_list(&memory);
}

#[allow(dead_code)]
fn str_to_exp_works() {
    let s1 = "true";
    let s2 = "3.14";
    let s3 = "(( 1  2 )
    (3 
        (4  
           5)))";
    let s4 = "(define x \"winter is coming\")";
    let s5 = "'( 1 ( 2 3))";
    let exp1 = str_to_exp(s1.to_string());
    let exp2 = str_to_exp(s2.to_string());
    let exp3 = str_to_exp(s3.to_string());
    let exp4 = str_to_exp(s4.to_string());
    let exp5 = str_to_exp(s5.to_string());
    assert_eq!(exp1, Exp::Bool(true));
    assert_eq!(exp2, Exp::FloatNumber(3.14));
    assert_eq!(
        exp3,
        scheme_list!(
            scheme_list!(Exp::Integer(1), Exp::Integer(2)),
            scheme_list!(
                Exp::Integer(3),
                scheme_list!(Exp::Integer(4), Exp::Integer(5))
            )
        )
    );
    assert_eq!(
        exp4,
        scheme_list!(
            Exp::Symbol("define".to_string()),
            Exp::Symbol("x".to_string()),
            Exp::SchemeString("winter is coming".to_string())
        )
    );
    assert_eq!(exp5, Exp::Quote("( 1 ( 2 3))".to_string()));
    print(exp1);
    print(exp2);
    print(exp3);
    print(exp4);
    print(exp5);
}

#[allow(dead_code)]
fn make_primitive_exp_works() {
    let mut memory = Memory::new(10);
    let mut machine = BasicMachine::new();
    let labels = Exp::List(Pair::Nil);
    machine.initilize_registers();
    let s = "(define x '(+ 1 2))";
    machine.set_register_contents_as_in_memory(&"root".to_string(), s.to_string(), &mut memory);
    let exp = "(reg root)".to_string();
    let r = make_primitive_exp(str_to_exp(exp), &mut machine, &mut memory, &labels);
    let result = consume_box_closure(r, &mut machine, &mut memory);
    assert_eq!(result, str_to_exp(s.to_string()));
}

fn make_operation_exp_works() {
    let mut memory = Memory::new(10);
    let mut machine = BasicMachine::new();
    let labels = Exp::List(Pair::Nil);
    machine.initilize_registers();
    machine.add_op("is_self_evaluating".to_string(), is_self_evaluating);
    let s = "winter is coming!";
    machine.set_register_contents(&"root".to_string(), Object::LispString(s.to_string()));
    let exp = str_to_exp("((op is_self_evaluating) (reg root))".to_string());
    let cb = make_operation_exp(exp, &mut machine, &mut memory, &labels);
    let result = consume_box_closure(cb, &mut machine, &mut memory);
    assert_eq!(result, Exp::Bool(true));
}

fn make_test_works() {
    let mut inst = str_to_exp("(test  (op =) (reg val) (const 1))".to_string());
    let mut memory = Memory::new(10);
    let mut machine = BasicMachine::new();
    machine.initilize_registers();
    let labels = Exp::List(Pair::Nil);
    machine.set_register_contents(&"val".to_string(), Object::Integer(1));
    let cb = make_test(inst, &mut machine, &mut memory, &labels);
    let result = consume_box_closure(cb, &mut machine, &mut memory);
    assert_eq!(
        machine.get_register_contents(&"flag".to_string()).unwrap(),
        Object::Bool(true)
    );
}
