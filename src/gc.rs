pub mod garbage_collector {
    use crate::machine::basic_machine::BasicMachine;
    use crate::memory::memory::Memory;
    use crate::representation::type_system::Object;

    #[allow(dead_code)]
    pub fn garbage_collector(machine: &mut BasicMachine, memory: &mut Memory) {
        machine.initilize_registers();
        machine.set_register_contents(&"free".to_string(), Object::Index(0));

        machine.set_register_contents(&"scan".to_string(), Object::Index(0));
        machine.set_register_contents(&"old".to_string(), Object::Index(0));

        machine.set_register_contents(
            &"relocate_continue".to_string(),
            Object::Symbol("reassign-root".to_string()),
        );
        relocate_old_result_in_new(machine, memory);
    }

    fn reassign_root(machine: &mut BasicMachine, memory: &mut Memory) {
        machine.assign_from_one_register_to_another(&"root".to_string(), &"new".to_string());
        gc_loop(machine, memory);
    }

    fn relocate_old_result_in_new(machine: &mut BasicMachine, memory: &mut Memory) {
        let old = machine.get_register_contents(&"old".to_string()).unwrap();
        if is_pair(&old, &memory) {
            relocate_pair(machine, memory);
        } else {
            machine.set_register_contents(&"new".to_string(), old);
            where_to_go(machine, memory);
        }
    }

    fn relocate_pair(machine: &mut BasicMachine, memory: &mut Memory) {
        let broken_heart = Object::Symbol("broken_heart".to_string());

        let old = machine
            .get_register_contents_ref("old".to_string())
            .unwrap();

        if let &Object::Index(i) = old {
            let item = memory.car(i);
            machine.set_register_contents(&"oldcr".to_string(), item);
            let oldcr = machine
                .get_register_contents_ref("oldcr".to_string())
                .unwrap();

            match oldcr {
                x if *x == broken_heart => {
                    already_moved(machine, memory);
                }
                _ => {
                    machine.assign_from_one_register_to_another(
                        &"new".to_string(),
                        &"free".to_string(),
                    );
                    machine.register_increment_by_one(&"free".to_string());
                    // copy the car and cdr to new memeory
                    let item = machine.get_register_contents(&"oldcr".to_string()).unwrap();
                    perform_memeory_set(machine, memory, "new_car", "new".to_string(), item);
                    assign_to_register_from_memory(
                        machine,
                        memory,
                        "the_cdrs",
                        "oldcr".to_string(),
                        "old".to_string(),
                    );
                    let item = machine.get_register_contents(&"oldcr".to_string()).unwrap();
                    perform_memeory_set(machine, memory, "new_cdr", "new".to_string(), item);
                    // construct the broken heart
                    perform_memeory_set(machine, memory, "car", "old".to_string(), broken_heart);
                    let item = machine.get_register_contents(&"new".to_string()).unwrap();
                    perform_memeory_set(machine, memory, "cdr", "old".to_string(), item);
                    let _label = machine
                        .get_register_contents(&"relocate_continue".to_string())
                        .unwrap();

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
        to: String,
        from: String,
    ) {
        let index = give_a_location(machine, from);
        let x;
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
        machine.set_register_contents(&to, x);
    }

    fn perform_memeory_set(
        machine: &mut BasicMachine,
        memory: &mut Memory,
        block: &'static str,
        to: String,
        item: Object,
    ) {
        let index = give_a_location(&machine, to);
        memory.update(block, item, index);
    }

    fn give_a_location(machine: &BasicMachine, name: String) -> usize {
        let item = machine.get_register_contents_ref(name).unwrap();

        if let &Object::Index(i) = item {
            i
        } else {
            panic!("not a proper Index, panic when running give_a_location!");
        }
    }

    fn already_moved(machine: &mut BasicMachine, memory: &mut Memory) {
        let old = machine
            .get_register_contents_ref("old".to_string())
            .unwrap();
        if let &Object::Index(i) = old {
            let item = *memory.the_cdrs[i].clone();
            match item {
                Object::Index(i) => {
                    let item = Object::Index(i);
                    machine.set_register_contents(&"new".to_string(), item);
                    let _label = machine
                        .get_register_contents(&"relocate_continue".to_string())
                        .unwrap();
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
                Object::Pair(_x) => true,
                _ => false,
            }
        } else {
            panic!("not a proper Index, panic when runnning is_pair!");
        }
    }

    fn update_car(machine: &mut BasicMachine, memory: &mut Memory) {
        let item = machine.get_register_contents(&"new".to_string()).unwrap();
        perform_memeory_set(machine, memory, "new_car", "scan".to_string(), item);
        assign_to_register_from_memory(
            machine,
            memory,
            "new_cdr",
            "old".to_string(),
            "scan".to_string(),
        );
        let label = Object::Symbol("update_cdr".to_string());
        machine.set_register_contents(&"relocate_continue".to_string(), label);
        relocate_old_result_in_new(machine, memory);
    }

    fn update_cdr(machine: &mut BasicMachine, memory: &mut Memory) {
        let item = machine.get_register_contents(&"new".to_string()).unwrap();
        perform_memeory_set(machine, memory, "new_cdrs", "scan".to_string(), item);
        machine.register_increment_by_one(&"scan".to_string());
        gc_loop(machine, memory);
    }

    fn gc_loop(machine: &mut BasicMachine, memory: &mut Memory) {
        let index_1 = machine.get_register_contents(&"scan".to_string()).unwrap();
        let index_2 = machine.get_register_contents(&"free".to_string()).unwrap();
        if index_1 == index_2 {
            gc_flip(memory);
        } else {
            assign_to_register_from_memory(
                machine,
                memory,
                "new_cars",
                "old".to_string(),
                "scan".to_string(),
            );
            let item = Object::Symbol("update_car".to_string());
            machine.set_register_contents(&"relocate_continue".to_string(), item);
            relocate_old_result_in_new(machine, memory);
        }
    }

    fn gc_flip(memory: &mut Memory) {
        memory.flip();
    }

    fn where_to_go(machine: &mut BasicMachine, memory: &mut Memory) {
        let label = machine
            .get_register_contents_ref("relocate_continue".to_string())
            .unwrap();

        let label_one = Object::Symbol("reassign-root".to_string());
        let label_two = Object::Symbol("gc-loop".to_string());
        let label_three = Object::Symbol("gc_flip".to_string());
        let label_four = Object::Symbol("update_car".to_string());
        let label_five = Object::Symbol("update_cdr".to_string());

        match label {
            x if *x == label_one => reassign_root(machine, memory),
            x if *x == label_two => gc_loop(machine, memory),
            x if *x == label_three => gc_flip(memory),
            x if *x == label_four => update_car(machine, memory),
            x if *x == label_five => update_cdr(machine, memory),
            _ => panic!("not a proper label!"),
        }
    }
}

#[cfg(test)]
mod test {}
