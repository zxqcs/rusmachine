pub mod garbage_collector {
    use crate::representation::type_system::Object;
    use crate::memory::memory::Memory;
    use crate::infrastructure::register::{Item, Register};

    const  Broken_Heart: Item = Item::object(Object::Symbol("broken_heart"));

    pub fn garbage_collector(memory: &mut Memory) {
        let mut root = Register::new("root");
        let mut free = Register::new("free");
        let mut scan = Register::new("scan");
        let mut old = Register::new("old");
        let mut oldcr = Register::new("oldcr");
        let mut new = Register::new("new");
        let mut relocate_continue = Register::new("relocate_continue");

        // initialization of Memory here for develop now, to be fixed later
        free.set(Item::index(0));
        scan.set(Item::index(0));
        old.set(root.get().clone());
        reassign_root();
        gc_loop();
        gc_flip();
    }

    fn reassign_root() {}
    fn gc_loop() {}
    fn relocate_old_result_in_new(old: &Register) {}
    fn relocate_pair() {}

    fn is_pair(old: &Register, memory: &Memory) -> bool {
        if let &Item::index(i) = old.get() {
            let s = &memory.the_cars[i];
            match &**s {
                Object::Pair(x) => true,
                _ => false,
            }
        } else {
            panic!("panic when runnning is_pair!");
        }
    }

    fn update_car() {}
    fn update_cdr() {}
    fn gc_flip() {}
}

#[cfg(test)]
mod test {
    use super::garbage_collector;

    #[test]
    fn is_pair_works() {

    }
    
    fn relocate_old_result_in_new_works() {

    }
    
}