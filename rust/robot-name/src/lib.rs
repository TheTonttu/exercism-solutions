use rand::Rng;

#[derive(Default)]
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            name: gen_random_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = gen_random_name()
    }
}

fn gen_random_name() -> String {
    let mut rng = rand::thread_rng();

    let letters: String = (0..2).map(|_| rng.gen_range('A'..'Z')).collect();
    let numbers: String = (0..3).map(|_| rng.gen_range('0'..='9')).collect();

    [letters, numbers].concat()
}
