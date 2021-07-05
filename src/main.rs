mod gc;
mod infrastructure;
mod machine;
mod memory;
mod parser;
mod representation;

use crate::machine::basic_machine::BasicMachine;
use crate::parser::parser::{build_syntax_tree_into_memeory, tokenizer};
use gc::garbage_collector::garbage_collector;
use infrastructure::{register, stack::Stack};
use memory::memory::Memory;
use representation::type_system::Object;

fn main() {
    let mut m = Memory::new(8);
    let car_0 = Object::Pair(1);
    let cdr_0 = Object::Pair(3);
    let car_1 = Object::Integer(1);
    let cdr_1 = Object::Pair(2);
    let car_2 = Object::Integer(2);
    let cdr_2 = Object::Nil;
    let car_3 = Object::Pair(4);
    let cdr_3 = Object::Nil;
    let car_4 = Object::Integer(3);
    let cdr_4 = Object::Pair(5);
    let car_5 = Object::Pair(6);
    let cdr_5 = Object::Nil;
    let car_6 = Object::Integer(4);
    let cdr_6 = Object::Pair(7);
    let car_7 = Object::Integer(5);
    let cdr_7 = Object::Nil;
    m.update("car", car_0, 0);
    m.update("cdr", cdr_0, 0);
    m.update("car", car_1, 1);
    m.update("cdr", cdr_1, 1);
    m.update("car", car_2, 2);
    m.update("cdr", cdr_2, 2);
    m.update("car", car_3, 3);
    m.update("cdr", cdr_3, 3);
    m.update("car", car_4, 4);
    m.update("cdr", cdr_4, 4);
    m.update("car", car_5, 5);
    m.update("cdr", cdr_5, 5);
    m.update("car", car_6, 6);
    m.update("cdr", cdr_6, 6);
    m.update("car", car_7, 7);
    m.update("cdr", cdr_7, 7);
    let mut machine = BasicMachine::new();
    machine.initilize_registers();
    let item = Object::Index(0);
    machine.set_register_contents("root", item);
    let reg = machine.get_register("root").unwrap();
    reg.print_list(&m);
    println!("Results for writing into memory!");
    build_syntax_tree_into_memeory_works();
    println!("Results for using Machine's API for writting into memory");
    set_register_contents_as_in_memory_works();
}

fn build_syntax_tree_into_memeory_works() {
    let mut memory = Memory::new(10);
    let mut machine = BasicMachine::new();
    machine.initilize_registers();
    let s = "(( 1  2 )
                       (3 
                           (4  
                              5)))";
    let mut tokens = tokenizer(s);
    let root = build_syntax_tree_into_memeory(&mut tokens, &mut memory, &mut machine);
    machine.set_register_contents("root", Object::Index(root));
    let reg = machine.get_register("root").unwrap();
    reg.print_list(&memory);
    println!("{}", memory);
}

fn set_register_contents_as_in_memory_works() {
    let mut memory = Memory::new(10);
    let mut machine = BasicMachine::new();
    machine.initilize_registers();
    let s = "(( 1  2 )
                       (3 
                           (4  
                              5)))";
    machine.set_register_contents_as_in_memory("root", s, &mut memory);
    machine.get_register("root").unwrap().print_list(&memory);
}
