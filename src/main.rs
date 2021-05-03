mod infrastructure;
use infrastructure::register::{Item, Register};

fn main() {
    let mut r = Register::new("Alpha");
    println!("{}", r);
    r.set(Item::RegisterNumber(3.14));
    println!("{}", r);
}
