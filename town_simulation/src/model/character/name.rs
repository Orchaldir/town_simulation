use std::fmt::{Display, Formatter};
use CharacterName::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CharacterName {
    Simple(String),
    Standard { first: String, last: String },
}

impl CharacterName {
    pub fn simple<S: Into<String>>(name: S) -> Self {
        Simple(name.into())
    }

    pub fn standard<S: Into<String>>(first: S, last: S) -> Self {
        Self::Standard {
            first: first.into(),
            last: last.into(),
        }
    }

    pub fn get_last(&self) -> Option<&str> {
        match self {
            Simple(..) => None,
            Standard { last, .. } => Some(last),
        }
    }

    pub fn update_last<S: Into<String>>(&self, last: S) -> Self {
        match self {
            Simple(name) => Standard {
                first: name.to_string(),
                last: last.into(),
            },
            Standard { first, .. } => Standard {
                first: first.to_string(),
                last: last.into(),
            },
        }
    }

    pub fn sorted(&self) -> String {
        match self {
            Simple(name) => name.clone(),
            Standard { first, last } => format!("{}, {}", last, first),
        }
    }
}

impl Display for CharacterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Simple(name) => write!(f, "{}", name),
            Standard { first, last } => write!(f, "{} {}", first, last),
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

    #[test]
    fn test_get_last() {
        let simple = CharacterName::simple("Test");
        let standard = CharacterName::standard("Aaa", "Bbb");

        assert_eq!(simple.get_last(), None);
        assert_eq!(standard.get_last(), Some("Bbb"));
    }
}
