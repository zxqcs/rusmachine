pub mod MachineCase {
    pub struct MachineCase {
        pub name: &'static str,
        pub controller_text: &'static str,
    }

    impl MachineCase {
        pub fn new() -> Self {
            MachineCase {
                name: "factorial_machine",
                controller_text: "(controller   
                    (assign continue (label fact-done)) 
                fact-loop 
                    (test (op =) (reg n) (const 1))   
                    (branch (label base-case))   
                    (save continue) 
                    (save n)                     
                    (assign n (op -) (reg n) (const 1)) 
                    (assign continue (label after-fact))  
                    (goto (label fact-loop))   
                after-fact            
                    (restore n) 
                    (restore continue) 
                    (assign val (op *) (reg n) (reg val))
                    (goto (reg continue))
                base-case
                    (assgin val (const 1))
                    (goto (reg continue))
                fact-done)",
            }
        }
    }
}
