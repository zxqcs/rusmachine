pub mod garbage_collector {
    use crate::representation::type_system::Object;
    use crate::memory::memory::Memory;
    use crate::infrastructure::register::{Item, Register};
    use crate::machine::basic_machine::BasicMachine;
    
    const  Broken_Heart: Item = Item::object(Object::Symbol("broken_heart"));

    pub fn garbage_collector(memory: &mut Memory) {
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
    }

    fn reassign_root() {}
    fn gc_loop() {}
    fn relocate_old_result_in_new(old: &Register) {}

    fn relocate_pair(oldcr: &mut Register, old: &Register, new: &mut Register, memory: &Memory) {
        if let &Item::index(i) = old.get() {
            let item = *memory.the_cars[i].clone();
            oldcr.set(Item::object(item));
            match oldcr.get() {
                &Broken_Heart => {
                    let item = (*memory.the_cdrs[i]).clone();
                    new.set(Item::object(item));
                }
            }
        } else {
            panic!("not a proper index, panic when running relocate_pair!");
        }

    }

    fn is_pair(old: &Register, memory: &Memory) -> bool {
        if let &Item::index(i) = old.get() {
            let s = &memory.the_cars[i];
            match &**s {
                Object::Pair(x) => true,
                _ => false,
            }
        } else {
            panic!("not a proper index, panic when runnning is_pair!");
        }
    }

    fn update_car() {}
    fn update_cdr() {}
    fn gc_flip() {}
    /*
    fn where_to_go() {
        let item = relocate_continue.get();
        match item {
            &Item::object(Object::Symbol("reassign-root")) => reassign_root(),
            &Item::object(Object::Symbol("gc-loop")) => gc_loop(),
            &Item::object(Object::Symbol("gc-flip")) => gc_flip(),
            _ => panic!("not a proper label!"),
        }
    } 
    */
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