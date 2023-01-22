use term_table::row::Row;
use term_table::{TableBuilder, TableStyle};

use crate::table::CodTable;

#[derive(Debug, Clone, Default)]
pub struct CodTableBuilder<'a> {
    pub(crate) max_column_width: Option<usize>,
    pub(crate) style: Option<TableStyle>,
    pub(crate) rows: Vec<Row<'a>>,
}

impl<'a> CodTableBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_column_width(mut self, witdh: usize) -> Self {
        self.max_column_width.replace(witdh);
        self
    }

    pub fn with_style(mut self, style: TableStyle) -> Self {
        self.style.replace(style);
        self
    }

    pub fn add_row(mut self, row: Row<'a>) -> Self {
        self.rows.push(row);
        self
    }

    pub fn add_rows<RI>(mut self, rows: RI) -> Self
    where
        RI: IntoIterator<Item = Row<'a>>,
    {
        self.rows.extend(rows);
        self
    }

    pub fn build(self) -> CodTable<'a> {
        let CodTableBuilder {
            max_column_width,
            style,
            rows,
        } = self;

        CodTable {
            table: TableBuilder::new()
                .max_column_width(max_column_width.unwrap_or(40))
                .style(style.unwrap_or(TableStyle::elegant()))
                .rows(rows)
                .build(),
        }
    }
}
