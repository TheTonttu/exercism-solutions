use once_cell::sync::Lazy;
use rand::Rng;
use std::collections::HashSet;
use std::sync::Mutex;

static NAME_REGISTRY: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::new()));

#[derive(Default)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: gen_unique_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        NAME_REGISTRY.lock().unwrap().remove(&self.name);
        self.name = gen_unique_name();
    }
}

fn gen_unique_name() -> String {
    let mut registry = NAME_REGISTRY.lock().unwrap();
    loop {
        let new_name = gen_random_name();
        if registry.insert(new_name.clone()) {
            return new_name;
        }
    }
}

fn gen_random_name() -> String {
    const LETTER_COUNT: usize = 2;
    const NUMBER_COUNT: usize = 3;

    let mut rng = rand::thread_rng();

    let letters: String = (0..LETTER_COUNT)
        .map(|_| rng.gen_range('A'..='Z'))
        .collect();
    let numbers: String = (0..NUMBER_COUNT)
        .map(|_| rng.gen_range('0'..='9'))
        .collect();

    [letters, numbers].concat()
}
