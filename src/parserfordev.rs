pub mod parser {
    use crate::parser::parser::{reverse, tokenizer};
    use crate::scheme_list;
    use crate::tpfordev::type_system::{append, car, scheme_cons, Exp, Pair};

    #[allow(dead_code)]
    pub fn build_syntax_tree(tokens: &mut Vec<String>) -> Exp {
        let mut tokens = reverse(tokens);
        let tree = build_syntax_tree_helper(&mut tokens);
        if tree != Exp::List(Pair::Nil) {
            car(tree).unwrap()
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
                x if x == "true" => {
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::Bool(true)));
                }
                x if x == "false" => {
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::Bool(false)));
                }
                // symbol value
                x if is_symbol(&x) => {
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::Symbol(x)));
                }
                // scheme string, for example, "winter is coming!"
                x if x == "\"".to_string() => {
                    let s = read_scheme_string(tokens);
                    tree_buffer = append(tree_buffer, scheme_list!(s));
                }
                // scheme quote, for example, 'winter
                x if x.chars().nth(0) == Some('\'') => {
                    tree_buffer = append(tree_buffer, scheme_list!(Exp::Quote(x)));
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

    fn read_scheme_string(_tokens: &mut Vec<String>) -> Exp {
        Exp::SchemeString("hello world!".to_string())
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
}
