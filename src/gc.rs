pub mod garbage_collector {
    use crate::infrastructure::register::{Item, Register};
    use crate::machine::basic_machine::BasicMachine;
    use crate::memory::memory::Memory;
    use crate::representation::type_system::Object;

    static BROKEN_HEART: Item = Item::Object(Object::Symbol("broken_heart"));

    pub fn garbage_collector(mut memory: &mut Memory) {
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        machine.set_register_contents("free", Item::Object(Object::Index(0)));

        machine.set_register_contents("scan", Item::Object(Object::Index(0)));
        machine.set_register_contents("old", Item::Object(Object::Index(0)));

        machine.set_register_contents(
            "relocate_continue",
            Item::Object(Object::Symbol("reassign-root")),
        );
        relocate_old_result_in_new(&mut machine, &mut memory);
    }

    fn reassign_root() {}
    fn gc_loop() {}
    fn relocate_old_result_in_new(machine: &mut BasicMachine, memory: &mut Memory) {
        let old = machine.get_register_contents("old").unwrap();
        if is_pair(old, &memory) {
            relocate_pair(machine, &memory);
        } else {
            let item = (*old).clone();
            machine.set_register_contents("new", item);
            let label = machine.get_register_contents("relocate_continue").unwrap();
            where_to_go(label);
        }
    }

    fn relocate_pair(machine: &mut BasicMachine, memory: &Memory) {
        let old = machine.get_register_contents("old").unwrap();

        if let &Item::Object(Object::Index(i)) = old {
            let item = *memory.the_cars[i].clone();
            machine.set_register_contents("oldcr", Item::Object(item));
            let oldcr = machine.get_register_contents("oldcr").unwrap();

            match oldcr {
                x if *x == BROKEN_HEART => {
                    already_moved(machine, memory);
                }
                _ => {}
            }
        } else {
            panic!("not a proper Index, panic when running relocate_pair!");
        }
    }

    fn already_moved(machine: &mut BasicMachine, memory: &Memory) {
        let old = machine.get_register_contents("old").unwrap();
        if let &Item::Object(Object::Index(i)) = old {
            let item = *memory.the_cdrs[i].clone();
            match item {
                Object::Index(i) => {
                    let item = Item::Object(Object::Index(i));
                    machine.set_register_contents("new", item);
                    let label = machine.get_register_contents("relocate_continue").unwrap();
                    where_to_go(label);
                }
                _ => {
                    panic!(
                        "not a proper forwarding address stored in cdr, 
                              panic when running already_moved!"
                    );
                }
            }
        } else {
            panic!("not a proper Index in old, panic when running already_moved!");
        }
    }

    fn is_pair(old: &Item, memory: &Memory) -> bool {
        if let &Item::Object(Object::Index(i)) = old {
            let s = &memory.the_cars[i];
            match &**s {
                Object::Pair(x) => true,
                _ => false,
            }
        } else {
            panic!("not a proper Index, panic when runnning is_pair!");
        }
    }

    fn update_car() {}
    fn update_cdr() {}
    fn gc_flip() {}

    fn where_to_go(label: &Item) {
        match label {
            &Item::Object(Object::Symbol("reassign-root")) => reassign_root(),
            &Item::Object(Object::Symbol("gc-loop")) => gc_loop(),
            &Item::Object(Object::Symbol("gc-flip")) => gc_flip(),
            _ => panic!("not a proper label!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::garbage_collector;

    #[test]
    fn is_pair_works() {}

    fn relocate_old_result_in_new_works() {}
}
