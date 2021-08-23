pub mod primitives {
    use crate::{
        infrastructure::stack::Stack,
        machine::basic_machine::BasicMachine,
        memory::memory::Memory,
        parser::parser::read_scheme_programs_from_stdin,
        parserfordev::parser::{exp_to_str, str_to_exp},
        representation::type_system::Object,
        scheme_list,
        tpfordev::type_system::{
            append, car, cdr, list_length, scheme_cons, set_car, set_cdr, Exp, Pair,
        },
    };

    /* primitives that are used as basic Scheme list operations
    note that these procedurs are not used as machine and semantic primitives
    directly, but machine promitives are built upon these list operations */
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

    /* The procedurs below are primitives of machine ops
    which is, has machine and memory as args */
    #[allow(dead_code)]
    pub fn machine_statistics(machine: &mut BasicMachine, _memory: &mut Memory) -> Exp {
        machine.stack.statistics();
        Exp::Quote("ok".to_string())
    }

    // this procedure is called each time we enter the driver loop since that
    // error may happen in last round.
    #[allow(dead_code)]
    pub fn initialize_stack(machine: &mut BasicMachine, _memory: &mut Memory) -> Exp {
        machine.stack = Stack::new();
        Exp::Quote("ok".to_string())
    }

    #[allow(dead_code)]
    pub fn prompt_for_input(_machine: &mut BasicMachine, _memory: &mut Memory) -> Exp {
        println!("=> ");
        Exp::Quote("ok".to_string())
    }

    #[allow(dead_code)]
    pub fn read(machine: &mut BasicMachine, memory: &mut Memory) -> Exp {
        let mut s = "".to_string();
        let r = read_scheme_programs_from_stdin(&mut s);
        match r {
            Ok(()) => {
                // println!("{}", s.clone());
                let exp = str_to_exp(s.clone());
                // println!("{:?}", exp.clone());
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
        Exp::Quote("ok".to_string())
    }

    /* The procedures below are semantic ops for machine
    such as self_evaluating in eval dispatch */
    // semantic primitives for IO
    #[allow(dead_code)]
    pub fn announce_output(_exp: &Exp) -> Exp {
        println!("=> ");
        Exp::Quote("ok".to_string())
    }

    #[allow(dead_code)]
    pub fn user_print(exp: &Exp) -> Exp {
        let arg = car(exp).unwrap();
        // println!("{:?}", arg.clone());
        let r = is_compound_procedure(&scheme_list!(arg.clone()));
        match r {
            Exp::Bool(true) => {
                let val = scheme_list!(
                    Exp::Quote("compound-procedure".to_string()),
                    procedure_parameters(exp),
                    procedure_body(exp)
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
    // although these primitives below are more like machine ops,
    // but they are independent of machine and memory state,
    // such that these primitives are classified as semantic primitives

    #[allow(dead_code)]
    pub fn multiply(exp: &Exp) -> Exp {
        let lhs = car(exp).unwrap();
        let rhs = cadr(exp).unwrap();
        match lhs {
            Exp::Integer(x) => match rhs {
                Exp::Integer(y) => return Exp::Integer(x * y),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x as f32 * y),
                _ => {
                    panic!("Error: Invalid operand for MULTIPLY!");
                }
            },
            Exp::FloatNumber(x) => match rhs {
                Exp::Integer(y) => return Exp::FloatNumber(x * y as f32),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x * y),
                _ => {
                    panic!("Error: Invalid operand for MULTIPLY!");
                }
            },
            _ => {
                panic!("Error: Invalid operand for MULTIPLY!");
            }
        }
    }

    #[allow(dead_code)]
    pub fn division(exp: &Exp) -> Exp {
        let lhs = car(exp).unwrap();
        let rhs = cadr(exp).unwrap();
        if rhs == Exp::Integer(0) || rhs == Exp::FloatNumber(0.0) {
            panic!("Error: denominator can't be ZERO!");
        }
        match lhs {
            Exp::Integer(x) => match rhs {
                Exp::Integer(y) => return Exp::FloatNumber(x as f32 / y as f32),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x as f32 / y),
                _ => {
                    panic!("Error: Invalid operand for DIVISION!");
                }
            },
            Exp::FloatNumber(x) => match rhs {
                Exp::Integer(y) => return Exp::FloatNumber(x / y as f32),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x / y),
                _ => {
                    panic!("Error: Invalid operand for DIVISION!");
                }
            },
            _ => {
                panic!("Error: Invalid operation for DIVISION!");
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
                    panic!("Error: Invalid operand for SUBSTRACT!");
                }
            },
            Exp::FloatNumber(x) => match rhs {
                Exp::Integer(y) => return Exp::FloatNumber(x - y as f32),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x - y),
                _ => {
                    panic!("Error: Invalid operation for SBUSTRACT!");
                }
            },
            _ => {
                panic!("Error: Invalid operation for SUBSTRACT!");
            }
        }
    }

    #[allow(dead_code)]
    pub fn add(exp: &Exp) -> Exp {
        let lhs = car(exp).unwrap();
        let rhs = cadr(exp).unwrap();
        match lhs {
            Exp::Integer(x) => match rhs {
                Exp::Integer(y) => return Exp::Integer(x + y),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x as f32 + y),
                _ => {
                    panic!("Error: Invalid operand for ADD!");
                }
            },
            Exp::FloatNumber(x) => match rhs {
                Exp::Integer(y) => return Exp::FloatNumber(x + y as f32),
                Exp::FloatNumber(y) => return Exp::FloatNumber(x + y),
                _ => {
                    panic!("Error: Invalid operand for ADD!");
                }
            },
            _ => {
                panic!("Error: Invalid operand for ADD!");
            }
        }
    }

    // semantic primitives that are related to apply dispatch
    #[allow(dead_code)]
    pub fn procedure_parameters(exp: &Exp) -> Exp {
        let p = car(exp).unwrap();
        cadr(&p).unwrap()
    }

    // procedure sample: (procedure (x) (* x x))
    #[allow(dead_code)]
    pub fn procedure_body(exp: &Exp) -> Exp {
        let p = car(exp).unwrap();
        caddr(&p).unwrap()
    }

    // note that there's no env component in making procedure, just parameters and body
    #[allow(dead_code)]
    pub fn make_procedure(args: &Exp) -> Exp {
        let parameters = car(args).unwrap();
        let body = cadr(args).unwrap();
        let tag = Exp::Symbol("procedure".to_string());
        scheme_list!(tag, parameters.clone(), body.clone())
    }

    #[allow(dead_code)]
    pub fn operands(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        cdr(&exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn operator(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        car(&exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn empty_arglist(_args: &Exp) -> Exp {
        Exp::List(Pair::Nil)
    }

    #[allow(dead_code)]
    pub fn is_no_operands(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        if exp.is_null() {
            Exp::Bool(true)
        } else {
            Exp::Bool(false)
        }
    }

    #[allow(dead_code)]
    pub fn is_last_operand(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let ops = cdr(&exp).unwrap();
        if ops.is_null() {
            Exp::Bool(true)
        } else {
            Exp::Bool(false)
        }
    }

    #[allow(dead_code)]
    pub fn first_operand(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        car(&exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn rest_operands(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        cdr(&exp).unwrap()
    }

    // args: ((primitive car) (1 2 3))
    // proc: (primitive car)
    // argl: (1 2 3)
    #[allow(dead_code)]
    pub fn meta_apply_primitive_procedure(args: &Exp) -> Exp {
        let proc = car(args).unwrap();
        let argl = cadr(args).unwrap();
        let symbol = cadr(&proc).unwrap();
        match symbol {
            x if x == Exp::Symbol("car".to_string()) => car(&car(&argl).unwrap()).unwrap(),
            x if x == Exp::Symbol("cdr".to_string()) => cdr(&car(&argl).unwrap()).unwrap(),
            x if x == Exp::Symbol("cons".to_string()) => {
                let lhs = car(&argl).unwrap();
                let rhs = cadr(&argl).unwrap();
                scheme_cons(lhs, rhs)
            }
            x if x == Exp::Symbol("null?".to_string()) => {
                let exp = car(&argl).unwrap();
                let r = exp.is_null();
                match r {
                    true => Exp::Bool(true),
                    false => Exp::Bool(false),
                }
            }
            x if x == Exp::Symbol("+".to_string()) => add(&argl),
            x if x == Exp::Symbol("-".to_string()) => substract(&argl),
            x if x == Exp::Symbol("*".to_string()) => multiply(&argl),
            x if x == Exp::Symbol("/".to_string()) => division(&argl),
            x if x == Exp::Symbol("<".to_string()) => is_smaller_than(&argl),
            x if x == Exp::Symbol(">".to_string()) => is_larger_than(&argl),
            x if x == Exp::Symbol("=".to_string()) => is_eq(&argl),
            _ => {
                panic!(
                    "Error: primitives not implemented yet: {}",
                    exp_to_str(symbol)
                );
            }
        }
    }

    //primitive that is used to debug evaluator controller as to print the content
    // of specific register in the process of evaluation
    #[allow(dead_code)]
    pub fn print_reg_content(args: &Exp) -> Exp {
        let content = car(args).unwrap();
        println!("{}", exp_to_str(content));
        Exp::Quote("ok".to_string())
    }

    #[allow(dead_code)]
    pub fn print_message(args: &Exp) -> Exp {
        let message = car(args).unwrap();
        println!("{}", exp_to_str(message));
        Exp::Quote("ok".to_string())
    } 
        
    // semantic primitives that are related to lambda dispatch
    #[allow(dead_code)]
    pub fn lambda_parameters(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        cadr(&exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn lambda_body(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        cddr(&exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn adjoin_arg(args: &Exp) -> Exp {
        let arg = car(args).unwrap();
        let arglist = cadr(args).unwrap();
        append(arglist, scheme_list!(arg))
    }

    #[allow(dead_code)]
    pub fn make_lambad(args: &Exp) -> Exp {
        let parameters = car(args).unwrap();
        let body = cadr(args).unwrap();
        scheme_cons(
            Exp::Symbol("lambda".to_string()),
            scheme_cons(parameters, body),
        )
    }

    // semantic primitives that are related to if dispatch
    #[allow(dead_code)]
    pub fn if_predicate(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        cadr(&exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn if_alternative(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        if cdddr(&exp).unwrap().is_null() {
            panic!("Error: if alternative not exist!");
        } else {
            cadddr(&exp).unwrap()
        }
    }

    #[allow(dead_code)]
    pub fn if_consequent(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        caddr(&exp).unwrap()
    }

    // semantic primitives that are realted to begin dispatch
    #[allow(dead_code)]
    //  (begin (set! x 5) (+ x 1))
    pub fn begin_actions(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        cdr(&exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn first_exp(args: &Exp) -> Exp {
        let seq = car(args).unwrap();
        car(&seq).unwrap()
    }

    #[allow(dead_code)]
    pub fn rest_exps(args: &Exp) -> Exp {
        let seq = car(args).unwrap();
        cdr(&seq).unwrap()
    }

    // semantic primitives that are related to assignment dispatch
    #[allow(dead_code)]
    pub fn assignment_variable(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        cadr(&exp).unwrap()
    }

    #[allow(dead_code)]
    pub fn assignment_value(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        caddr(&exp).unwrap()
    }

    // semantic primitives that are related to definition dispatch
    #[allow(dead_code)]
    pub fn definition_variable(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        if cadr(&exp).unwrap().is_symbol() {
            cadr(&exp).unwrap()
        } else {
            caadr(&exp).unwrap()
        }
    }

    #[allow(dead_code)]
    pub fn definition_value(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        // println!("def_value=> {}", exp_to_str(exp.clone()));
        if cadr(&exp).unwrap().is_symbol() {
            if cddr(&exp).unwrap().is_null() {
                Exp::List(Pair::Nil)
            } else {
                caddr(&exp).unwrap()
            }
        } else {
            let parameters = cdadr(&exp).unwrap();
            let body = cddr(&exp).unwrap();
            let args = scheme_list!(parameters, body);
            make_lambad(&args)
        }
    }

    // semantic operations that return a Scheme bool value
    #[allow(dead_code)]
    pub fn is_true(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        // println!("{}", exp_to_str(exp.clone()));
        match exp {
            Exp::Bool(true) => Exp::Bool(true),
            Exp::Bool(false) => Exp::Bool(false),
            _ => panic!("Error: not a legal BOOL value: {}", exp_to_str(exp)),
        }
    }

    #[allow(dead_code)]
    pub fn is_last_exp(args: &Exp) -> Exp {
        let seq = car(args).unwrap();
        let exp = cdr(&seq).unwrap();
        if exp.is_null() {
            Exp::Bool(true)
        } else {
            Exp::Bool(false)
        }
    }

    #[allow(dead_code)]
    pub fn is_primitive_procedure(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let tag = Exp::Symbol("primitive".to_string());
        let args = scheme_list!(exp, tag);
        is_tagged_list(&args)
    }

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
    pub fn is_larger_than(args: &Exp) -> Exp {
        let lhs = car(args).unwrap();
        let rhs = cadr(args).unwrap();
        match lhs {
            Exp::Integer(x) => match rhs {
                Exp::Integer(y) => {
                    if x > y {
                        Exp::Bool(true)
                    } else {
                        Exp::Bool(false)
                    }
                }
                Exp::FloatNumber(y) => {
                    let t = x as f32;
                    if t > y {
                        Exp::Bool(true)
                    } else {
                        Exp::Bool(false)
                    }
                }
                _ => panic!("type mismatch for comparision!"),
            },
            Exp::FloatNumber(x) => match rhs {
                Exp::Integer(y) => {
                    let t = y as f32;
                    if x > t {
                        Exp::Bool(true)
                    } else {
                        Exp::Bool(false)
                    }
                }
                Exp::FloatNumber(y) => {
                    if x > y {
                        Exp::Bool(true)
                    } else {
                        Exp::Bool(false)
                    }
                }
                _ => panic!("type mismatch for comparision!"),
            },
            _ => panic!("type mismatch for comparision"),
        }
    }

    #[allow(dead_code)]
    pub fn is_smaller_than(args: &Exp) -> Exp {
        let r = is_eq(args);
        match r {
            Exp::Bool(false) => {
                let r = is_larger_than(args);
                match r {
                    Exp::Bool(false) => Exp::Bool(true),
                    _ => Exp::Bool(false),
                }
            }
            _ => Exp::Bool(false),
        }
    }

    #[allow(dead_code)]
    pub fn is_tagged_list(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        if !exp.is_pair() {
            return Exp::Bool(false);
        }
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

    #[allow(dead_code)]
    pub fn is_assignment(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let args = scheme_list!(exp, Exp::Symbol("set!".to_string()));
        is_tagged_list(&args)
    }

    #[allow(dead_code)]
    pub fn is_definition(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let args = scheme_list!(exp, Exp::Symbol("define".to_string()));
        is_tagged_list(&args)
    }

    #[allow(dead_code)]
    pub fn is_if(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let args = scheme_list!(exp, Exp::Symbol("if".to_string()));
        is_tagged_list(&args)
    }

    #[allow(dead_code)]
    pub fn is_lambda(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let args = scheme_list!(exp, Exp::Symbol("lambda".to_string()));
        is_tagged_list(&args)
    }

    #[allow(dead_code)]
    pub fn is_begin(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let args = scheme_list!(exp, Exp::Symbol("begin".to_string()));
        is_tagged_list(&args)
    }

    #[allow(dead_code)]
    pub fn is_application(args: &Exp) -> Exp {
        let exp = car(args).unwrap();
        let r = exp.is_pair();
        match r {
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
        // println!("{:?}", exp.clone());
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
    // or lookup var-val pair in environment
    #[allow(dead_code)]
    pub fn extend_environment(args: &Exp) -> Exp {
        // args = vars+vals+base_env
        let vars = car(args).unwrap();
        let vals = cadr(args).unwrap();
        let base_env = caddr(args).unwrap();
        if list_length(&vars) == list_length(&vals) {
            let env = scheme_cons(make_frame(vars, vals), base_env);
            env
        } else {
            panic!("Error: NUMBER of args mismatch!")
        }
    }

    #[allow(dead_code)]
    pub fn lookup_variable_value(args: &Exp) -> Exp {
        let var = car(args).unwrap();
        let env = cadr(args).unwrap();
        let empty_env = Exp::List(Pair::Nil);
        // println!("env=> {}", exp_to_str(env.clone()));
        if env == empty_env {
            panic!("Error: unbound variable {}", exp_to_str(var));
        } else {
            let frame = first_frame(&env);
            let s = scan(&frame_variables(&frame), &frame_values(&frame), var.clone());
            match s {
                Some(x) => x,
                None => {
                    let enclosing_environment = enclosing_environment(&env);
                    let args = scheme_list!(var, enclosing_environment);
                    lookup_variable_value(&args)
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn enclosing_environment(env: &Exp) -> Exp {
        cdr(env).unwrap()
    }

    #[allow(dead_code)]
    fn scan(vars: &Exp, vals: &Exp, target: Exp) -> Option<Exp> {
        let null = Exp::List(Pair::Nil);
        if *vars == null {
            None
        } else if target == car(vars).unwrap() {
            Some(car(vals).unwrap())
        } else {
            scan(&cdr(vars).unwrap(), &cdr(vals).unwrap(), target)
        }
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

    #[allow(dead_code)]
    pub fn set_variable_value(args: &Exp) -> Exp {
        let var = car(args).unwrap();
        let val = cadr(args).unwrap();
        let env = caddr(args).unwrap();
        println!("env in set=> {}", exp_to_str(env.clone()));
        if env == Exp::List(Pair::Nil) {
            panic!("unbound variable: SET!");
        } else {
            let mut tag = false;
            let frame = first_frame(&env);
            let s = scan_and_set(
                frame_variables(&frame),
                frame_values(&frame),
                var.clone(),
                val.clone(),
                &mut tag,
            );
            if tag {
                let temp_frame = set_cdr(frame, s).unwrap();
                set_car(env, temp_frame).unwrap()
            } else {
                let enclosing_env = enclosing_environment(&env);
                let args = scheme_list!(var, val, enclosing_env);
                let temp_env = set_variable_value(&args);
                set_cdr(env, temp_env).unwrap()
            }
        }
    }

    #[allow(dead_code)]
    pub fn scan_and_set(
        vars: Exp,
        vals: Exp,
        target_var: Exp,
        target_val: Exp,
        tag: &mut bool,
    ) -> Exp {
        let null = Exp::List(Pair::Nil);
        if vars == null {
            null
        } else if target_var == car(&vars).unwrap() {
            *tag = true;
            set_car(vals, target_val).unwrap()
        } else {
            let temp_vals = scan_and_set(
                cdr(&vars).unwrap(),
                cdr(&vals).unwrap(),
                target_var,
                target_val,
                tag,
            );
            set_cdr(vals, temp_vals).unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        append,
        primitives::primitives::{
            caadr, caar, cadddr, caddr, cadr, cdadr, cdar, cdddr, cddr, define_variable,
            is_assignment, is_definition, is_primitive_procedure, is_self_evaluating,
            is_tagged_list, lambda_body, lambda_parameters, lookup_variable_value,
            meta_apply_primitive_procedure, multiply,
        },
        scheme_cons, scheme_list, str_to_exp,
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
    fn is_assignment_works() {
        let args = str_to_exp("((set! x 4))".to_string());
        assert_eq!(is_assignment(&args), Exp::Bool(true));
    }

    #[test]
    fn is_definition_works() {
        let args = str_to_exp("((define x 1))".to_string());
        assert_eq!(is_definition(&args), Exp::Bool(true));
    }

    #[test]
    fn is_primitive_procedure_works() {
        let args = str_to_exp("((primitive cons))".to_string());
        assert_eq!(is_primitive_procedure(&args), Exp::Bool(true));
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
    fn lookup_variable_value_works() {
        let env = str_to_exp("(((a b c) 1 2 3) ((x y z) 4 5 6))".to_string());
        let mut args = scheme_list!(Exp::Symbol("a".to_string()), env.clone());
        assert_eq!(lookup_variable_value(&args), Exp::Integer(1));
        args = scheme_list!(Exp::Symbol("y".to_string()), env);
        assert_eq!(lookup_variable_value(&args), Exp::Integer(5));
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

    #[test]
    fn lambda_parameters_works() {
        let args = str_to_exp("((lambad (x) (* x x)))".to_string());
        assert_eq!(
            lambda_parameters(&args),
            scheme_list!(Exp::Symbol("x".to_string()))
        );
    }

    #[test]
    fn lambda_body_works() {
        let args = str_to_exp("((lambad (x) (* x x)))".to_string());
        assert_eq!(lambda_body(&args), str_to_exp("((* x x))".to_string()));
    }

    #[test]
    fn meta_apply_primitive_procedure_works() {
        let mut args = str_to_exp("((primitive +) (1  2.1))".to_string());
        assert_eq!(meta_apply_primitive_procedure(&args), Exp::FloatNumber(3.1));
        args = str_to_exp("((primitive +) (1  2))".to_string());
        assert_eq!(meta_apply_primitive_procedure(&args), Exp::Integer(3));
        args = str_to_exp("((primitive -) (3.14  2))".to_string());
        assert_eq!(
            meta_apply_primitive_procedure(&args),
            Exp::FloatNumber(1.1400001)
        );
        args = str_to_exp("((primitive - ) (3  2))".to_string());
        assert_eq!(meta_apply_primitive_procedure(&args), Exp::Integer(1));
        args = str_to_exp("((primitive *) (3  2))".to_string());
        assert_eq!(meta_apply_primitive_procedure(&args), Exp::Integer(6));
        args = str_to_exp("((primitive *) (3  2.1))".to_string());
        assert_eq!(
            meta_apply_primitive_procedure(&args),
            Exp::FloatNumber(6.2999997)
        );
        args = str_to_exp("((primitive /) (3  2))".to_string());
        assert_eq!(meta_apply_primitive_procedure(&args), Exp::FloatNumber(1.5));
        args = str_to_exp("((primitive cons) (1  ()))".to_string());
        assert_eq!(
            meta_apply_primitive_procedure(&args),
            str_to_exp("(1)".to_string())
        );
        args = str_to_exp("((primitive car) ((1 2 3)))".to_string());
        assert_eq!(
            meta_apply_primitive_procedure(&args),
            str_to_exp("1".to_string())
        );
        args = str_to_exp("((primitive cdr) ((1 2 3)))".to_string());
        assert_eq!(
            meta_apply_primitive_procedure(&args),
            str_to_exp("(2 3)".to_string())
        );
    }
}
