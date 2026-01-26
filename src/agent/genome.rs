pub struct Genome {
    speed: u32,
    vision_range: u32,
    foraging_efficiency: f32,
}

impl Genome {
    pub fn random() -> Self {
        // Placeholder for random genome generation
        Genome {
            speed: 1,
            vision_range: 5,
            foraging_efficiency: 0.5,
        }
    }
}
