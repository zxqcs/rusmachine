mod infrastructure;
use infrastructure::register::{Item, Register};

fn main() {
    let mut r = Register::new();
    println!("{:?}", r.get());
    r.set(Item::RegisterNumber(3.14));
    println!("{:?}", r.get());
}
