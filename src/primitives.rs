pub mod primitives {
    use crate::tpfordev::type_system::{car, cdr, Exp, Pair};
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

    #[allow(dead_code)]
    pub fn is_tagged_list(exp: &Exp, tag: &'static str) -> bool {
        if exp.is_pair() {
            if let Ok(Exp::Symbol(x)) = car(exp) {
                match x {
                    t if t == tag => true,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{parserfordev::parser::str_to_exp, primitives::primitives::{caadr, caar, cadddr, caddr, cadr, cdadr, cdar, cdddr, cddr, is_tagged_list}};

    #[test]
    fn cadr_works() {
        let mut items = str_to_exp("((1 2) 3 4 5)");
        assert_eq!(cadr(&items).unwrap(), str_to_exp("3"));
        items = str_to_exp("(assign continue (label fact-done))");
        assert_eq!(cadr(&items).unwrap(), str_to_exp("continue"));
    }

    #[test]
    fn cddr_works() {
        let mut items = str_to_exp("((1 2) (3 4 5) 6 7)");
        assert_eq!(cddr(&items).unwrap(), str_to_exp("(6 7)"));
        items = str_to_exp("(test (op =) (reg n) (const 1))");
        assert_eq!(cddr(&items).unwrap(), str_to_exp("((reg n) (const 1))"));
    }

    #[test]
    fn caar_works() {
        let items = str_to_exp("((1 2) 3 4 5)");
        assert_eq!(caar(&items).unwrap(), str_to_exp("1"));
    }

    #[test]
    fn cdar_works() {
        let mut items = str_to_exp("((1 2) (3 4 5) 6 7)");
        assert_eq!(cdar(&items).unwrap(), str_to_exp("(2)"));
        items = str_to_exp("((reg n) (const 1))");
        assert_eq!(cdar(&items).unwrap(), str_to_exp("(n)"));
    }

    #[test]
    fn cdddr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)");
        assert_eq!(cdddr(&items).unwrap(), str_to_exp("(7)"));
    }

    #[test]
    fn cadddr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)");
        assert_eq!(cadddr(&items).unwrap(), str_to_exp("7"));
    }

    #[test]
    fn caddr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)");
        assert_eq!(caddr(&items).unwrap(), str_to_exp("6"));
    }

    #[test]
    fn caadr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)");
        assert_eq!(caadr(&items).unwrap(), str_to_exp("3"));
    }

    #[test]
    fn cdadr_works() {
        let items = str_to_exp("((1 2) (3 4 5) 6 7)");
        assert_eq!(cdadr(&items).unwrap(), str_to_exp("(4 5)"));
    }

    #[test]
    fn is_tagged_list_works() {
        let mut items = str_to_exp("(reg continue)");
        assert_eq!(is_tagged_list(&items, "reg"), true);
        items = str_to_exp("(const 1)");
        assert_eq!(is_tagged_list(&items, "const"), true);
    }
}
