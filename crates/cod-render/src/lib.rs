pub mod spinner;
pub mod table;
pub mod ui;

pub mod prelude {
    pub use super::table::builder::CodTableBuilder;
    pub use term_table::row::Row;
    pub use term_table::table_cell::Alignment;
    pub use term_table::table_cell::TableCell;
}
