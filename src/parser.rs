pub mod parser {
    use std::usize;

    use crate::{
        machine::basic_machine::BasicMachine, memory::memory::Memory,
        representation::type_system::Object,
    };

    pub struct PairStack {
        capacity: usize,
        container: Vec<usize>,
    }

    impl PairStack {
        pub fn new() -> Self {
            PairStack {
                capacity: 100,
                container: Vec::new(),
            }
        }

        pub fn push(&mut self, index: usize) {
            if self.container.len() < self.capacity {
                self.container.push(index);
            } else {
                panic!("Maximum depth violated!");
            }
        }

        pub fn pop(&mut self) -> Option<usize> {
            self.container.pop()
        }

        pub fn peek(&self) -> Option<usize> {
            if self.container.len() > 0 {
                Some(self.container[self.container.len() - 1])
            } else {
                None
            }
        }
    }

    pub fn tokenizer(s: String) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];
        let ss = s.replace("(", " ( ");
        let ss = ss.replace(")", " ) ");
        let v: Vec<&str> = ss.trim().split_whitespace().collect();
        for item in v {
            tokens.push(item.to_string());
        }
        tokens
    }

    #[allow(dead_code)]
    pub fn reverse(s: &mut Vec<String>) -> Vec<String> {
        let mut x = vec![];
        while let Some(token) = s.pop() {
            x.push(token);
        }
        x
    }

    #[allow(dead_code)]
    pub fn syntax_checker(t: &Vec<String>) -> bool {
        let mut iterator = t.iter();
        let mut left_parenthesis = 0;
        let mut right_parenthesis = 0;
        let mut token = iterator.next();
        loop {
            match token {
                x if x == Some(&("(".to_string())) => {
                    left_parenthesis = left_parenthesis + 1;
                }
                x if x == Some(&(")".to_string())) => {
                    right_parenthesis = right_parenthesis + 1;
                }
                Some(_x) => {}
                None => {
                    break;
                }
            }
            token = iterator.next();
        }
        if left_parenthesis == right_parenthesis {
            true
        } else {
            false
        }
    }

    // writing a list into memory and return a index to the root of this list object
    #[allow(dead_code)]
    pub fn build_syntax_tree_into_memeory(
        tokens: &mut Vec<String>,
        memory: &mut Memory,
        machine: &mut BasicMachine,
    ) -> usize {
        if !syntax_checker(&tokens) {
            panic!("syntax wrong!");
        }
        let mut tokens = reverse(tokens);
        machine.set_register_contents(&"free".to_string(), Object::Index(0));
        let free = machine.get_register(&"free".to_string()).unwrap();
        let mut stack = PairStack::new();
        let root = free.get_memory_index();

        build_syntax_tree_into_memory_helper(&mut tokens, &mut stack, memory, machine);
        // root is the beginining index of the list written into memory
        root
    }

    #[allow(dead_code)]
    fn build_syntax_tree_into_memory_helper(
        tokens: &mut Vec<String>,
        stack: &mut PairStack,
        memory: &mut Memory,
        machine: &mut BasicMachine,
    ) {
        // if flag is set to true, the item should be written into car, otherwise written to cdr
        let mut flag = true;

        while let Some(t) = tokens.pop() {
            let token = t;
            let free = machine.get_register(&"free".to_string()).unwrap();
            // free_index indicates the first index of memory space that is not used
            let free_index = free.get_memory_index();
            // pair_index indicates the current memory index that is being written
            let pair_index = stack.peek();

            match token {
                // begin of a list
                x if x == "(".to_string() => {
                    match pair_index {
                        Some(i) => {
                            let item = Object::Pair(free_index);
                            if flag {
                                memory.update("car", item, i);
                                stack.push(free_index);
                                machine.register_increment_by_one(&"free".to_string());
                                let next_token = tokens.last();
                                let null = ")".to_string();
                                match next_token {
                                    t if t == Some(&null) => flag = false,
                                    _ => flag = true,
                                }
                            } else {
                                memory.update("cdr", item, i);
                                stack.pop();
                                // push the new pair index into stack
                                stack.push(free_index);
                                let pair_index = free_index;
                                machine.register_increment_by_one(&"free".to_string());
                                let free_index = machine
                                    .get_register(&"free".to_string())
                                    .unwrap()
                                    .get_memory_index();
                                let item = Object::Pair(free_index);

                                memory.update("car", item, pair_index);
                                let next_token = tokens.last();
                                let null = ")".to_string();
                                match next_token {
                                    t if t == Some(&null) => flag = false,
                                    _ => {
                                        flag = true;
                                        stack.push(
                                            machine
                                                .get_register(&"free".to_string())
                                                .unwrap()
                                                .get_memory_index(),
                                        );
                                        machine.register_increment_by_one(&"free".to_string());
                                    }
                                }
                            }
                        }
                        None => {
                            stack.push(free_index);
                            // note that the free indicator is always ahead of pair_index
                            machine.register_increment_by_one(&"free".to_string());
                        }
                    }
                }
                // tail of a List
                x if x == ")".to_string() => {
                    // find the right index for the Nil by pop PairStack, which is the
                    // current pair being written into memory
                    let null = Object::Nil;
                    match pair_index {
                        Some(i) => {
                            if !flag {
                                memory.update("cdr", null, i);
                                // pop current pair_index since both car and cdr part
                                // has been filled
                                stack.pop();
                                let next_token = tokens.last();
                                match next_token {
                                    Some(t) => {
                                        if t == ")" {
                                            continue;
                                        } else {
                                            // after the current pair_index has been poped
                                            // another issue arise, which is,
                                            // since current list has a cdr part(next_token exists!)
                                            //,there's a blank
                                            // position in the front of memory that need to be filled
                                            // and this position can be obtained by peek stack
                                            // (since position yet to be filled must be in
                                            //  stack at moment)
                                            let pair_index = stack.peek();
                                            match pair_index {
                                                Some(i) => {
                                                    let pair_item = Object::Pair(free_index);
                                                    memory.update("cdr", pair_item, i);
                                                    stack.pop();
                                                }
                                                None => {}
                                            }
                                            stack.push(free_index);
                                            machine.register_increment_by_one(&"free".to_string());
                                            flag = true;
                                        }
                                    }
                                    None => {
                                        break;
                                    }
                                }
                            } else {
                                panic!("Something wrong happened in writing into memory!");
                            }
                        }
                        None => {
                            panic!("Error in PairStack: Not Match!");
                        }
                    }
                }

                x if (x == "true"
                    || x == "false"
                    || is_i32(x.clone())
                    || x == "\"".to_string()
                    || x.chars().nth(0) == Some('\'')
                    || is_f32(x.clone())
                    || is_symbol(&x)) =>
                {
                    let mut item = Object::Nil;
                    match x {
                        x if x == "true" => item = Object::Bool(true),
                        x if x == "false" => item = Object::Bool(false),
                        x if is_i32(x.clone()) => {
                            item = Object::Integer(x.parse::<i32>().unwrap());
                        }
                        x if is_f32(x.clone()) => {
                            item = Object::Number(x.parse::<f32>().unwrap());
                        }
                        x if is_symbol(&x) => item = Object::Symbol(x),
                        x if x.chars().nth(0) == Some('"') => {
                            let s = read_scheme_string(x, tokens);
                            item = Object::LispString(s);
                        }
                        x if x.chars().nth(0) == Some('\'') => {
                            let s = read_scheme_quote(x, tokens);
                            item = Object::Quote(s);
                        }
                        _ => {}
                    }
                    match pair_index {
                        Some(i) => {
                            if flag {
                                memory.update("car", item, i);
                                flag = false;
                            } else {
                                let pair_item = Object::Pair(free_index);
                                memory.update("cdr", pair_item, i);
                                stack.pop();
                                stack.push(free_index);
                                let pair_index = free_index;
                                machine.register_increment_by_one(&"free".to_string());
                                memory.update("car", item, pair_index);
                                flag = false;
                            }
                        }
                        None => {
                            panic!("Something wrong happened in writing into memory!");
                        }
                    }
                }
                _ => {
                    panic!("unknow token!");
                }
            }
        }
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

    pub fn read_scheme_string(t: String, tokens: &mut Vec<String>) -> String {
        let mut tt = (&t[1..]).to_string();
        loop {
            let s = tokens.pop();
            match s {
                Some(x) => {
                    if is_end_with_double_quote(&x) {
                        tt.push(' ');
                        let tx = &x[..(x.len() - 1)];
                        tt.push_str(tx);
                        return tt;
                    } else {
                        tt.push(' ');
                        tt.push_str(&x);
                    }
                }
                None => {
                    panic!("missing part for a Scheme String!");
                }
            }
        }
    }

    pub fn is_end_with_double_quote(s: &str) -> bool {
        s.chars().last().unwrap() == '\"'
    }

    pub fn read_scheme_quote(t: String, tokens: &mut Vec<String>) -> String {
        if t.len() == 1 {
            let first_token = tokens.pop().unwrap();
            let mut left = 0;
            let mut right = 0;
            if first_token != "(" {
                panic!("syntax wrong!");
            } else {
                let mut s = "".to_string();
                s.push('(');
                left = left + 1;
                loop {
                    let token = tokens.pop();
                    match token {
                        Some(x) => {
                            if x == "(" {
                                left = left + 1;
                                s.push(' ');
                                s.push('(');
                                if left == right {
                                    return s;
                                }
                            } else if x == ")" {
                                right = right + 1;
                                s.push(')');
                                if left == right {
                                    return s;
                                }
                            } else {
                                s.push(' ');
                                s.push_str(&x);
                            }
                        }
                        None => {
                            panic!("syntax wrong!");
                        }
                    }
                }
            }
        } else {
            let s = (&t[1..]).to_string();
            s
        }
    }
}

#[cfg(test)]
mod test {
    use super::parser::{
        build_syntax_tree_into_memeory, is_end_with_double_quote, read_scheme_quote,
        read_scheme_string, reverse, tokenizer,
    };

    use crate::representation::type_system::Object;
    use crate::{machine::basic_machine::BasicMachine, memory::memory::Memory};

    #[test]
    fn tokenizer_works() {
        let s = "(define (fac n) 
                         (if (= n 1)
                              1
                             (* n 
                                (fac (- n 1)))))"
            .to_string();
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
                                  5)))"
            .to_string();
        let mut tokens = tokenizer(s);
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

        let car_0_checkout = Object::Pair(1);
        let cdr_0_checkout = Object::Pair(3);
        let car_1_checkout = Object::Integer(1);
        let cdr_1_checkout = Object::Pair(2);
        let car_2_checkout = Object::Integer(2);
        let cdr_2_checkout = Object::Nil;
        let car_3_checkout = Object::Pair(4);
        let cdr_3_checkout = Object::Nil;
        let car_4_checkout = Object::Integer(3);
        let cdr_4_checkout = Object::Pair(5);
        let car_5_checkout = Object::Pair(6);
        let cdr_5_checkout = Object::Nil;
        let car_6_checkout = Object::Integer(4);
        let cdr_6_checkout = Object::Pair(7);
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

    #[test]
    fn read_scheme_string_works() {
        let t = vec!["\"winter", "is", "coming\""];
        let mut tokens: Vec<String> = t.into_iter().map(|x| x.to_string()).collect();
        tokens = reverse(&mut tokens);
        let x = tokens.pop().unwrap();
        let s = read_scheme_string(x, &mut tokens);
        assert_eq!(s, "winter is coming".to_string());
    }

    #[test]
    fn read_scheme_quote_works() {
        let quote = "'(1 ( 2 3 ))".to_string();
        let mut tokens = reverse(&mut tokenizer(quote));
        let x = tokens.pop().unwrap();
        let s = read_scheme_quote(x, &mut tokens);
        assert_eq!(s, "( 1 ( 2 3))".to_string());
        let another_quote = "'symbol ( 1 2)".to_string();
        let mut ttokens = reverse(&mut tokenizer(another_quote));
        let xx = ttokens.pop().unwrap();
        let ss = read_scheme_quote(xx, &mut ttokens);
        assert_eq!(ss, "symbol".to_string());
    }

    #[test]
    fn is_end_with_double_quote_works() {
        let s = "coming\"";
        assert_eq!(is_end_with_double_quote(s), true);
    }
}
