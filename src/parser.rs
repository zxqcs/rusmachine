pub mod parser {
    use std::ascii::AsciiExt;

    use crate::representation::type_system::Object;

    pub fn old_tokenize(p: &mut Vec<String>) -> Vec<String> {
        let mut ss: Vec<String> = p.into_iter().map(|x| x.replace("(", " ( ")).collect();
        ss = ss.into_iter().map(|x| x.replace(")", " ) ")).collect();
        let mut tokens: Vec<String> = vec![];
        for item in ss.iter() {
            let mut v = item
                .trim()
                .split_whitespace()
                .collect::<Vec<_>>()
                .into_iter()
                .map(|x| x.to_string())
                .collect();
            tokens.append(&mut v);
        }
        tokens
    }

    pub fn tokenizer(s: &'static str) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];
        tokens.push("hello".to_string());
        tokens
    }
}