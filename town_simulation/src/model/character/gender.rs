use crate::model::character::gender::Gender::{Female, Male};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn reverse(&self) -> Gender {
        match self {
            Male => Female,
            Female => Male,
        }
    }

    pub fn is_reverse(&self, gender: Gender) -> bool {
        self.reverse() == gender
    }
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Male
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Male.reverse(), Female);
        assert_eq!(Female.reverse(), Male);
    }

    #[test]
    fn test_is_reverse() {
        assert!(Male.is_reverse(Female));
        assert!(!Male.is_reverse(Male));
        assert!(Female.is_reverse(Male));
        assert!(!Female.is_reverse(Female));
    }
}
