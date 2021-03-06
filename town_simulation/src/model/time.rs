use derive_more::Constructor;

#[derive(Constructor, Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Date(u32);

impl Date {
    pub fn get_years_since(&self, date: Date) -> u32 {
        self.0 - date.0
    }

    pub fn get_year(&self) -> u32 {
        self.0
    }

    pub fn increase_year(&mut self) {
        self.0 += 1;
    }

    pub fn increase_by(&mut self, years: u32) {
        self.0 += years;
    }
}
