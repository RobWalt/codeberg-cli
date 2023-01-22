use std::ops::{Deref, DerefMut};

use term_table::row::Row;
use term_table::Table;

use crate::builder::CodTableBuilder;

#[derive(Debug, Clone)]
pub struct CodTable<'data> {
    pub(crate) table: Table<'data>,
}

impl<'data> Deref for CodTable<'data> {
    type Target = Table<'data>;
    fn deref(&self) -> &Self::Target {
        &self.table
    }
}

impl<'data> DerefMut for CodTable<'data> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}

impl<'data> CodTable<'data> {
    pub fn builder() -> CodTableBuilder {
        CodTableBuilder::new()
    }

    pub fn add_row(mut self, row: Row<'data>) -> Self {
        self.table.add_row(row);
        self
    }

    pub fn add_rows<RI>(mut self, rows: RI) -> Self
    where
        RI: IntoIterator<Item = Row<'data>>,
    {
        rows.into_iter().for_each(|row| self.table.add_row(row));
        self
    }

    pub fn render(self) -> String {
        self.table.render()
    }
}
