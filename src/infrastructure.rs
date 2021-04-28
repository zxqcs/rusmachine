pub mod register {
    pub struct Register {
        pub contents: &'static str,
    }

    impl Register {
        pub fn get(&self) -> &'static str {
            self.contents
        }

        pub fn set(&mut self, val: &'static str) {
            self.contents = val;
        }
    }
}

#[cfg(test)]
mod test {
    use super::register::Register;

    #[test]
    fn register_get_works() {
        let r = Register {
            contents: "unsigned",
        };
        assert_eq!("unsigned", r.get());
    }

    #[test]
    fn register_set_works() {
        let mut r = Register {
            contents: "unsigned",
        };
        r.set("apple");
        assert_eq!("apple", r.get());
    }
}