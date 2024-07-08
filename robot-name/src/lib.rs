use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {
    pub static NAME_SET: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::generate_unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::generate_unique_name();
    }

    fn generate_unique_name() -> String {
        let mut name: String = "".to_string();
        NAME_SET.with(|name_set| loop {
            let mut rng = rand::thread_rng();
            let letters: String = (0..2).map(|_| rng.gen_range(b'A'..=b'Z') as char).collect();
            let numbers: String = (0..3).map(|_| rng.gen_range(b'0'..=b'9') as char).collect();
            name = format!("{}{}", letters, numbers);

            if !name_set.borrow().contains(&name) {
                name_set.borrow_mut().insert(name.clone());
                break;
            }
        });

        name
    }
}
