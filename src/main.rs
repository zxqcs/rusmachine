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

use crate::assembler::assembler::extract_labels_alternative;
use crate::machine::basic_machine::BasicMachine;
use crate::parser::parser::{build_syntax_tree_into_memeory, tokenizer};
use crate::parserfordev::parser::{exp_to_str, print, str_to_exp};
use crate::primitives::primitives::define_variable;
use crate::tpfordev::type_system::{append, scheme_cons, Exp, Pair};
use machine_cases::MachineCase::MachineCase;
use memory::memory::Memory;
use parserfordev::parser::scheme_list_pretty_print;
use representation::type_system::Object;

fn main() {
    extract_labels_alternative_works();
    println!("Stay Strong!");
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

fn define_variable_works() {
    let mut var = Exp::Symbol("x".to_string());
    let mut val = Exp::Integer(4);
    let mut env = str_to_exp("(((a b c) 1 2 3))".to_string());
    let mut args = scheme_list!(var, val, env);
    env = define_variable(&args);
    println!("env => {}", exp_to_str(env.clone()));
    var = Exp::Symbol("y".to_string());
    val = Exp::Integer(5);
    args = scheme_list!(var, val, env);
    env = define_variable(&args);
    println!("env => {}", exp_to_str(env));
}

fn extract_labels_alternative_works() {
    let factorial = MachineCase::test_case();
    let text = factorial.controller_text.to_string();
    let mut machine = BasicMachine::new();
    let insts = extract_labels_alternative(text, &mut machine);
    scheme_list_pretty_print(&insts);
    println!("{:?}", machine.labels);
}
