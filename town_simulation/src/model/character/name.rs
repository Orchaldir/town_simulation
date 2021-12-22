use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum CharacterName {
    Simple(String),
    Standard { first: String, last: String },
}

impl CharacterName {
    pub fn simple<S: Into<String>>(name: S) -> Self {
        Self::Simple(name.into())
    }

    pub fn standard<S: Into<String>>(first: S, last: S) -> Self {
        Self::Standard {
            first: first.into(),
            last: last.into(),
        }
    }

    pub fn sorted(&self) -> String {
        match self {
            CharacterName::Simple(name) => name.clone(),
            CharacterName::Standard { first, last } => format!("{}, {}", last, first),
        }
    }
}

impl Display for CharacterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CharacterName::Simple(name) => write!(f, "{}", name),
            CharacterName::Standard { first, last } => write!(f, "{} {}", first, last),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let simple = CharacterName::simple("Test");
        let standard = CharacterName::standard("Aaa", "Bbb");

        assert_eq!(simple.to_string(), "Test");
        assert_eq!(standard.to_string(), "Aaa Bbb");
    }

    #[test]
    fn test_sorted() {
        let simple = CharacterName::simple("Test");
        let standard = CharacterName::standard("Aaa", "Bbb");

        assert_eq!(simple.sorted(), "Test");
        assert_eq!(standard.sorted(), "Bbb, Aaa");
    }
}
