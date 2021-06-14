mod basic_machine;
mod infrastructure;
mod memory;
mod gc;
mod representation;

use infrastructure::{register::Item, stack::Stack};
use representation::type_system::Object;
use gc::garbage_collector::garbage_collector;
use memory::memory::Memory;

fn main() {
    let mut s = Stack::new();
    s.push(Item::object(Object::Nummber(1.0)));
    s.push(Item::object(Object::LispString(String::from("winter"))));
    println!("{}", s);
}
