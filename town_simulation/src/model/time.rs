use derive_more::Constructor;

#[derive(Constructor, Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Date(u32);
