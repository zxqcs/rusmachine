pub mod primitives {
    use crate::{
        infrastructure::stack::Stack,
        machine::basic_machine::BasicMachine,
        memory::memory::Memory,
        parser::parser::read_scheme_programs_from_stdin,
        parserfordev::parser::{exp_to_str, str_to_exp},
        representation::type_system::Object,
        scheme_list,
        tpfordev::type_system::{append, car, cdr, scheme_cons, set_car, set_cdr, Exp, Pair},
    };
    #[allow(dead_code)]
    pub fn cadr(exp: &Exp) -> Result<Exp, &'static str> {
        let s = cdr(exp).unwrap();
        car(&s)
    }

    #[allow(dead_code)]
    pub fn cddr(exp: &Exp) -> Result<Exp, &'static str> {
        let s1 = cdr(exp).unwrap();
        cdr(&s1)
    }

    #[allow(dead_code)]
    pub fn caar(exp: &Exp) -> Result<Exp, &'static str> {
        let s1 = car(exp).unwrap();
        car(&s1)
    }

    #[allow(dead_code)]
    pub fn cdar(exp: &Exp) -> Result<Exp, &'static str> {
        let s1 = car(exp).unwrap();
        cdr(&s1)
    }

    #[allow(dead_code)]
    pub fn cdddr(exp: &Exp) -> Result<Exp, &'static str> {
        let s1 = cdr(exp).unwrap();
        let s2 = cdr(&s1).unwrap();
        cdr(&s2)
    }

    #[allow(dead_code)]
    pub fn cadddr(exp: &Exp) -> Result<Exp, &'static str> {
        let s1 = cdddr(exp).unwrap();
        car(&s1)
    }

    #[allow(dead_code)]
    pub fn caddr(exp: &Exp) -> Result<Exp, &'static str> {
        let s1 = cdr(exp).unwrap();
        let s2 = cdr(&s1).unwrap();
        car(&s2)
    }

    #[allow(dead_code)]
    pub fn caadr(exp: &Exp) -> Result<Exp, &'static str> {
        let s1 = cdr(exp).unwrap();
        let s2 = car(&s1).unwrap();
        car(&s2)
    }

    #[allow(dead_code)]
    pub fn cdadr(exp: &Exp) -> Result<Exp, &'static str> {
        let s1 = cadr(exp).unwrap();
        let s2 = cdr(&s1);
        s2
    }
    /* The procedurs belwo are machine ops */
    #[allow(dead_code)]
    pub fn machine_statistics(machine: &mut BasicMachine, _memory: &mut Memory) {
        machine.stack.statistics();
    }

    // this procedure is called each time we enter the driver loop since that
    // error may happen in last round.
    #[allow(dead_code)]
    pub fn initialize_stack(machine: &mut BasicMachine, _memory: &mut Memory) {
        machine.stack = Stack::new();
    }

    #[allow(dead_code)]
    pub fn initialize_env(machine: &mut BasicMachine, memory: &mut Memory) {
        machine.set_register_contents_as_in_memory(&"evn".to_string(), "".to_string(), memory);
    }

    #[allow(dead_code)]
    pub fn prompt_for_input(_machine: &mut BasicMachine, _memory: &mut Memory) {
        println!("=> ");
    }

    #[allow(dead_code)]
    pub fn read(machine: &mut BasicMachine, memory: &mut Memory) {
        let mut s = "".to_string();
        let r = read_scheme_programs_from_stdin(&mut s);
        match r {
            Ok(()) => {
                let exp = str_to_exp(s.clone());
                match exp {
                    Exp::List(_x) => {
                        let index = memory.write(s, machine);
                        machine.set_register_contents(&"exp".to_string(), Object::Index(index));
                    }
                    _ => {
                        machine.set_register_contents(&"exp".to_string(), exp.exp_to_object());
                    }
                }
            }
            Err(x) => {
                panic!("Error when reading input {}", x);
            }
        }
    }

    /* The procedures below are semantic ops for machine */
    // semantic primitives for IO
    #[allow(dead_code)]
    pub fn announce_output(_exp: &Exp) -> Exp {
        println!("=> ");
        Exp::Quote("ok".to_string())
    }

    #[allow(dead_code)]
    pub fn user_print(exp: &Exp) -> Exp {
        let arg = car(exp).unwrap();
        let r = is_compound_procedure(&arg);
        match r {
            Exp::Bool(true) => {
                let val = scheme_list!(
                    Exp::Quote("compound-procedure".to_string()),
                    procedure_parameters(&arg),
                    procedure_body(&arg)
                );
                println!("{}", exp_to_str(val));
            }
            Exp::Bool(false) => {
                println!("{}", exp_to_str(arg));
            }
            _ => panic!("Error: USER-PRINT {}", exp_to_str(r)),
        }
        Exp::Quote("ok".to_string())
    }

    // semantic primitives that return a Scheme Object(Exp)
    #[allow(dead_code)]
    pub fn multiply(exp: &Exp) -> Exp {
        let lhs = car(exp).unwrap();
        let rhs = cadr(exp).unwrap();
        match lhs {
            Exp::Integer(x) => match rhs {
                Exp::Integer(y) => return Exp::Integer(x * y),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x as f32 * y),
                _ => {
                    panic!("Error: Invalid operation for MULTIPLY!");
                }
            },
            Exp::FloatNumber(x) => match rhs {
                Exp::Integer(y) => return Exp::FloatNumber(x * y as f32),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x * y),
                _ => {
                    panic!("Error: Invalid operation for MULTIPLY!");
                }
            },
            _ => {
                panic!("Error: Invalid operation for MULTIPLY!");
            }
        }
    }

    #[allow(dead_code)]
    pub fn substract(exp: &Exp) -> Exp {
        let lhs = car(exp).unwrap();
        let rhs = cadr(exp).unwrap();
        match lhs {
            Exp::Integer(x) => match rhs {
                Exp::Integer(y) => return Exp::Integer(x - y),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x as f32 - y),
                _ => {
                    panic!("Error: Invalid operation for MULTIPLY!");
                }
            },
            Exp::FloatNumber(x) => match rhs {
                Exp::Integer(y) => return Exp::FloatNumber(x - y as f32),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x - y),
                _ => {
                    panic!("Error: Invalid operation for MULTIPLY!");
                }
            },
            _ => {
                panic!("Error: Invalid operation for MULTIPLY!");
            }
        }
    }

    #[allow(dead_code)]
    pub fn procedure_parameters(exp: &Exp) -> Exp {
        let p = car(exp).unwrap();
        cadr(&p).unwrap()
    }

    #[allow(dead_code)]
    pub fn procedure_body(exp: &Exp) -> Exp {
        let p = car(exp).unwrap();
        caddr(&p).unwrap()
    }

    // semantic operations that return a Scheme bool value
    #[allow(dead_code)]
    pub fn is_eq(exp: &Exp) -> Exp {
        let lhs = car(exp).unwrap();
        let rhs = cadr(exp).unwrap();
        if lhs == rhs {
            Exp::Bool(true)
        } else {
            Exp::Bool(false)
        }
    }

    #[allow(dead_code)]
    pub fn is_tagged_list(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let tag = cadr(args).unwrap();
        if let Exp::Symbol(y) = tag {
            if let Exp::Symbol(x) = car(&exp).unwrap() {
                if x == y {
                    Exp::Bool(true)
                } else {
                    Exp::Bool(false)
                }
            } else {
                Exp::Bool(false)
            }
        } else {
            panic!("Invalid tag!");
        }
    }

    // It should be noted that a Exp::Bool is returned instead of a real Rust bool
    // Because this procedure is used as a primitive op for our machine, hence, a Scheme bool is
    // returned here!
    #[allow(dead_code)]
    pub fn is_variable(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let flag = exp.is_symbol();
        match flag {
            true => Exp::Bool(true),
            false => Exp::Bool(false),
        }
    }

    // It should be noted that a Exp::Bool is returned instead of a real Rust bool
    // Because this procedure is used as a primitive op for our machine, hence, a Scheme bool is
    // returned here!
    #[allow(dead_code)]
    pub fn is_self_evaluating(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let r = exp.is_bool()
            || exp.is_null()
            || exp.is_number()
            || exp.is_quote()
            || exp.is_string()
            || exp.is_self_evuluating_list();
        match r {
            true => Exp::Bool(true),
            false => Exp::Bool(false),
        }
    }

    #[allow(dead_code)]
    pub fn is_compound_procedure(exp: &Exp) -> Exp {
        let p = car(exp).unwrap();
        let tag = Exp::Symbol("procedure".to_string());
        let args = scheme_list!(p, tag);
        is_tagged_list(&args)
    }

    // semantic primitives and helper procedures that has a effect on environment
    #[allow(dead_code)]
    pub fn assignment_variable(args: &Exp) -> Exp {
        cadr(args).unwrap()
    }

    #[allow(dead_code)]
    pub fn make_procedure(args: &Exp) -> Exp {
        let parameters = car(args).unwrap();
        let body = cadr(args).unwrap();
        let env = caddr(args).unwrap();
        let tag = Exp::Symbol("procedure".to_string());
        scheme_list!(tag, parameters.clone(), body.clone(), env.clone())
    }

    // note that by define_variable, for example,
    // a env (((a b c) 1 2 3))
    // is transformed to (((a b c x) 1 2 3 4))
    // by adding new binding (x 4) to the original env
    #[allow(dead_code)]
    pub fn define_variable(args: &Exp) -> Exp {
        let target_var = car(args).unwrap();
        let target_val = cadr(args).unwrap();
        let env = caddr(args).unwrap();

        if env == Exp::List(Pair::Nil) {
            let frame = scheme_list!(scheme_list!(target_var), target_val);
            scheme_list!(frame)
        } else {
            let frame = first_frame(&env);
            let temp_frame = scan_and_define(target_var, target_val, frame);
            set_car(env, temp_frame).unwrap()
        }
    }

    #[allow(dead_code)]
    fn scan_and_define(target_var: Exp, target_val: Exp, frame: Exp) -> Exp {
        let vars = frame_variables(&frame);
        let vals = frame_values(&frame);
        if vars == Exp::List(Pair::Nil) {
            add_binding_to_frame(target_var, target_val, frame)
        } else if target_var == car(&vars).unwrap() {
            let temp_vals = set_car(vals, target_val).unwrap();
            make_frame(vars, temp_vals)
        } else {
            let mut temp_frame = make_frame(cdr(&vars).unwrap(), cdr(&vals).unwrap());
            temp_frame = scan_and_define(target_var, target_val, temp_frame);
            let temp_vars = set_cdr(vars, frame_variables(&temp_frame)).unwrap();
            let temp_vals = set_cdr(vals, frame_values(&temp_frame)).unwrap();
            make_frame(temp_vars, temp_vals)
        }
    }

    // ((x y z ) 1 2 3)
    #[allow(dead_code)]
    fn make_frame(variables: Exp, values: Exp) -> Exp {
        scheme_cons(variables, values)
    }

    #[allow(dead_code)]
    fn frame_variables(frame: &Exp) -> Exp {
        car(frame).unwrap()
    }

    #[allow(dead_code)]
    fn frame_values(frame: &Exp) -> Exp {
        cdr(frame).unwrap()
    }

    #[allow(dead_code)]
    fn first_frame(env: &Exp) -> Exp {
        car(&env).unwrap()
    }

    #[allow(dead_code)]
    pub fn add_binding_to_frame(var: Exp, val: Exp, frame: Exp) -> Exp {
        if frame == Exp::List(Pair::Nil) {
            scheme_list!(scheme_list!(var), val)
        } else {
            let temp = set_car(frame.clone(), scheme_cons(var, frame_variables(&frame))).unwrap();
            set_cdr(temp, scheme_cons(val, frame_values(&frame))).unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        append,
        parserfordev::parser::str_to_exp,
        primitives::primitives::{
            caadr, caar, cadddr, caddr, cadr, cdadr, cdar, cdddr, cddr, define_variable,
            is_self_evaluating, is_tagged_list, multiply,
        },
        scheme_cons, scheme_list,
        tpfordev::type_system::Exp,
        Pair,
    };

    use super::primitives::add_binding_to_frame;

    #[test]
    fn cadr_works() {
        let mut items = str_to_exp("((1 2) 3 4 5)".to_string());
        assert_eq!(cadr(&items).unwrap(), str_to_exp("3".to_string()));
        items = str_to_exp("(assign continue (label fact-done))".to_string());
        assert_eq!(cadr(&items).unwrap(), str_to_exp("continue".to_string()));
    }

    #[test]
    fn cddr_works() {
        let mut items = str_to_exp("((1 2) (3 4 5) 6 7)".to_string());
        assert_eq!(cddr(&items).unwrap(), str_to_exp("(6 7)".to_string()));
        items = str_to_exp("(test (op =) (reg n) (const 1))".to_string());
        assert_eq!(
            cddr(&items).unwrap(),
            str_to_exp("((reg n) (const 1))".to_string())
        );
    }

    #[test]
    fn caar_works() {
        let items = str_to_exp("((1 2) 3 4 5)".to_string());
        assert_eq!(caar(&items).unwrap(), str_to_exp("1".to_string()));
    }

    #[test]
    fn cdar_works() {
        let mut items = str_to_exp("((1 2) (3 4 5) 6 7)".to_string());
        assert_eq!(cdar(&items).unwrap(), str_to_exp("(2)".to_string()));
        items = str_to_exp("((reg n) (const 1))".to_string());
        assert_eq!(cdar(&items).unwrap(), str_to_exp("(n)".to_string()));
    }

    #[test]
    fn cdddr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)".to_string());
        assert_eq!(cdddr(&items).unwrap(), str_to_exp("(7)".to_string()));
    }

    #[test]
    fn cadddr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)".to_string());
        assert_eq!(cadddr(&items).unwrap(), str_to_exp("7".to_string()));
    }

    #[test]
    fn caddr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)".to_string());
        assert_eq!(caddr(&items).unwrap(), str_to_exp("6".to_string()));
    }

    #[test]
    fn caadr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)".to_string());
        assert_eq!(caadr(&items).unwrap(), str_to_exp("3".to_string()));
    }

    #[test]
    fn cdadr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)".to_string());
        assert_eq!(cdadr(&items).unwrap(), str_to_exp("(4 5)".to_string()));
    }

    #[test]
    fn is_tagged_list_works() {
        let mut items = str_to_exp("(reg continue)".to_string());
        let mut exp = scheme_list!(items, Exp::Symbol("reg".to_string()));
        assert_eq!(is_tagged_list(&exp), Exp::Bool(true));
        items = str_to_exp("(const 1)".to_string());
        exp = scheme_list!(items, Exp::Symbol("const".to_string()));
        assert_eq!(is_tagged_list(&exp), Exp::Bool(true));
    }

    #[test]
    fn multiply_works() {
        let lhs = Exp::Integer(3);
        let rhs = Exp::FloatNumber(2.14);
        let args = scheme_list!(lhs, rhs);
        assert_eq!(multiply(&args), Exp::FloatNumber(6.42));
    }

    #[test]
    fn add_binding_to_frame_works() {
        let mut frame = str_to_exp("((a b c) 1 2 3)".to_string());
        let mut var = Exp::Symbol("x".to_string());
        let mut val = Exp::Integer(4);
        let new_frame = add_binding_to_frame(var, val, frame);
        assert_eq!(new_frame, str_to_exp("((x a b c) 4 1 2 3)".to_string()));
        frame = str_to_exp("".to_string());
        var = Exp::Symbol("y".to_string());
        val = Exp::Integer(5);
        let new_frame = add_binding_to_frame(var, val, frame);
        assert_eq!(new_frame, str_to_exp("((y) 5)".to_string()));
    }

    #[test]
    fn define_variable_works() {
        let mut var = Exp::Symbol("x".to_string());
        let mut val = Exp::Integer(4);
        let mut env = str_to_exp("(((a b c) 1 2 3))".to_string());
        let mut args = scheme_list!(var.clone(), val.clone(), env);
        env = define_variable(&args);
        assert_eq!(env, str_to_exp("(((a b c x) 1 2 3 4))".to_string()));
        var = Exp::Symbol("y".to_string());
        val = Exp::Integer(5);
        args = scheme_list!(var, val, env);
        env = define_variable(&args);
        assert_eq!(env, str_to_exp("(((a b c x y) 1 2 3 4 5))".to_string()));
        var = Exp::Symbol("z".to_string());
        val = Exp::Integer(6);
        env = Exp::List(Pair::Nil);
        args = scheme_list!(var, val, env);
        env = define_variable(&args);
        assert_eq!(env, str_to_exp("(((z) 6))".to_string()));
        var = Exp::Symbol("c".to_string());
        val = Exp::Integer(4);
        env = str_to_exp("(((a b c) 1 2 3))".to_string());
        args = scheme_list!(var.clone(), val.clone(), env);
        env = define_variable(&args);
        assert_eq!(env, str_to_exp("(((a b c) 1 2 4))".to_string()));
    }

    #[test]
    fn is_self_evaluating_works() {
        let mut exp = str_to_exp("(())".to_string());
        assert_eq!(is_self_evaluating(&exp), Exp::Bool(true));
        exp = str_to_exp("((1 2 (3 4 ()) 5))".to_string());
        assert_eq!(is_self_evaluating(&exp), Exp::Bool(true));
        exp = str_to_exp("((1 2 'summer (3 ()) (\"winter is coming\"  5)))".to_string());
        assert_eq!(is_self_evaluating(&exp), Exp::Bool(true));
    }
    /*
    #[test]
    fn read_works() {
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        machine.add_machine_op("read".to_string(), read);
        let mut memory = Memory::new(20);
        machine.call_machine_op("read".to_string(), &mut memory);
        let content = machine.get_register_contents_as_in_memory(&"exp".to_string(), &memory);
        let checkout = str_to_exp("(define (square x) (* x x))".to_string());
        assert_eq!(checkout, str_to_exp(content));
    }
    */
}
