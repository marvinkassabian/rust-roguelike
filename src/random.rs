use rltk::RandomNumberGenerator;

use crate::console_log;

pub struct Random {
    rng: RandomNumberGenerator,
}

const DEBUG_SEED: Option<u64> = Some(6836765706277375599);

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
        if DEBUG_SEED.is_some() {
            RandomNumberGenerator::seeded(DEBUG_SEED.unwrap())
        } else {
            let random_seed = Random::get_random_seed();
            console_log(format!("random seed: {}", random_seed));

            RandomNumberGenerator::seeded(random_seed)
        }
    }

    fn get_random_seed() -> u64 {
        let mut rng = RandomNumberGenerator::new();

        rng.next_u64()
    }

    pub fn flip_coin(&mut self) -> bool {
        self.range(0, 2) == 1
    }

    pub fn roll_die(&mut self, die_type: i32) -> i32 {
        self.rng.roll_dice(1, die_type)
    }
}