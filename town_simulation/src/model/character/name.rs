use std::fmt::{Display, Formatter};
use CharacterName::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CharacterName {
    Simple(String),
    Standard {
        first: String,
        last: String,
    },
    Married {
        first: String,
        last: String,
        birth: String,
    },
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

    pub fn married<S: Into<String>>(first: S, last: S, birth: S) -> Self {
        Self::Married {
            first: first.into(),
            last: last.into(),
            birth: birth.into(),
        }
    }

    pub fn get_last(&self) -> Option<&str> {
        match self {
            Simple(..) => None,
            Standard { last, .. } => Some(last),
            Married { last, .. } => Some(last),
        }
    }

    pub fn marry<S: Into<String>>(&self, new_last: S) -> Self {
        match self {
            Simple(..) => self.clone(),
            Standard { first, last } => Married {
                first: first.to_string(),
                last: new_last.into(),
                birth: last.to_string(),
            },
            Married { first, birth, .. } => Married {
                first: first.to_string(),
                last: new_last.into(),
                birth: birth.to_string(),
            },
        }
    }

    pub fn sorted(&self) -> String {
        match self {
            Simple(name) => name.clone(),
            Standard { first, last } => format!("{}, {}", last, first),
            Married { first, last, birth } => format!("{} nee {}, {}", last, birth, first),
        }
    }
}

impl Display for CharacterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Simple(name) => write!(f, "{}", name),
            Standard { first, last } => write!(f, "{} {}", first, last),
            Married { first, last, birth } => write!(f, "{} {} nee {}", first, last, birth),
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
        let married = CharacterName::married("Aaa", "Ccc", "Bbb");

        assert_eq!(simple.to_string(), "Test");
        assert_eq!(standard.to_string(), "Aaa Bbb");
        assert_eq!(married.to_string(), "Aaa Ccc nee Bbb");
    }

    #[test]
    fn test_sorted() {
        let simple = CharacterName::simple("Test");
        let standard = CharacterName::standard("Aaa", "Bbb");
        let married = CharacterName::married("Aaa", "Ccc", "Bbb");

        assert_eq!(simple.sorted(), "Test");
        assert_eq!(standard.sorted(), "Bbb, Aaa");
        assert_eq!(married.sorted(), "Ccc nee Bbb, Aaa");
    }

    #[test]
    fn test_get_last() {
        let simple = CharacterName::simple("Test");
        let standard = CharacterName::standard("Aaa", "Bbb");
        let married = CharacterName::married("Aaa", "Ccc", "Bbb");

        assert_eq!(simple.get_last(), None);
        assert_eq!(standard.get_last(), Some("Bbb"));
        assert_eq!(married.get_last(), Some("Ccc"));
    }

    #[test]
    fn test_marry() {
        let simple = CharacterName::simple("Test");
        let standard = CharacterName::standard("Aaa", "Bbb");
        let married = CharacterName::married("Aaa", "Ccc", "Bbb");
        let married2 = CharacterName::married("Aaa", "Ddd", "Bbb");

        assert_eq!(simple.marry("Ccc"), simple);
        assert_eq!(standard.marry("Ccc"), married);
        assert_eq!(married.marry("Ddd"), married2);
    }
}
