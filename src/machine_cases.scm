(define gcd-machine
(make-machine
 '(a b t)
 (list (list 'rem remainder) (list '= =))
 '(test-b
     (test (op =) (reg b) (const 0))
     (branch (label gcd-done))
     (assign t (op rem) (reg a) (reg b))
     (assign a (reg b))
     (assign b (reg t))                                        
     (goto (label test-b))
   gcd-done)))

(define factorial-machine
'(controller   
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
fact-done))