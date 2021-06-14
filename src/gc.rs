pub mod garbage_collector {
    use crate::representation::type_system::Object;
    use crate::memory::memory::Memory;
    use crate::infrastructure::register::{Item, Register};

    pub fn garbage_collector() {
        let mut root = Register::new("root");
        let mut free = Register::new("free");
        let mut scan = Register::new("scan");
        let mut old = Register::new("old");
        let mut new = Register::new("new");
        let mut relocate_continue = Register::new("relocate_continue");

        free.set(Item::index(0));
        scan.set(Item::index(0));
        old.set(root.get().clone());
        reassign_root();
        gc_loop();
        gc_flip();
    }

    fn reassign_root() {}
    fn gc_loop() {}
    fn relocate_old_result_in_new() {}
    fn pair() {}
    fn update_car() {}
    fn update_cdr() {}
    fn gc_flip() {}
}