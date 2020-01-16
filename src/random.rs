use rltk::RandomNumberGenerator;

pub struct Random {
    rng: RandomNumberGenerator,
}

const DEBUG: bool = false;

impl Random {
    pub fn new() -> Random {
        Random {
            rng: Random::get_rng()
        }
    }

    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        self.rng.range(min, max)
    }

    pub fn inclusive_range(&mut self, min: i32, max: i32) -> i32 {
        self.range(min, max + 1)
    }

    fn get_rng() -> RandomNumberGenerator {
        if DEBUG {
            RandomNumberGenerator::new()
        } else {
            RandomNumberGenerator::seeded(1)
        }
    }

    pub fn flip_coin(&mut self) -> bool {
        self.range(0, 2) == 1
    }
}