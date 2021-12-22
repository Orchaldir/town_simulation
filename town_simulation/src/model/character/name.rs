use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum CharacterName {
    Standard { first: String, last: String },
}

impl CharacterName {
    pub fn standard<S: Into<String>>(first: S, last: S) -> Self {
        Self::Standard {
            first: first.into(),
            last: last.into(),
        }
    }
}

impl Display for CharacterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterName::Standard { first, last } => write!(f, "{} {}", first, last),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let standard = CharacterName::standard("Aaa", "Bbb");

        assert_eq!(standard.to_string(), "Aaa Bbb");
    }
}