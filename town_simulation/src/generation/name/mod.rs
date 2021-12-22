use derive_more::Constructor;

#[derive(Constructor)]
pub struct Entry {
    name: String,
    value: u32,
}

pub struct NameGenerator {
    entries: Vec<Entry>,
    total_value: u32,
}

impl NameGenerator {
    pub fn new(mut entries: Vec<Entry>) -> Self {
        let mut total_value = 0u32;

        for entry in entries.iter_mut() {
            total_value += entry.value;
            entry.value = total_value;
        }

        Self {
            entries,
            total_value,
        }
    }

    pub fn get(&self, index: u32) -> &str {
        let index = index % self.total_value;

        for entry in &self.entries {
            if index < entry.value {
                return &entry.name;
            }
        }

        "IMPOSSIBLE"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let generator = NameGenerator::new(vec![
            Entry::new("A".to_string(), 3),
            Entry::new("B".to_string(), 2),
        ]);

        assert_eq!(generator.get(0), "A");
        assert_eq!(generator.get(1), "A");
        assert_eq!(generator.get(2), "A");
        assert_eq!(generator.get(3), "B");
        assert_eq!(generator.get(4), "B");
        assert_eq!(generator.get(5), "A");
    }
}
