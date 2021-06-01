
pub mod type_system {
    use std::usize;

    #[derive(Debug, Clone, PartialEq)]
    pub enum  Objects{
        Nummber(f32),
        Integer(i32),
        Symbol(&'static str),
        Quote(&'static str),
        Bool(bool),
        Nil,
    }   

    #[derive(Debug, Clone, PartialEq)]
    pub struct ListObject {
        Item: Objects,
        Index: usize,
    }

    impl ListObject {
        pub fn new() -> Self {
            ListObject {
                Item: Objects::Nil,
                Index: 0,
            }
        }
    }
    pub struct Pairs {
        pub the_cars: Vec<Box<ListObject>>, 
        pub the_cdrs: Vec<Box<ListObject>>, 
        pub free: usize,
    }

    impl Pairs {
        pub fn new() -> Self {
            let size = 1000;
            let mut x: Vec<Box<ListObject>> = Vec::with_capacity(size);
            let mut item = ListObject {
                Item: Objects::Nil,
                Index: 0,
            };
            for i in 0..size {
                x.push(Box::new(item.clone()));
            }
            let mut y = x.clone();
            Pairs {
                the_cars: x,
                the_cdrs: y,
                free: 0,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::type_system::{Objects, ListObject, Pairs};

    #[test]
    fn pair_new_works() {
        let s = Pairs::new();
        let item = ListObject::new();
        assert_eq!(s.the_cdrs.len(), 1000);
        assert_eq!(Some(&Box::new(item)), s.the_cars.get(0)); 
    }
}