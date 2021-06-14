pub mod memory {
    use crate::representation::type_system::Object;
    pub struct Memory {
        pub the_cars: Vec<Box<Object>>,
        pub the_cdrs: Vec<Box<Object>>,
        pub new_cars: Vec<Box<Object>>,
        pub new_cdrs: Vec<Box<Object>>,
    }

    impl Memory {
        pub fn new(size: usize) -> Self {
            let mut v = Vec::with_capacity(size);

            for i in 0..size {
                v.push(Box::new(Object::Nil));
            }

            Memory {
                the_cars: v.clone(),
                the_cdrs: v.clone(),
                new_cars: v.clone(),
                new_cdrs: v.clone(),
            }
        }

        pub fn update(&mut self, message: &'static str, item: Object, index: usize) {
            let s = Box::new(item);
            match message {
                "the_cars" => {
                    self.the_cars[index] = s;
                }
                "the_cdrs" => {
                    self.the_cdrs[index] = s;
                }
                "new_cars" => {
                    self.new_cars[index] = s;
                }
                "new_cdrs" => {
                    self.new_cdrs[index] = s;
                }
                _ => {
                    panic!("Unknown Operations!");
                }
            }
        }

        pub fn flip(&mut self) {
            let mut temp = self.the_cars.clone();
            self.the_cars = self.new_cars.clone();
            self.new_cars = temp;
            temp = self.the_cdrs.clone();
            self.the_cdrs = self.new_cdrs.clone();
            self.new_cdrs = temp;
        }
    }
}
