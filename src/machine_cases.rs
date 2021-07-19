pub mod machine_cases {
    pub struct machine_case {
        pub name: &'static str,
        pub controller_text: &'static str,
    }

    impl machine_case {
        pub fn new() -> Self {
            machine_case {
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
