use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hasher;

#[derive(Clone, Debug, PartialEq)]
pub enum RandomNumberGenerator {
    Hash,
    Mock {
        values: HashMap<(u32, usize, u32), u64>,
        default: u64,
    },
}

impl RandomNumberGenerator {
    pub fn roll_d100(&self, year: u32, index: usize, usage: u32) -> u32 {
        (self.next(year, index, usage) % 100) as u32
    }

    fn next(&self, year: u32, index: usize, usage: u32) -> u64 {
        match self {
            RandomNumberGenerator::Hash => {
                let mut hasher = DefaultHasher::new();
                hasher.write_u32(year);
                hasher.write_usize(index);
                hasher.write_u32(usage);
                hasher.finish()
            }
            RandomNumberGenerator::Mock { values, default } => {
                *values.get(&(year, index, usage)).unwrap_or(default)
            }
        }
    }
}
