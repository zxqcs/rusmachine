mod gc;
mod infrastructure;
mod machine;
mod memory;
mod representation;

use gc::garbage_collector::garbage_collector;
use infrastructure::{register::Item, stack::Stack};
use memory::memory::Memory;
use representation::type_system::Object;

fn main() {
    let mut s = Stack::new();
    s.push(Item::Object(Object::Nummber(1.0)));
    s.push(Item::Object(Object::LispString(String::from("winter"))));
    println!("{}", s);
}
