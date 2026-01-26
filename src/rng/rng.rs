use rand::Rng;
use rand::prelude::*;

pub struct RNG {
    rng: rand_chacha::ChaCha8Rng,
}

impl RNG {
    pub fn new(seed: u64) -> Self {
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        Self { rng }
    }

    pub fn choose<'a, T>(&mut self, choices: &'a [T]) -> &'a T {
        let index = self.rng.random_range(0..choices.len());
        &choices[index]
    }

    pub fn choose_range(&mut self, range: std::ops::Range<u32>) -> u32 {
        self.rng.random_range(range)
    }
}
