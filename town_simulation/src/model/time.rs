use derive_more::Constructor;

#[derive(Constructor, Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Date(u32);

impl Date {
    pub fn get_year(&self) -> u32 {
        self.0
    }
}
