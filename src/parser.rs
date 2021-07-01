pub mod parser {
    use crate::representation::type_system::Object;

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
}

#[cfg(test)]
mod test {
    use super::parser::tokenizer;

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
}
