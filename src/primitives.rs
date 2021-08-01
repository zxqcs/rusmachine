pub mod primitives {
    use crate::{
        parserfordev::parser::exp_to_str,
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

    // This primitive procedure shall be implemented in another way, it is too slow!
    #[allow(dead_code)]
    pub fn multiply(exp: &Exp) -> Exp {
        let lhs = exp_to_str(car(exp).unwrap());
        let rhs = exp_to_str(cadr(exp).unwrap());
        let operand_x = lhs.parse::<f32>().unwrap();
        let operand_y = rhs.parse::<f32>().unwrap();
        Exp::FloatNumber(operand_x * operand_y)
    }

    #[allow(dead_code)]
    pub fn eq(exp: &Exp) -> Exp {
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
        match exp {
            Exp::SchemeString(_x) => Exp::Bool(true),
            Exp::Integer(_x) => Exp::Bool(true),
            Exp::FloatNumber(_x) => Exp::Bool(true),
            _ => Exp::Bool(false),
        }
    }

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
    fn add_binding_to_frame(var: Exp, val: Exp, frame: Exp) -> Exp {
        let temp = set_car(frame.clone(), scheme_cons(var, frame_variables(&frame))).unwrap();
        set_cdr(temp, scheme_cons(val, frame_values(&frame))).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::result;

    use crate::{
        append,
        parserfordev::parser::str_to_exp,
        primitives::primitives::{
            caadr, caar, cadddr, caddr, cadr, cdadr, cdar, cdddr, cddr, define_variable,
            is_tagged_list, multiply,
        },
        scheme_cons, scheme_list,
        tpfordev::type_system::Exp,
        Pair,
    };

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
    fn define_variable_works() {
        let mut var = Exp::Symbol("x".to_string());
        let mut val = Exp::FloatNumber(3.14);
        let mut env = Exp::List(Pair::Nil);
        let mut args = scheme_list!(var.clone(), val.clone(), env);
        env = define_variable(&args);
        let mut checkout = scheme_list!(scheme_list!(scheme_list!(var), val));
        assert_eq!(env, checkout);
        var = Exp::Symbol("y".to_string());
        val = Exp::Integer(3);
        args = scheme_list!(var, val, env);
        env = define_variable(&args);
        checkout = scheme_list!(scheme_list!(
            scheme_list!(Exp::Symbol("x".to_string()), Exp::Symbol("y".to_string())),
            Exp::FloatNumber(3.14),
            Exp::Integer(3)
        ));
        assert_eq!(env, checkout);
    }
}
