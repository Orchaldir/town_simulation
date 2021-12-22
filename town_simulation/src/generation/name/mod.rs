pub struct NameGenerator {
    names: Vec<(String, u32)>,
    total_value: u32,
}

impl NameGenerator {
    pub fn new(mut names: Vec<(String, u32)>) -> Self {
        let mut total_value = 0u32;

        for (_name, value) in names.iter_mut() {
            total_value += *value;
            *value = total_value;
        }

        Self { names, total_value }
    }

    pub fn get(&self, index: u32) -> &str {
        let index = index % self.total_value;

        for (name, value) in &self.names {
            if index < *value {
                return name;
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
        let generator = NameGenerator::new(vec![("A".to_string(), 3), ("B".to_string(), 2)]);

        assert_eq!(generator.get(0), "A");
        assert_eq!(generator.get(1), "A");
        assert_eq!(generator.get(2), "A");
        assert_eq!(generator.get(3), "B");
        assert_eq!(generator.get(4), "B");
        assert_eq!(generator.get(5), "A");
    }
}
