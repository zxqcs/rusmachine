pub mod garbage_collector {
    use crate::infrastructure::register::Register;
    use crate::machine::basic_machine::BasicMachine;
    use crate::memory::memory::Memory;
    use crate::representation::type_system::Object;

    static BROKEN_HEART: Object = Object::Symbol("broken_heart");

    pub fn garbage_collector(machine: &mut BasicMachine, memory: &mut Memory) {
        machine.initilize_registers();
        machine.set_register_contents("free", Object::Index(0));

        machine.set_register_contents("scan", Object::Index(0));
        machine.set_register_contents("old", Object::Index(0));

        machine.set_register_contents(
            "relocate_continue",
            Object::Symbol("reassign-root"),
        );
        relocate_old_result_in_new(machine, memory);
    }

    fn reassign_root(machine: &mut BasicMachine, memory: &mut Memory) {
        machine.assign_from_one_register_to_another("root", "new");
        gc_loop(machine, memory);
    }

    fn relocate_old_result_in_new(machine: &mut BasicMachine, memory: &mut Memory) {
        let old = machine.get_register_contents("old").unwrap();
        if is_pair(&old, &memory) {
            relocate_pair(machine, memory);
        } else {
            machine.set_register_contents("new", old);
            where_to_go(machine, memory);
        }
    }

    fn relocate_pair(machine: &mut BasicMachine, memory: &mut Memory) {
        let old = machine.get_register_contents_ref("old").unwrap();

        if let &Object::Index(i) = old {
            let item = memory.car(i);
            machine.set_register_contents("oldcr", item);
            let oldcr = machine.get_register_contents_ref("oldcr").unwrap();

            match oldcr {
                x if *x == BROKEN_HEART => {
                    already_moved(machine, memory);
                }
                _ => {
                    machine.assign_from_one_register_to_another("new", "free");
                    machine.register_increment_by_one("free");
                    // copy the car and cdr to new memeory
                    let item = machine.get_register_contents("oldcr").unwrap();
                    perform_memeory_set(machine, memory, "new_cars", "new", item);
                    assign_to_register_from_memory(machine, memory, "the_cdrs", "oldcr", "old");
                    let item = machine.get_register_contents("oldcr").unwrap();
                    perform_memeory_set(machine, memory, "new_cdrs", "new", item);
                    // construct the broken heart
                    let item = Object::Symbol("broken_heart");
                    perform_memeory_set(machine, memory, "the_cars", "old", item);
                    let item = machine.get_register_contents("new").unwrap();
                    perform_memeory_set(machine, memory, "the_cdrs", "old", item);
                    let label = machine.get_register_contents("relocate_continue").unwrap();

                    where_to_go(machine, memory);
                }
            }
        } else {
            panic!("not a proper Index, panic when running relocate_pair!");
        }
    }

    fn assign_to_register_from_memory(
        machine: &mut BasicMachine,
        memory: &Memory,
        block: &'static str,
        to: &'static str,
        from: &'static str,
    ) {
        let index = give_a_location(machine, from);
        let mut x = Object::Nil;
        match block {
            "car" => {
                let item = memory.car(index);
                x = item;
            }
            "cdr" => {
                let item = memory.cdr(index).clone();
                x = item;
            }
            "new_car" => {
                let item = memory.new_car(index).clone();
                x = item;
            }
            "new_cdr" => {
                let item = memory.new_cdr(index).clone();
                x = item;
            }
            _ => {
                panic!("Not a legal Memeory Block");
            }
        }
        machine.set_register_contents(to, x);
    }

    fn perform_memeory_set(
        machine: &mut BasicMachine,
        memory: &mut Memory,
        block: &'static str,
        to: &'static str,
        item: Object,
    ) {
        let index = give_a_location(&machine, to);
        memory.update(block, item, index);
    }

    fn give_a_location(machine: &BasicMachine, name: &'static str) -> usize {
        let item = machine.get_register_contents_ref(name).unwrap();

        if let &Object::Index(i) = item {
            i
        } else {
            panic!("not a proper Index, panic when running give_a_location!");
        }
    }

    fn already_moved(machine: &mut BasicMachine, memory: &mut Memory) {
        let old = machine.get_register_contents_ref("old").unwrap();
        if let &Object::Index(i) = old {
            let item = *memory.the_cdrs[i].clone();
            match item {
                Object::Index(i) => {
                    let item = Object::Index(i);
                    machine.set_register_contents("new", item);
                    let label = machine.get_register_contents("relocate_continue").unwrap();
                    where_to_go(machine, memory);
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

    fn is_pair(old: &Object, memory: &Memory) -> bool {
        if let &Object::Index(i) = old {
            let s = memory.car(i);
            match s {
                Object::Pair(x) => true,
                _ => false,
            }
        } else {
            panic!("not a proper Index, panic when runnning is_pair!");
        }
    }

    fn update_car(machine: &mut BasicMachine, memory: &mut Memory) {
        let item = machine.get_register_contents("new").unwrap();
        perform_memeory_set(machine, memory, "new_cars", "scan", item);
        assign_to_register_from_memory(machine, memory, "new_cdrs", "old", "scan");
        let label = Object::Symbol("update_cdr");
        machine.set_register_contents("relocate_continue", label);
        relocate_old_result_in_new(machine, memory);
    }

    fn update_cdr(machine: &mut BasicMachine, memory: &mut Memory) {
        let item = machine.get_register_contents("new").unwrap();
        perform_memeory_set(machine, memory, "new_cdrs", "scan", item);
        machine.register_increment_by_one("scan");
        gc_loop(machine, memory);
    }

    fn gc_loop(machine: &mut BasicMachine, memory: &mut Memory) {
        let index_1 = machine.get_register_contents("scan").unwrap();
        let index_2 = machine.get_register_contents("free").unwrap();
        if index_1 == index_2 {
            gc_flip(memory);
        } else {
            assign_to_register_from_memory(machine, memory, "new_cars", "old", "scan");
            let item = Object::Symbol("update_car");
            machine.set_register_contents("relocate_continue", item);
            relocate_old_result_in_new(machine, memory);
        }
    }

    fn gc_flip(memory: &mut Memory) {
        memory.flip();
    }

    fn where_to_go(machine: &mut BasicMachine, memory: &mut Memory) {
        let label = machine.get_register_contents_ref("relocate_continue").unwrap();
        match label {
            &Object::Symbol("reassign-root") => reassign_root(machine, memory),
            &Object::Symbol("gc-loop") => gc_loop(machine, memory),
            &Object::Symbol("gc-flip") => gc_flip(memory),
            &Object::Symbol("update_car") => update_car(machine, memory),
            &Object::Symbol("update_cdr") => update_cdr(machine, memory),
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
