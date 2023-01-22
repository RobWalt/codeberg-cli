mod builder;
mod table;

pub mod prelude {
    pub use super::builder::CodTableBuilder;
    pub use term_table::row::Row;
    pub use term_table::table_cell::Alignment;
    pub use term_table::table_cell::TableCell;
}
