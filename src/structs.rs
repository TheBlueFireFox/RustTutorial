pub fn run() {
    let mut p = Person::new(2, "Peter".to_string());
    
    p.print_me();

    p.inc_age();

    p.print_me();
}

#[derive(Debug)]
struct Person {
    age: u8,
    alive: bool,
    name: String,
}

impl Person {
    fn new(age: u8, first_name: String) -> Self {
        Self {
            age, // syntax suggar for -> age: age,
            alive: true,
            name: first_name,
        }
    }

    fn adult(&self) -> bool {
        self.age >= 18
    }

    fn inc_age(&mut self) {
        self.age += 1;
    }

    fn print_me(&self) {
        println!("{:?}", self);
    }
}

// Generated for us by derive macro 'Debug'
// impl std::fmt::Debug for Person {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Person")
//             .field("age", &self.age)
//             .field("alive", &self.alive)
//             .field("name", &self.name)
//             .finish()
//     }
// }
