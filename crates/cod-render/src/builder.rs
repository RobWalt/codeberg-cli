use term_table::{Table, TableStyle};

use crate::table::CodTable;

#[derive(Debug, Clone)]
pub struct CodTableBuilder {
    pub(crate) max_column_width: Option<usize>,
    pub(crate) style: Option<TableStyle>,
}

impl Default for CodTableBuilder {
    fn default() -> Self {
        Self {
            max_column_width: Some(40),
            style: Some(TableStyle::elegant()),
        }
    }
}

impl CodTableBuilder {
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

    pub fn build<'a>(self) -> CodTable<'a> {
        let CodTableBuilder {
            max_column_width,
            style,
        } = self;

        let mut table = Table::new();

        if let Some(witdh) = max_column_width {
            table.max_column_width(witdh);
        }

        if let Some(style) = style {
            table.style = style;
        }

        CodTable { table }
    }
}
