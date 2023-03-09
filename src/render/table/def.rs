use std::ops::{Deref, DerefMut};

use term_table::Table;

use crate::render::table::builder::BergTableBuilder;

#[derive(Debug, Clone)]
pub struct BergTable<'a> {
    pub(crate) table: Table<'a>,
}

impl<'a> Deref for BergTable<'a> {
    type Target = Table<'a>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<'a> DerefMut for BergTable<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl<'a> BergTable<'a> {
    pub fn builder() -> BergTableBuilder<'a> {
        BergTableBuilder::new()
    }

    pub fn render(self) -> String {
        self.table.render()
    }
}
