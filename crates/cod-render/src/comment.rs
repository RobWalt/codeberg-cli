use crate::prelude::*;
use chrono::{DateTime, Utc};

pub fn render_comment(
    username: &str,
    creation_time: DateTime<Utc>,
    comment: &str,
    max_width: usize,
) -> String {
    const PARENTHESES: usize = 2;
    const COLON: usize = 1;

    const BORDER_CHARS: usize = 2;
    const BORDER_PADDING: usize = 2;
    const EXTRA_PADDING_IF_UNEVEN: usize = 1;

    let creation_time_formatted = creation_time.format("%d.%m.%Y - %H:%M").to_string();

    let comment_text_width =
        max_width - (2 * BORDER_CHARS + 2 * BORDER_PADDING + EXTRA_PADDING_IF_UNEVEN);
    println!("{comment_text_width}");

    format!(
        "{}\n({}):\n{}\n\n{}",
        username,
        creation_time_formatted,
        "=".repeat(creation_time_formatted.len() + PARENTHESES + COLON),
        CodTableBuilder::new()
            .with_max_column_width(comment_text_width)
            .add_row(Row::new(vec![TableCell::new(comment)]))
            .build()
            .render()
    )
}
