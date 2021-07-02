pub mod parser {
    use crate::{
        machine::basic_machine::BasicMachine, memory::memory::Memory,
        representation::type_system::Object,
    };

    pub fn tokenizer(s: &'static str) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];
        let mut ss = s.replace("(", " ( ");
        let ss = ss.replace(")", " ) ");
        let v: Vec<&str> = ss.trim().split_whitespace().collect();
        for item in v {
            tokens.push(item.to_string());
        }
        tokens
    }

    #[allow(dead_code)]
    fn reverse(s: &mut Vec<String>) -> Vec<String> {
        let mut x = vec![];
        while let Some(token) = s.pop() {
            x.push(token);
        }
        x
    }
    #[allow(dead_code)]
    pub fn build_syntax_tree_into_memeory(
        tokens: &mut Vec<String>,
        memory: &mut Memory,
        machine: &mut BasicMachine,
    ) {
        let mut tokens = reverse(tokens);
        build_syntax_tree_into_memory_helper(&mut tokens, memory, machine);
    }
    #[allow(dead_code)]
    fn build_syntax_tree_into_memory_helper(
        tokens: &mut Vec<String>,
        memory: &mut Memory,
        machine: &mut BasicMachine,
    ) {
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
    }

    fn read_scheme_string(_tokens: &mut Vec<String>) {
        Exp::SchemeString("hello world!".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::parser::{build_syntax_tree_into_memeory, tokenizer};
    use crate::representation::type_system::{Object, Pair};
    use crate::{machine::basic_machine::BasicMachine, memory::memory::Memory};

    #[test]
    fn tokenizer_works() {
        let s = "(define (fac n) 
                         (if (= n 1)
                              1
                             (* n 
                                (fac (- n 1)))))";
        let tokens = tokenizer(s);
        let v = vec![
            "(", "define", "(", "fac", "n", ")", "(", "if", "(", "=", "n", "1", ")", "1", "(", "*",
            "n", "(", "fac", "(", "-", "n", "1", ")", ")", ")", ")", ")",
        ];
        assert_eq!(tokens, v);
    }

    #[test]
    fn build_syntax_tree_into_memeory_works() {
        let mut memory = Memory::new(10);
        let mut machine = BasicMachine::new();
        machine.initilize_registers();
        let s = "(( 1  2 )
                           (3 
                               (4  
                                  5)))";
        let tokens = tokenizer(s);
        build_syntax_tree_into_memeory(&mut tokens, &mut memory, &mut machine);
        let car_0 = memory.car(0);
        let cdr_0 = memory.cdr(0);

        let car_1 = memory.car(1);
        let cdr_1 = memory.cdr(1);

        let car_2 = memory.car(2);
        let cdr_2 = memory.cdr(2);

        let car_3 = memory.car(3);
        let cdr_3 = memory.cdr(3);

        let car_4 = memory.car(4);
        let cdr_4 = memory.cdr(4);

        let car_5 = memory.car(5);
        let cdr_5 = memory.cdr(5);

        let car_6 = memory.car(6);
        let cdr_6 = memory.cdr(6);

        let car_7 = memory.car(7);
        let cdr_7 = memory.cdr(7);

        let car_0_checkout = Object::Pair(Pair::new(1));
        let cdr_0_checkout = Object::Pair(Pair::new(3));
        let car_1_checkout = Object::Integer(1);
        let cdr_1_checkout = Object::Pair(Pair::new(2));
        let car_2_checkout = Object::Integer(2);
        let cdr_2_checkout = Object::Nil;
        let car_3_checkout = Object::Pair(Pair::new(4));
        let cdr_3_checkout = Object::Nil;
        let car_4_checkout = Object::Integer(3);
        let cdr_4_checkout = Object::Pair(Pair::new(5));
        let car_5_checkout = Object::Pair(Pair::new(6));
        let cdr_5_checkout = Object::Nil;
        let car_6_checkout = Object::Integer(4);
        let cdr_6_checkout = Object::Pair(Pair::new(7));
        let car_7_checkout = Object::Integer(5);
        let cdr_7_checkout = Object::Nil;

        assert_eq!(car_0, car_0_checkout);
        assert_eq!(cdr_0, cdr_0_checkout);
        assert_eq!(car_1, car_1_checkout);
        assert_eq!(cdr_1, cdr_1_checkout);
        assert_eq!(car_2, car_2_checkout);
        assert_eq!(cdr_2, cdr_2_checkout);
        assert_eq!(car_3, car_3_checkout);
        assert_eq!(cdr_3, cdr_3_checkout);
        assert_eq!(car_4, car_4_checkout);
        assert_eq!(cdr_4, cdr_4_checkout);
        assert_eq!(car_5, car_5_checkout);
        assert_eq!(cdr_5, cdr_5_checkout);
        assert_eq!(car_6, car_6_checkout);
        assert_eq!(cdr_6, cdr_6_checkout);
        assert_eq!(car_7, car_7_checkout);
        assert_eq!(cdr_7, cdr_7_checkout);
    }
}
