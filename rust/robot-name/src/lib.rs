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
    const LETTER_COUNT: usize = 2;
    const NUMBER_COUNT: usize = 3;

    let mut rng = rand::thread_rng();

    let letters: String = (0..LETTER_COUNT).map(|_| rng.gen_range('A'..='Z')).collect();
    let numbers: String = (0..NUMBER_COUNT).map(|_| rng.gen_range('0'..='9')).collect();

    [letters, numbers].concat()
}
