use std::ops::{Deref, DerefMut};

use term_table::Table;

use crate::builder::CodTableBuilder;

#[derive(Debug, Clone)]
pub struct CodTable<'a> {
    pub(crate) table: Table<'a>,
}

impl<'a> Deref for CodTable<'a> {
    type Target = Table<'a>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<'a> DerefMut for CodTable<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl<'a> CodTable<'a> {
    pub fn builder() -> CodTableBuilder<'a> {
        CodTableBuilder::new()
    }

    pub fn render(self) -> String {
        self.table.render()
    }
}
