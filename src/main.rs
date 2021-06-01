use infrastructure::{register::Item, stack::Stack};

mod infrastructure;
mod basic_machine;
mod representation;
fn main() {
    let mut s = Stack::new();
    s.push(Item::Number(1.0));
    s.push(Item::Str("winter"));
    println!("{}", s);
}
