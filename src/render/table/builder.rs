use term_table::row::Row;
use term_table::{TableBuilder, TableStyle};

use crate::render::table::def::BergTable;
use crate::render::table::text::wrap_text_for_table;
use crate::render::table::MAXIMUM_TABLE_WIDTH;

#[derive(Debug, Clone, Default)]
pub struct BergTableBuilder<'a> {
    pub(crate) max_column_width: Option<usize>,
    pub(crate) style: Option<TableStyle>,
    pub(crate) rows: Vec<Row<'a>>,
}

impl<'a> BergTableBuilder<'a> {
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

    pub fn add_row(self, row: Row<'a>) -> Self {
        self.add_rows([row])
    }

    fn table_wrap_width(&self) -> usize {
        self.max_column_width
            .unwrap_or(MAXIMUM_TABLE_WIDTH)
            // subtract 4, because
            // - 2 for border characters
            // - 2 for inner margin between text and border
            .saturating_sub(4)
    }

    pub fn add_rows<RI>(mut self, rows: RI) -> Self
    where
        RI: IntoIterator<Item = Row<'a>>,
    {
        let wrap_width = self.table_wrap_width();
        let wrapped_rows = rows.into_iter().map(|mut row| {
            row.cells.iter_mut().for_each(|mut cell| {
                cell.data = wrap_text_for_table(cell.data.as_ref(), wrap_width).into();
            });
            row
        });
        self.rows.extend(wrapped_rows);
        self
    }

    pub fn build(self) -> BergTable<'a> {
        let BergTableBuilder {
            max_column_width,
            style,
            rows,
        } = self;

        BergTable {
            table: TableBuilder::new()
                .max_column_width(max_column_width.unwrap_or(MAXIMUM_TABLE_WIDTH))
                .style(style.unwrap_or_else(TableStyle::elegant))
                .rows(rows)
                .build(),
        }
    }
}
