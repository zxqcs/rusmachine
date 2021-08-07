pub mod parser {
    use crate::parser::parser::{
        read_scheme_quote, read_scheme_string, reverse, syntax_checker, tokenizer,
    };
    use crate::scheme_list;
    use crate::tpfordev::type_system::{append, car, scheme_cons, scheme_for_each, Exp, Pair};

    #[allow(dead_code)]
    pub fn build_syntax_tree(tokens: &mut Vec<String>) -> Exp {
        let mut tokens = reverse(tokens);
        let tree = build_syntax_tree_helper(&mut tokens);
        if tree != Exp::List(Pair::Nil) {
            car(&tree).unwrap()
        } else {
            Exp::List(Pair::Nil)
        }
    }

    #[allow(dead_code)]
    fn build_syntax_tree_helper(tokens: &mut Vec<String>) -> Exp {
        let mut tree_buffer = Exp::List(Pair::Nil);
        while let Some(t) = tokens.pop() {
            let token = t;
            match token {
                // head of a Exp::List
                x if x == "(".to_string() => {
                    let subtree = build_syntax_tree_helper(tokens);
                    tree_buffer = append(tree_buffer, scheme_list!(subtree));
                }
                // tail of a Exp::List
                x if x == ")".to_string() => {
                    break;
                }
                x if x == "Nil".to_string() => {
                    tree_buffer = append(tree_buffer, Exp::List(Pair::Nil));
                }
                // bool value
                x if x == "true".to_string() => {
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::Bool(true)));
                }
                x if x == "false".to_string() => {
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::Bool(false)));
                }
                // symbol value
                x if is_symbol(&x) => {
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::Symbol(x)));
                }
                // scheme string, for example, "winter is coming!"
                x if x.chars().nth(0) == Some('"') => {
                    let s = read_scheme_string(x, tokens);
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::SchemeString(s)));
                }
                // scheme quote, for example, 'winter
                x if x.chars().nth(0) == Some('\'') => {
                    let s = read_scheme_quote(x, tokens);
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::Quote(s)));
                }
                // i32
                x if is_i32(x.clone()) => {
                    tree_buffer = append(
                        tree_buffer,
                        scheme_list!(Exp::Integer(x.parse::<i32>().unwrap())),
                    );
                }
                // f32
                x if is_f32(x.clone()) => {
                    tree_buffer = append(
                        tree_buffer,
                        scheme_list!(Exp::FloatNumber(x.parse::<f32>().unwrap())),
                    );
                }
                _ => {
                    panic!("unknow token!");
                }
            }
        }
        tree_buffer
    }

    fn is_symbol(x: &String) -> bool {
        x.chars().nth(0).unwrap().is_alphabetic()
            || x == "="
            || x == "+"
            || x == "-"
            || x == "*"
            || x == "/"
            || x == ">"
            || x == "<"
    }

    fn is_f32(x: String) -> bool {
        let s = x.parse::<f32>();
        match s {
            Ok(_x) => true,
            _ => false,
        }
    }

    fn is_i32(x: String) -> bool {
        let s = x.parse::<i32>();
        match s {
            Ok(_x) => true,
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn print(exp: Exp) {
        match exp {
            Exp::FloatNumber(x) => print!("{}", x),
            Exp::Integer(x) => print!("{}", x),
            Exp::Symbol(x) => print!("{}", x),
            Exp::Quote(x) => {
                print!("{}", &x[1..x.len()]);
            }
            Exp::SchemeString(x) => print!("{}", x),
            Exp::Index(x) => print!("{}", x),
            Exp::Bool(x) => print!("{}", x),
            Exp::List(Pair::Nil) => {
                print!("()");
            }
            Exp::List(Pair::Cons(x, y)) => {
                print!("(");
                print(*x);
                if *y != Pair::Nil {
                    print!(" ");
                }
                let mut temp = y;
                while let Pair::Cons(lhs, rhs) = *temp {
                    print(*lhs);
                    if *rhs == Pair::Nil {
                        break;
                    }
                    print!(" ");
                    temp = rhs;
                }
                print!(")");
            }
        }
    }

    #[allow(dead_code)]
    pub fn exp_to_str(exp: Exp) -> String {
        let mut s = "".to_string();
        exp_to_str_iter(&mut s, exp);
        s = (&s).trim().to_string();
        s
    }

    #[allow(dead_code)]
    pub fn exp_to_str_iter(s: &mut String, exp: Exp) {
        match exp {
            Exp::FloatNumber(x) => {
                s.push(' ');
                s.push_str(&x.to_string());
            }
            Exp::Integer(x) => {
                s.push(' ');
                s.push_str(&x.to_string());
            }
            Exp::Symbol(x) => {
                s.push(' ');
                s.push_str(&x.to_string());
            }
            Exp::Index(x) => {
                s.push(' ');
                s.push_str(&x.to_string());
            }
            Exp::Quote(x) => {
                s.push(' ');
                s.push('\'');
                s.push_str(&x.to_string());
            }
            Exp::SchemeString(x) => {
                s.push(' ');
                s.push('"');
                s.push_str(&x.to_string());
                s.push('"');
            }
            Exp::Bool(x) => {
                s.push(' ');
                s.push_str(&x.to_string());
            }
            Exp::List(Pair::Nil) => {
                s.push_str(&"()".to_string());
            }
            Exp::List(Pair::Cons(x, y)) => {
                s.push_str(&"(".to_string());
                exp_to_str_iter(s, *x);
                /*
                if *y != Pair::Nil {
                    s.push(' ');
                }
                */
                let mut temp = y;
                while let Pair::Cons(lhs, rhs) = *temp {
                    exp_to_str_iter(s, *lhs);
                    if *rhs == Pair::Nil {
                        break;
                    }
                    s.push(' ');
                    temp = rhs;
                }
                s.push(')');
            }
        }
    }

    #[allow(dead_code)]
    pub fn str_to_exp(s: String) -> Exp {
        let mut tokens = tokenizer(s);
        if !syntax_checker(&tokens) {
            panic!("syntax wrong!");
        }
        let exp = build_syntax_tree(&mut tokens);
        exp
    }

    #[allow(dead_code)]
    pub fn scheme_list_pretty_print(items: &Exp) {
        let pretty_print = |x: Exp| {
            let line = exp_to_str(x);
            print!("    ");
            println!("{}", line);
        };
        println!("(");
        scheme_for_each(pretty_print, items);
        println!(")");
    }
}

#[cfg(test)]
mod test {
    use super::parser::{exp_to_str, str_to_exp};
    use crate::{
        scheme_list,
        tpfordev::type_system::{append, scheme_cons, Exp, Pair},
    };

    #[test]
    fn str_to_exp_works() {
        let s1 = "true";
        let s2 = "3.14";
        let s3 = "(( 1  2 )
        (3 
            (4  
               5)))";
        let s4 = "(define x \"winter is coming\")";
        let s5 = "'( 1 ( 2 3))";
        let s6 = "()";
        let exp1 = str_to_exp(s1.to_string());
        let exp2 = str_to_exp(s2.to_string());
        let exp3 = str_to_exp(s3.to_string());
        let exp4 = str_to_exp(s4.to_string());
        let exp5 = str_to_exp(s5.to_string());
        let exp6 = str_to_exp(s6.to_string());
        assert_eq!(exp1, Exp::Bool(true));
        assert_eq!(exp2, Exp::FloatNumber(3.14));
        assert_eq!(
            exp3,
            scheme_list!(
                scheme_list!(Exp::Integer(1), Exp::Integer(2)),
                scheme_list!(
                    Exp::Integer(3),
                    scheme_list!(Exp::Integer(4), Exp::Integer(5))
                )
            )
        );
        assert_eq!(
            exp4,
            scheme_list!(
                Exp::Symbol("define".to_string()),
                Exp::Symbol("x".to_string()),
                Exp::SchemeString("winter is coming".to_string())
            )
        );
        assert_eq!(exp5, Exp::Quote("( 1 ( 2 3))".to_string()));
        assert_eq!(exp6, Exp::List(Pair::Nil));
    }

    #[test]
    fn exp_to_str_works() {
        let s1 = "true";
        let s2 = "3.14";
        let s3 = "(( 1  2 )
        (3 
            (4  
               5)))";
        let s4 = "(define x \"winter is coming\")";
        let s5 = "'( 1 ( 2 3))";
        let s6 = "";
        let exp1 = str_to_exp(s1.to_string());
        let exp2 = str_to_exp(s2.to_string());
        let exp3 = str_to_exp(s3.to_string());
        let exp4 = str_to_exp(s4.to_string());
        let exp5 = str_to_exp(s5.to_string());
        let exp6 = str_to_exp(s6.to_string());
        let ss1 = exp_to_str(exp1);
        let ss2 = exp_to_str(exp2);
        let ss3 = exp_to_str(exp3);
        let ss4 = exp_to_str(exp4);
        let ss5 = exp_to_str(exp5);
        let ss6 = exp_to_str(exp6);
        assert_eq!(ss1, "true".to_string());
        assert_eq!(ss2, "3.14".to_string());
        assert_eq!(ss3, "(( 1 2)( 3( 4 5)))".to_string());
        assert_eq!(ss4, "( define x  \"winter is coming\")".to_string());
        assert_eq!(ss5, "'( 1 ( 2 3))".to_string());
        assert_eq!(ss6, "".to_string());
    }
}
