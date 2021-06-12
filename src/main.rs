use infrastructure::{register::Item, stack::Stack};
use representation::type_system::Object;
mod basic_machine;
mod infrastructure;
mod representation;
mod memory;
fn main() {
    let mut s = Stack::new();
    s.push(Item::object(Object::Nummber(1.0)));
    s.push(Item::object(Object::LispString(String::from("winter"))));
    println!("{}", s);
}
